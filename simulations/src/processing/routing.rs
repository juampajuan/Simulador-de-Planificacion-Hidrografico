use crate::structs::depth_matrix::DepthMatrix;

// Genera un recorrido sobre la matriz de profundidad, con un azimut y separación dados. El resultado es un vector de coordenadas (x, y) que representan el recorrido.
// El azimut se mide en grados, con 0° apuntando hacia el norte y aumentando en sentido horario. La separación se mide en metros y determina la distancia entre las piernas del zig-zag.
// Devuelvo todo el recorrido mas que nada porque puede servir para el front
pub fn generate_route(matrix: &DepthMatrix, azimuth_deg: f64, separation_meters: f64) -> Vec<(usize, usize)> {

    let mut path = Vec::new();

    let w = matrix.width as f64;
    let h = matrix.height as f64;

    let center_x = w / 2.0;
    let center_y = h / 2.0;

    let diagonal = (w.powi(2) + h.powi(2)).sqrt();

    // Aca obtengo el angulo
    let angle = azimuth_deg.to_radians();
    let (sin_a, cos_a) = angle.sin_cos();

    // Cuanto mide un pixel en metros
    let size_x = matrix.size_x;
    let size_y = matrix.size_y;

    // Dirección principal normalizada para poder usarla mas adelante como vector director. 
    // La divido por el tamaño del pixel para que la dirección esté en unidades de píxeles, y luego la normalizo para que tenga magnitud 1.
    let dir_x_px = sin_a / size_x;
    let dir_y_px = -cos_a / size_y;

    let mag_dir = (dir_x_px.powi(2) + dir_y_px.powi(2)).sqrt();

    let dir_x = dir_x_px / mag_dir;
    let dir_y = dir_y_px / mag_dir;

    // Esta es la dicc perpendicular dividida por el tamaño del pixel.
    let perpendicular_x_px = cos_a / size_x;
    let perpendicular_y_px = sin_a / size_y;

    let mag_perpendicular = (perpendicular_x_px.powi(2) + perpendicular_y_px.powi(2)).sqrt();

    // Esta es la direccion perpendicular normalizada.
    let perpendicular_x = perpendicular_x_px / mag_perpendicular;
    let perpendicular_y = perpendicular_y_px / mag_perpendicular;

    // Separación entre piernas en píxeles. Osea digamos la cantidad de píxeles que tengo que avanzar en la dirección perpendicular para lograr la separación deseada en metros.
    let separation_px = separation_meters * mag_perpendicular;

    let legs = (diagonal / separation_px).ceil() as i32;

    let mut previous_end: Option<(f64, f64)> = None;

    for leg in -legs / 2..=legs / 2 {

        let mut line = build_leg(matrix, center_x, center_y, perpendicular_x, perpendicular_y, dir_x, dir_y, diagonal, separation_px,  leg);

        if line.is_empty() {
            continue;
        }

        // Para el zig-zag. Despues conectaria la punta de esta pata con la anterior para que quedo un camino continuo
        // Por ahora lo dejo así para probar.
        if leg % 2 != 0 {
            line.reverse();
        }

        // Conecto la pierna con la otra
        if let Some(prev) = previous_end {
            connect(matrix, prev, line[0], &mut path);
        }        

        // Agregar pierna actual
        path.extend(
            line.iter().map(|(x, y)| {
                (x.round() as usize, y.round() as usize)
            })
        );

        //Guardamos el último punto de esta pierna para conectarlo con el inicio de la que viene
        update_previous_end(&line, &mut previous_end);
    }

    path
}

fn build_leg(matrix: &DepthMatrix, center_x: f64, center_y: f64, perpendicular_x: f64, perpendicular_y: f64, dir_x: f64, dir_y: f64, diagonal: f64, separation_px: f64, leg: i32) -> Vec<(f64, f64)> {

    let offset = leg as f64 * separation_px;

    // Este es el punto de origen de la pierna, que se desplaza a lo largo de la dirección perpendicular. Tanto para X como para Y.
    let origin_x = center_x + perpendicular_x * offset;
    let origin_y = center_y + perpendicular_y * offset;

    let mut line = Vec::new();

    let mut d = -diagonal / 2.0;

    while d <= diagonal / 2.0 {

        // Sobre el punto de origen obtenido en las lineas 80 y 81, avanzo en la dirección del azimut para generar el recorrido de la pierna. Tanto para X como para Y.
        let x = origin_x + dir_x * d;
        let y = origin_y + dir_y * d;

        if valid(matrix, x, y) {
            line.push((x, y));
        }

        d += 1.0;
    }

    line
}

