use rand::RngExt;

use crate::{processing::measuring::calculate_distance_between_points, structs::depth_matrix::DepthMatrix};

pub fn generate_route(
    matrix: &DepthMatrix,
    azimuth_deg: f64,
    separation_meters: f64,
    max_offset_meters: f64,
) -> Vec<(usize, usize)> {

    let mut path: Vec<(usize, usize)> = Vec::new();
    // Cada segmento es un rango de índices sobre `path` que corresponde
    // a un tramo recto (pierna o conexión). Los calculamos acá para no
    // repetir el trabajo en apply_gnss_noise.
    let mut segments: Vec<std::ops::Range<usize>> = Vec::new();

    let w = matrix.width as f64;
    let h = matrix.height as f64;
    let center_x = w / 2.0;
    let center_y = h / 2.0;
    let diagonal = (w.powi(2) + h.powi(2)).sqrt();

    let angle = azimuth_deg.to_radians();
    let (sin_a, cos_a) = angle.sin_cos();

    let size_x = matrix.size_x;
    let size_y = matrix.size_y;

    let dir_x_px = sin_a / size_x;
    let dir_y_px = -cos_a / size_y;
    let mag_dir = (dir_x_px.powi(2) + dir_y_px.powi(2)).sqrt();
    let dir_x = dir_x_px / mag_dir;
    let dir_y = dir_y_px / mag_dir;

    let perpendicular_x_px = cos_a / size_x;
    let perpendicular_y_px = sin_a / size_y;
    let mag_perpendicular = (perpendicular_x_px.powi(2) + perpendicular_y_px.powi(2)).sqrt();
    let perpendicular_x = perpendicular_x_px / mag_perpendicular;
    let perpendicular_y = perpendicular_y_px / mag_perpendicular;

    let separation_px = separation_meters * mag_perpendicular;
    let legs = (diagonal / separation_px).ceil() as i32;

    let mut previous_end: Option<(f64, f64)> = None;

    for leg in -legs / 2..=legs / 2 {

        let mut line = build_leg(
            matrix, center_x, center_y,
            perpendicular_x, perpendicular_y,
            dir_x, dir_y,
            diagonal, separation_px, leg,
        );

        if line.is_empty() {
            continue;
        }

        if leg % 2 != 0 {
            line.reverse();
        }

        // Conexión: la registramos como segmento propio
        if let Some(prev) = previous_end {
            let seg_start = path.len();
            connect(matrix, prev, line[0], &mut path);
            let seg_end = path.len();
            if seg_end > seg_start {
                segments.push(seg_start..seg_end);
            }
        }

        // Pierna: también la registramos como segmento
        let seg_start = path.len();
        path.extend(line.iter().map(|(x, y)| (x.round() as usize, y.round() as usize)));
        let seg_end = path.len();
        if seg_end > seg_start {
            segments.push(seg_start..seg_end);
        }

        update_previous_end(&line, &mut previous_end);
    }

    // let noisy = apply_gnss_noise_segmented(&path, &segments, matrix, max_offset_meters);

    // noisy
    path
}

// Versión pública que acepta un path ya construido y recalcula los segmentos
// con la misma heurística de ángulo que usaba la versión original.
// Útil si necesitás aplicar ruido a un path externo.
pub fn apply_gnss_noise(
    path: &[(usize, usize)],
    matrix: &DepthMatrix,
    max_offset_meters: f64,
) -> Vec<(usize, usize)> {

    let segments = detect_segments(path);
    apply_gnss_noise_segmented(path, &segments, matrix, max_offset_meters)
}

// Detecta segmentos rectos usando la heurística de ángulo (dot < 0.7 → giro).
// Solo se usa cuando el caller no tiene los segmentos precalculados.
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

    for i in 1..n {
        if is_turn[i] {
            if i > start + 1 {
                segments.push(start..i);
            }
            start = i;
        }
    }

    segments
}

fn apply_gnss_noise_segmented(
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

        // Distancia real en metros entre los extremos del segmento
        let len_meters = calculate_distance_between_points(start_point, end_point, matrix);
        if len_meters == 0.0 { continue; }

        // dx/dy en metros → dirección perpendicular al segmento, en metros
        let dx_m = (end_point.0 as f64 - start_point.0 as f64) * matrix.size_x;
        let dy_m = (end_point.1 as f64 - start_point.1 as f64) * matrix.size_y;

        // Perpendicular normalizada (en el espacio métrico)
        let (perp_x_m, perp_y_m) = (-dy_m / len_meters, dx_m / len_meters);

        // Convertimos la perpendicular métrica a píxeles para aplicarla sobre las coordenadas
        let perp_x_px = perp_x_m / matrix.size_x;
        let perp_y_px = perp_y_m / matrix.size_y;

        let harmonics: Vec<(f64, f64)> = (1..=4)
            .map(|k| {
                let amplitude = max_offset_meters / k as f64;
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

            // offset en metros, con envelope que vale 0 en los extremos
            let offset_m = (raw / max_possible) * max_offset_meters * t.sin();

            // Aplicamos el offset en píxeles usando la perpendicular ya convertida
            let nx = path[idx].0 as f64 + perp_x_px * offset_m;
            let ny = path[idx].1 as f64 + perp_y_px * offset_m;

            if valid(matrix, nx, ny) {
                result[idx] = (nx.round() as usize, ny.round() as usize);
            }
        }
    }

    result
}

fn build_leg(
    matrix: &DepthMatrix,
    center_x: f64, center_y: f64,
    perpendicular_x: f64, perpendicular_y: f64,
    dir_x: f64, dir_y: f64,
    diagonal: f64, separation_px: f64,
    leg: i32,
) -> Vec<(f64, f64)> {

    let offset = leg as f64 * separation_px;
    let origin_x = center_x + perpendicular_x * offset;
    let origin_y = center_y + perpendicular_y * offset;
    let mut line = Vec::new();
    let mut d = -diagonal / 2.0;

    while d <= diagonal / 2.0 {
        let x = origin_x + dir_x * d;
        let y = origin_y + dir_y * d;
        if valid(matrix, x, y) {
            line.push((x, y));
        }
        d += 1.0;
    }

    line
}

fn connect(
    matrix: &DepthMatrix,
    start: (f64, f64),
    end: (f64, f64),
    path: &mut Vec<(usize, usize)>,
) {
    let (x0, y0) = start;
    let (x1, y1) = end;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let steps = dx.abs().max(dy.abs()).ceil() as i32;

    if steps == 0 { return; }

    for current_step in 1..=steps {
        let t = current_step as f64 / steps as f64;
        let px = x0 + dx * t;
        let py = y0 + dy * t;
        if valid(matrix, px, py) {
            path.push((px.round() as usize, py.round() as usize));
        }
    }
}

fn valid(matrix: &DepthMatrix, x: f64, y: f64) -> bool {
    let xi = x.round() as isize;
    let yi = y.round() as isize;
    xi >= 0 && yi >= 0
        && xi < matrix.width as isize
        && yi < matrix.height as isize
        && Some(matrix.data[yi as usize][xi as usize]) != matrix.no_data
}

fn update_previous_end(line: &Vec<(f64, f64)>, previous_end: &mut Option<(f64, f64)>) {
    if let Some(last_point) = line.last() {
        *previous_end = Some(*last_point);
    }
}