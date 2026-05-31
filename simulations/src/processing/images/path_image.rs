use image::{Rgb, RgbImage};
use crate::structs::depth_matrix::DepthMatrix;
use super::colormap::depth_color;

pub fn makepng_with_matrix_and_path(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
) -> RgbImage {
    let (min_depth, max_depth) = depth_range(matrix);
    let range = if (max_depth - min_depth).abs() < 1e-10 { 1.0 } else { max_depth - min_depth };

    let mut img = RgbImage::new(matrix.width as u32, matrix.height as u32);

    for (y, row) in matrix.data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let color = if !is_valid(val, matrix) {
                Rgb([30u8, 30u8, 30u8])
            } else {
                depth_color((val - min_depth) / range)
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    for &(x, y) in path {
    if y < matrix.height && x < matrix.width {
        // Calculamos los límites usando usize de forma segura
        let y_min = y.saturating_sub(1);
        let y_max = (y + 1).min(matrix.height - 1);
        
        let x_min = x.saturating_sub(1);
        let x_max = (x + 1).min(matrix.width - 1);

        // Iteramos directamente sobre las coordenadas válidas de la matriz
        for ny in y_min..=y_max {
            for nx in x_min..=x_max {
                img.put_pixel(nx as u32, ny as u32, Rgb([255, 255, 255]));
            }
        }
    }
}

    img
}

fn depth_range(matrix: &DepthMatrix) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for row in &matrix.data {
        for &val in row {
            if is_valid(val, matrix) {
                min = min.min(val);
                max = max.max(val);
            }
        }
    }
    (min, max)
}

fn is_valid(val: f64, matrix: &DepthMatrix) -> bool {
    match matrix.no_data {
        Some(nd) => val != nd,
        None => val.is_finite(),
    }
}