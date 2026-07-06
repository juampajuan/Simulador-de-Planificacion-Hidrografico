/*Archivo para funciones que se comparten unicamente en el modulo de images */
use image::{Rgb,Rgba};

use crate::structs::depth_matrix::DepthMatrix;
use crate::processing::processing_helpers::is_valid;

pub const ZONE_FILL_COLOR: Rgba<u8> = Rgba([71, 85, 105, 45]);
pub const COVERAGE_OVERLAY_COLOR: Rgba<u8> = Rgba([14, 116, 144, 180]);

// Esta escala esta basada en la imagen de demostración que mandó Fernando.
// t=0.0 → rojo        (menos profundo)
// t=0.5 → crema claro (profundidad promedio)
// t=1.0 → azul        (más profundo)
const STOPS: [(f64, f64, f64, f64); 5] = [
    (0.00, 180.0,  15.0,   0.0),  // rojo oscuro
    (0.25, 240.0, 100.0,  20.0),  // naranja
    (0.50, 255.0, 225.0, 155.0),  // crema
    (0.75,  30.0, 175.0, 135.0),  // teal
    (1.00,   0.0,  70.0, 190.0),  // azul
];

// Paleta secuencial para magnitud de diferencia:
// verde (sin diferencia) -> amarillo -> rojo (diferencia máxima)
const DIFF_STOPS: [(f64, f64, f64, f64); 3] = [
    (0.0, 26.0,  152.0, 80.0),   // verde   (t=0.0 -> diferencia mínima)
    (0.5, 255.0, 255.0, 191.0),  // amarillo pálido (t=0.5 -> diferencia media)
    (1.0, 215.0, 25.0,  28.0),   // rojo    (t=1.0 -> diferencia máxima)
];

pub enum ImageType {
    DepthImage,
    DifferenceImage,
}

impl ImageType {
    pub fn color_fn(&self) -> impl Fn(f64) -> Rgb<u8> {
        match self {
            ImageType::DepthImage    => depth_color as fn(f64) -> Rgb<u8>,
            ImageType::DifferenceImage=> diff_color as fn(f64) -> Rgb<u8>,
        }
    }
}

pub fn depth_color(t: f64) -> Rgb<u8> {
    let t = t.clamp(0.0, 1.0);

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

pub fn diff_color(t: f64) -> Rgb<u8> {
    let t = t.clamp(0.0, 1.0);

    let mut seg_start = DIFF_STOPS[0];
    let mut seg_end   = DIFF_STOPS[1];
    for i in 0..DIFF_STOPS.len() - 1 {
        if t <= DIFF_STOPS[i + 1].0 {
            seg_start = DIFF_STOPS[i];
            seg_end   = DIFF_STOPS[i + 1];
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

pub fn depth_range(matrix: &Vec<Vec<f64>>, no_data: Option<f64>) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for row in matrix {
        for &val in row {
            if is_valid(val, no_data) {
                min = min.min(val);
                max = max.max(val);
            }
        }
    }
    (min, max)
}

pub fn fill_zone_translucent(
    img: &mut image::RgbaImage,
    matrix: &DepthMatrix,
    color: Rgba<u8>,
) {
    for (y, row) in matrix.data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if is_valid(val, matrix.no_data) {
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn draw_covered_points(
    img: &mut image::RgbaImage,
    covered_points: &[((usize, usize), f64)],
    color: Rgba<u8>,
) {
    let (width, height) = img.dimensions();
    for &((x, y), _) in covered_points {
        if (x as u32) < width && (y as u32) < height {
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}

pub fn draw_path(
    img: &mut image::RgbaImage,
    matrix: &DepthMatrix,
    path: &[(usize, usize)],
    color: Rgba<u8>,
) {
    let hw = matrix.width.max(matrix.height) / 1500;
    for &(x, y) in path {
        if y < matrix.height && x < matrix.width {
            let y_min = y.saturating_sub(hw);
            let y_max = (y + hw).min(matrix.height - 1);
            let x_min = x.saturating_sub(hw);
            let x_max = (x + hw).min(matrix.width - 1);
            for ny in y_min..=y_max {
                for nx in x_min..=x_max {
                    img.put_pixel(nx as u32, ny as u32, color);
                }
            }
        }
    }
}