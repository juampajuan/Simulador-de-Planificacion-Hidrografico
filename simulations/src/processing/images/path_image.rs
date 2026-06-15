use image::{Rgba, RgbaImage};
use crate::structs::depth_matrix::DepthMatrix;
use super::helpers::{depth_color,depth_range,is_valid};

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

