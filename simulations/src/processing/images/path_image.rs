use image::{Rgba, RgbaImage};
use crate::structs::depth_matrix::DepthMatrix;
use super::helpers::{depth_color, depth_range, is_valid};

#[allow(dead_code)]
pub fn makepng_with_matrix_and_path(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
) -> RgbaImage {
    let (min_depth, max_depth) = depth_range(matrix);
    let range = if (max_depth - min_depth).abs() < 1e-10 { 1.0 } else { max_depth - min_depth };

    let mut img = RgbaImage::new(matrix.width as u32, matrix.height as u32);

    for (y, row) in matrix.data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let color = if !is_valid(val, matrix) {
                Rgba([0u8, 0u8, 0u8, 0u8])
            } else {
                let c = depth_color((val - min_depth) / range);
                Rgba([c[0], c[1], c[2], 255u8])
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    for &(x, y) in path {
        if y < matrix.height && x < matrix.width {
            let y_min = y.saturating_sub(1);
            let y_max = (y + 1).min(matrix.height - 1);
            let x_min = x.saturating_sub(1);
            let x_max = (x + 1).min(matrix.width - 1);
            for ny in y_min..=y_max {
                for nx in x_min..=x_max {
                    img.put_pixel(nx as u32, ny as u32, Rgba([255, 255, 255, 255]));
                }
            }
        }
    }

    img
}

pub fn makepng_transparent_with_path(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
) -> RgbaImage {
    let mut img = RgbaImage::new(matrix.width as u32, matrix.height as u32);

    // grosor proporcional a la resolucion: fino en rasters chicos, visible en grandes
    let hw = matrix.width.max(matrix.height) / 1500;
    
    for &(x, y) in path {
        if y < matrix.height && x < matrix.width {
            let y_min = y.saturating_sub(hw);
            let y_max = (y + hw).min(matrix.height - 1);
            let x_min = x.saturating_sub(hw);
            let x_max = (x + hw).min(matrix.width - 1);
            for ny in y_min..=y_max {
                for nx in x_min..=x_max {
                    img.put_pixel(nx as u32, ny as u32, Rgba([255, 255, 255, 255]));
                }
            }
        }
    }

    img
}

// Recibe los puntos ya calculados por get_measures.
// Fondo transparente, puntos cubiertos en azul oscuro, recorrido blanco semitransparente.
pub fn make_shaded_png(
    matrix: &DepthMatrix,
    covered_points: &[((usize, usize), f64)],
    path: &Vec<(usize, usize)>,
) -> RgbaImage {
    let mut img = RgbaImage::new(matrix.width as u32, matrix.height as u32);

    // Fondo transparente — sin pintar el GeoTIFF a color
    for (y, row) in matrix.data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let color = if is_valid(val, matrix) {
                Rgba([0u8, 0u8, 0u8, 0u8])
            } else {
                Rgba([0u8, 0u8, 0u8, 0u8])
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    // Puntos cubiertos en azul oscuro (estilo de la página)
    for &((x, y), _) in covered_points {
        if y < matrix.height && x < matrix.width {
            img.put_pixel(x as u32, y as u32, Rgba([14u8, 116u8, 144u8, 180u8]));
        }
    }

    // Recorrido blanco semitransparente encima
    let hw = matrix.width.max(matrix.height) / 1500;

    for &(x, y) in path {
        if y < matrix.height && x < matrix.width {
            let y_min = y.saturating_sub(hw);
            let y_max = (y + hw).min(matrix.height - 1);
            let x_min = x.saturating_sub(hw);
            let x_max = (x + hw).min(matrix.width - 1);
            for ny in y_min..=y_max {
                for nx in x_min..=x_max {
                    img.put_pixel(nx as u32, ny as u32, Rgba([255, 255, 255, 180u8]));
                }
            }
        }
    }

    img
}