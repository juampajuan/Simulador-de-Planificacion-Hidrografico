#![allow(dead_code)]

use rand::RngExt;
use crate::{processing::measuring::calculate_distance_between_points, structs::depth_matrix::DepthMatrix};

/// Aplica ruido GNSS a un path ya construido, detectando los segmentos automáticamente.
pub fn apply_gnss_noise(
    path: &[(usize, usize)],
    matrix: &DepthMatrix,
    max_offset_meters: f64,
) -> Vec<(usize, usize)> {
    let segments = detect_segments(path);
    apply_gnss_noise_segmented(path, &segments, matrix, max_offset_meters)
}

/// Aplica ruido GNSS usando segmentos precalculados.
pub fn apply_gnss_noise_segmented(
    path: &[(usize, usize)],
    segments: &[std::ops::Range<usize>],
    matrix: &DepthMatrix,
    max_offset_meters: f64,
) -> Vec<(usize, usize)> {
    let mut rng = rand::rng();
    let mut result: Vec<(usize, usize)> = path.to_vec();

    for seg in segments {
        let seg_start = seg.start;
        let seg_end = seg.end;
        let seg_len = seg_end - seg_start;

        if seg_len < 2 { continue; }

        let start_point = &path[seg_start];
        let end_point   = &path[seg_end - 1];

        let len_meters = calculate_distance_between_points(start_point, end_point, matrix);
        if len_meters == 0.0 { continue; }

        // El desvío máximo real se acota según la longitud del segmento.
        // - `max_fraction`: el desvío nunca supera el 15% de la longitud del tramo,
        //   evitando quiebres absurdos en conexiones cortas.
        let max_fraction = 0.15;
        let effective_max = max_offset_meters
            .min(len_meters * max_fraction)
            .min(max_offset_meters * (len_meters / (len_meters + max_offset_meters * 5.0)).sqrt());

        let dx_m = (end_point.0 as f64 - start_point.0 as f64) * matrix.size_x;
        let dy_m = (end_point.1 as f64 - start_point.1 as f64) * matrix.size_y;

        let (perp_x_m, perp_y_m) = (-dy_m / len_meters, dx_m / len_meters);
        let perp_x_px = perp_x_m / matrix.size_x;
        let perp_y_px = perp_y_m / matrix.size_y;

        let harmonics: Vec<(f64, f64)> = (1..=4)
            .map(|k| {
                let amplitude = effective_max / k as f64;
                let phase: f64 = rng.random::<f64>() * std::f64::consts::TAU;
                (amplitude, phase)
            })
            .collect();

        let max_possible: f64 = harmonics.iter().map(|(a, _)| a).sum();

        for j in 0..seg_len {
            let idx = seg_start + j;
            let t = j as f64 / (seg_len - 1) as f64 * std::f64::consts::PI;

            let raw: f64 = harmonics.iter()
                .enumerate()
                .map(|(ki, (amp, phase))| {
                    let k = ki + 1;
                    amp * (k as f64 * t + phase).sin()
                })
                .sum();

            let offset_m = (raw / max_possible) * effective_max * t.sin();

            let nx = path[idx].0 as f64 + perp_x_px * offset_m;
            let ny = path[idx].1 as f64 + perp_y_px * offset_m;

            if valid(matrix, nx, ny) {
                result[idx] = (nx.round() as usize, ny.round() as usize);
            }
        }
    }

    result
}

/// Detecta segmentos rectos usando la heurística de ángulo (dot < 0.7 → giro).
fn detect_segments(path: &[(usize, usize)]) -> Vec<std::ops::Range<usize>> {
    let n = path.len();

    let is_turn: Vec<bool> = (0..n)
        .map(|i| {
            if i == 0 || i + 1 >= n { return true; }
            let (px, py) = (path[i - 1].0 as f64, path[i - 1].1 as f64);
            let (cx, cy) = (path[i].0 as f64, path[i].1 as f64);
            let (nx, ny) = (path[i + 1].0 as f64, path[i + 1].1 as f64);
            let dx1 = cx - px; let dy1 = cy - py;
            let dx2 = nx - cx; let dy2 = ny - cy;
            let len1 = (dx1 * dx1 + dy1 * dy1).sqrt();
            let len2 = (dx2 * dx2 + dy2 * dy2).sqrt();
            if len1 == 0.0 || len2 == 0.0 { return true; }
            let dot = (dx1 * dx2 + dy1 * dy2) / (len1 * len2);
            dot < 0.7
        })
        .collect();

    let mut segments = Vec::new();
    let mut start = 0;

    for (i, _) in is_turn.iter().enumerate().take(n).skip(1) {
        if is_turn[i] {
            if i > start + 1 {
                segments.push(start..i);
            }
            start = i;
        }
    }

    segments
}

fn valid(matrix: &DepthMatrix, x: f64, y: f64) -> bool {
    let xi = x.round() as isize;
    let yi = y.round() as isize;
    xi >= 0 && yi >= 0
        && xi < matrix.width as isize
        && yi < matrix.height as isize
        && Some(matrix.data[yi as usize][xi as usize]) != matrix.no_data
}