use image::{ImageBuffer, Rgb, RgbImage};
use crate::structs::depth_matrix::DepthMatrix;

fn depth_color(t: f64) -> Rgb<u8> {
    // Esta escala esta basada en la imagen de demostracion que mando Fernando hace bastante.
    // t=0.0 → rojo        (menos profundo / valor f64 menor)
    // t=0.5 → crema claro (profundidad promedio)
    // t=1.0 → azul        (más profundo / valor f64 mayor)
    const STOPS: [(f64, f64, f64, f64); 5] = [
        (0.00, 180.0,  15.0,   0.0),  // rojo oscuro
        (0.25, 240.0, 100.0,  20.0),  // naranja
        (0.50, 255.0, 225.0, 155.0),  // naranja muy claro / crema  ← clave
        (0.75,  30.0, 175.0, 135.0),  // teal / azul verdoso
        (1.00,   0.0,  70.0, 190.0),  // azul
    ];

    let t = t.clamp(0.0, 1.0);

    // Buscar el segmento correspondiente
    let mut seg_start = STOPS[0];
    let mut seg_end   = STOPS[1];
    for i in 0..STOPS.len() - 1 {
        if t <= STOPS[i + 1].0 {
            seg_start = STOPS[i];
            seg_end   = STOPS[i + 1];
            break;
        }
    }

    let (t0, r0, g0, b0) = seg_start;
    let (t1, r1, g1, b1) = seg_end;

    let factor = if (t1 - t0).abs() < 1e-10 {
        0.0
    } else {
        (t - t0) / (t1 - t0)
    };

    Rgb([
        (r0 + factor * (r1 - r0)).clamp(0.0, 255.0) as u8,
        (g0 + factor * (g1 - g0)).clamp(0.0, 255.0) as u8,
        (b0 + factor * (b1 - b0)).clamp(0.0, 255.0) as u8,
    ])
}

pub fn makePNG_with_matrix_and_path(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
) -> RgbImage {

    let mut min_depth = f64::MAX;
    let mut max_depth = f64::MIN;

    for row in &matrix.data {
        for &val in row {
            let is_valid = match matrix.no_data {
                Some(nd) => val != nd,
                None => val.is_finite(),
            };
            if is_valid {
                min_depth = min_depth.min(val);
                max_depth = max_depth.max(val);
            }
        }
    }

    let range = if (max_depth - min_depth).abs() < 1e-10 { 1.0 } else { max_depth - min_depth };

    let mut img = RgbImage::new(matrix.width as u32, matrix.height as u32);

    for (y, row) in matrix.data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let is_valid = match matrix.no_data {
                Some(nd) => val != nd,
                None => val.is_finite(),
            };

            let color = if !is_valid {
                Rgb([30u8, 30u8, 30u8]) // no_data → gris oscuro
            } else {
                let t = (val - min_depth) / range;
                depth_color(t)
            };

            img.put_pixel(x as u32, y as u32, color);
        }
    }

    for &(x, y) in path {
        if y < matrix.height && x < matrix.width {
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    let ny = y as i32 + dy;
                    let nx = x as i32 + dx;
                    if ny >= 0
                        && nx >= 0
                        && (ny as usize) < matrix.height
                        && (nx as usize) < matrix.width
                    {
                        img.put_pixel(nx as u32, ny as u32, Rgb([255, 255, 255]));
                    }
                }
            }
        }
    }

    img
}

pub fn makePng_with_matrix_and_interpolation(
    matrix: &Vec<Vec<f64>>,
    geotiff: &DepthMatrix,
) -> RgbImage {
    let fondo_especial = geotiff.no_data.unwrap_or(170141000000000000000000000000000000000.0_f64);

    let height = matrix.len() as u32;
    let width = matrix[0].len() as u32;

    let mut min_val = f64::INFINITY;
    let mut max_val = f64::NEG_INFINITY;

    for row in matrix {
        for &val in row {
            if val != fondo_especial {
                if val < min_val { min_val = val; }
                if val > max_val { max_val = val; }
            }
        }
    }

    if min_val == f64::INFINITY {
        min_val = 0.0;
        max_val = 10.0;
    }
    let range = if (max_val - min_val).abs() < 1e-10 { 1.0 } else { max_val - min_val };

    let mut img = RgbImage::new(width, height);

    for (y, row) in matrix.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let color = if val == fondo_especial {
                Rgb([0u8, 0u8, 0u8]) // no_data → negro
            } else {
                let t = ((val - min_val) / range).clamp(0.0, 1.0);
                depth_color(t)
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    img.save("escala_personalizada.png").expect("Error al guardar imagen");
    println!("¡Imagen generada con éxito!");
    println!("Escala: Mín={:.2} (rojo/poco profundo) → Máx={:.2} (azul/profundo)", min_val, max_val);

    img
}