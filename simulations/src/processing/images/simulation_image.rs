use image::{Rgba, RgbaImage};
use crate::{processing::images::ImageType::{DepthImage, DifferenceImage}, structs::depth_matrix::DepthMatrix};
use super::images_helpers::{depth_color, depth_range, ImageType};

/// Genera el png del resultado de la simulacion segun los puntos medidos
/// Utiliza los colores especificados en helpers.rs para las alturas
pub fn makepng_with_matrix_and_interpolation(
    matrix: &[Vec<f64>],
    geotiff: &DepthMatrix,
    image_type: ImageType,
) -> (RgbaImage, f64, f64) {
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);

    let height = matrix.len() as u32;
    let width  = matrix[0].len() as u32;

    let color_fn = image_type.color_fn();
    
    let (min_val, max_val) = match image_type{
        DepthImage => {depth_range(&geotiff.data, geotiff.no_data)}
        DifferenceImage => {depth_range(matrix, geotiff.no_data)}
    };

    let range = if (max_val - min_val).abs() < 1e-10 { 1.0 } else { max_val - min_val };
 
    let mut img = RgbaImage::new(width, height);
 
    for (y, row) in matrix.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let color = if val == no_data {
                Rgba([0u8, 0u8, 0u8, 0u8])
            } else {
                let t = ((val - min_val) / range).clamp(0.0, 1.0);
                let c = color_fn(t);
                Rgba([c[0], c[1], c[2], 255u8])
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }
 
    (img, min_val, max_val)
}

pub fn create_scale_image() -> RgbaImage {
    let width = 20u32;
    let height = 300u32;
    
    let mut img = RgbaImage::new(width, height);
    
    for y in 0..height {
        let t = (y as f64) / ((height - 1) as f64);
        
        let c = depth_color(t);
        let color = Rgba([c[0], c[1], c[2], 255u8]);
        
        for x in 0..width {
            img.put_pixel(x, y, color);
        }
    }
    
    img
}