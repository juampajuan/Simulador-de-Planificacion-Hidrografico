use image::{Rgb, RgbImage};
use crate::structs::depth_matrix::DepthMatrix;
use super::colormap::depth_color;

pub fn makePng_with_matrix_and_interpolation(
    matrix: &Vec<Vec<f64>>,
    geotiff: &DepthMatrix,
) -> RgbImage {
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);

    let height = matrix.len() as u32;
    let width  = matrix[0].len() as u32;

    let (min_val, max_val) = depth_range(matrix, no_data);
    let range = if (max_val - min_val).abs() < 1e-10 { 1.0 } else { max_val - min_val };

    let mut img = RgbImage::new(width, height);

    for (y, row) in matrix.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let color = if val == no_data {
                Rgb([0u8, 0u8, 0u8])
            } else {
                let t = ((val - min_val) / range).clamp(0.0, 1.0);
                depth_color(t)
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    img
}

fn depth_range(matrix: &Vec<Vec<f64>>, no_data: f64) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for row in matrix {
        for &val in row {
            if val != no_data {
                if val < min { min = val; }
                if val > max { max = val; }
            }
        }
    }
    if min == f64::INFINITY {
        (0.0, 10.0)
    } else {
        (min, max)
    }
}