fn connect(matrix: &DepthMatrix, start: (f64, f64), end: (f64, f64), path: &mut Vec<(usize, usize)>,) {
    // Rellena los puntos intermedios entre el final de una pierna y el inicio de la otra
    
    let (x0, y0) = start;
    let (x1, y1) = end;

    let dx = x1 - x0;
    let dy = y1 - y0;

    // Calculamos la cantidad de pasos
    let steps = dx.abs().max(dy.abs()).ceil() as i32;

    if steps == 0 {
        return;
    }

    let mut current_step = 1;

    while current_step <= steps {
        
        // Calculamos la proporcion del avance
        let t = current_step as f64 / steps as f64;
        
        let px = x0 + (dx * t);
        let py = y0 + (dy * t);

        if valid(matrix, px, py) {
            path.push((px.round() as usize, py.round() as usize));
        }

        current_step = current_step + 1;
    }
}

fn valid(matrix: &DepthMatrix, x: f64, y: f64) -> bool {

    let xi = x.round() as isize;
    let yi = y.round() as isize;

    xi >= 0 && yi >= 0 && xi < matrix.width as isize && yi < matrix.height as isize && Some(matrix.data[yi as usize][xi as usize]) != matrix.no_data
}

fn update_previous_end(line: &Vec<(f64, f64)>, previous_end: &mut Option<(f64, f64)>) {

    match line.last() {
        
        Some(last_point) => {
            *previous_end = Some(*last_point);
        },
        _ => {} 
    }
}

use rand::{Rng, RngExt};

pub fn apply_gnss_noise(
    path: &[(usize, usize)],
    matrix: &DepthMatrix,
    max_offset_meters: f64,
) -> Vec<(usize, usize)> {
    let mut rng = rand::rng();
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

    let mut segments: Vec<(usize, usize)> = vec![];
    let mut start = 0;
    for i in 1..n {
        if is_turn[i] {
            if i > start + 1 { segments.push((start, i)); }
            start = i;
        }
    }

    let mut result: Vec<(usize, usize)> = path.to_vec();

    for (seg_start, seg_end) in segments {
        let seg_len = seg_end - seg_start;

        let (xs, ys) = (path[seg_start].0 as f64, path[seg_start].1 as f64);
        let (xe, ye) = (path[seg_end].0 as f64, path[seg_end].1 as f64);
        let dx = xe - xs;
        let dy = ye - ys;
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 { continue; }
        let (px, py) = (-dy / len, dx / len);


        let harmonics: Vec<(f64, f64)> = (1..=4)
            .map(|k| {
                let amplitude = max_offset_meters / k as f64;
                let phase: f64 = rng.random::<f64>() * std::f64::consts::TAU;
                (amplitude, phase)
            })
            .collect();

        let max_possible: f64 = harmonics.iter().map(|(a, _)| a).sum();

        for j in 0..=seg_len {
            let idx = seg_start + j;
            let t = j as f64 / seg_len as f64 * std::f64::consts::PI;

            // Suma de armónicos, todos multiplicados por sin(t) para forzar 0 en extremos
            let raw: f64 = harmonics.iter()
                .enumerate()
                .map(|(ki, (amp, phase))| {
                    let k = ki + 1;
                    amp * (k as f64 * t + phase).sin()
                })
                .sum();

            // Envelope: sin(t) fuerza que los extremos sean exactamente 0
            let offset = (raw / max_possible) * max_offset_meters * t.sin();

            let (x0, y0) = (path[idx].0 as f64, path[idx].1 as f64);
            let nx = x0 + px * offset / matrix.size_x;
            let ny = y0 + py * offset / matrix.size_y;

            if valid(matrix, nx, ny) {
                result[idx] = (nx.round() as usize, ny.round() as usize);
            }
        }
    }

    result
}