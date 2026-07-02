use image::{Rgb,Rgba};

use crate::structs::depth_matrix::DepthMatrix;

pub const ZONE_FILL_COLOR: Rgba<u8> = Rgba([71, 85, 105, 45]);

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

pub fn depth_range(matrix: &DepthMatrix) -> (f64, f64) {
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

pub fn is_valid(val: f64, matrix: &DepthMatrix) -> bool {
    match matrix.no_data {
        Some(nd) => val != nd,
        None => val.is_finite(),
    }
}

pub fn fill_zone_translucent(
    img: &mut image::RgbaImage,
    matrix: &DepthMatrix,
    color: Rgba<u8>,
) {
    for (y, row) in matrix.data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if is_valid(val, matrix) {
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}