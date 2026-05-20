use image::{ImageBuffer, Rgb, RgbImage};
use crate::{structs::depth_matrix::DepthMatrix};

pub fn makePNG_with_matrix_and_path(matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
)-> RgbImage {
    // 1. Min/max ignorando no_data
    let mut min_depth = f64::MAX;
    let mut max_depth = f64::MIN;

    for row in &matrix.data {
        for &val in row {
            let is_valid = match matrix.no_data {
                Some(nd) => val != nd,
                None     => val.is_finite(),
            };
            if is_valid {
                min_depth = min_depth.min(val);
                max_depth = max_depth.max(val);
            }
        }
    }

     // 2. Crear imagen
    let mut img = RgbImage::new(matrix.width as u32, matrix.height as u32);

    // 3. Pintar fondo con gradiente azul
    for (y, row) in matrix.data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let is_valid = match matrix.no_data {
                Some(nd) => val != nd,
                None     => val.is_finite(),
            };

            let color = if !is_valid {
                Rgb([30u8, 30u8, 30u8])  // el noData es gris oscuro
            } else {
                let t = (val - min_depth) / (max_depth - min_depth);
                Rgb([
                    (20.0 + t * 10.0) as u8,
                    (80.0 + t * 40.0) as u8,
                    (150.0 + t * 80.0) as u8,
                ])
            };

            img.put_pixel(x as u32, y as u32, color);
        }
    }

    // 4. Pintar recorrido en rojo (con grosor de 1px alrededor)
    for &(row, col) in path {
        if row < matrix.height && col < matrix.width {
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    let ny = row as i32 + dy;
                    let nx = col as i32 + dx;
                    if ny >= 0 && nx >= 0
                        && (ny as usize) < matrix.height
                        && (nx as usize) < matrix.width
                    {
                        img.put_pixel(nx as u32, ny as u32, Rgb([255u8, 50u8, 50u8]));
                    }
                }
            }
        }
    }

    img
}

pub fn process_depth(matrix: &Vec<Vec<f64>>) {

    let fondo_especial = 170141000000000000000000000000000000000.0_f64;

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
    let range = if max_val == min_val { 1.0 } else { max_val - min_val };

    let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);

    for row in matrix {
        for &val in row {
            if val == fondo_especial {
                pixels.push(0); // R
                pixels.push(0); // G
                pixels.push(0); // B
            } else {
                let t = ((val - min_val) / range).clamp(0.0, 1.0);

                let (r, g, b) = if t < 0.5 {
                    let factor = t * 2.0; 
                    (
                        (factor * 255.0) as u8,
                        0,
                        ((1.0 - factor) * 255.0) as u8,
                    )
                } else {
                    let factor = (t - 0.5) * 2.0;
                    (
                        255,
                        (factor * 255.0) as u8,
                        0,
                    )
                };

                pixels.push(r);
                pixels.push(g);
                pixels.push(b);
            }
        }
    }

    if let Some(img_buffer) = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width, height, pixels) {
        img_buffer.save("escala_personalizada.png").expect("Error al guardar imagen");
        println!("¡Imagen generada con éxito!");
        println!("Escala calculada para datos reales: Mín={:.2}, Máx={:.2}", min_val, max_val);
    }
}