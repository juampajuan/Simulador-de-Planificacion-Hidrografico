use image::{ImageBuffer, Luma, Rgb};
use crate::{structs::depth_matrix::DepthMatrix};

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