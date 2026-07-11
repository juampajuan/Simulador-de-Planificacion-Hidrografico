use super::gnss::apply_gnss_noise_segmented;
use crate::structs::depth_matrix::DepthMatrix;

pub fn generate_route(
    matrix: &DepthMatrix,
    azimuth_deg: f64,
    separation_meters: f64,
    max_offset_meters: f64,
) -> Vec<(usize, usize)> {
    let geometry = RouteGeometry::new(matrix, azimuth_deg, separation_meters);
    let (path, segments) = build_path(matrix, &geometry);
    apply_gnss_noise_segmented(&path, &segments, matrix, max_offset_meters)
}

// ------------------------------------------------------------
//  Geometría del recorrido
// ------------------------------------------------------------

struct RouteGeometry {
    center_x: f64,
    center_y: f64,
    diagonal: f64,
    dir_x: f64,
    dir_y: f64,
    perpendicular_x: f64,
    perpendicular_y: f64,
    separation_px: f64,
    legs: i32,
}

impl RouteGeometry {
    fn new(matrix: &DepthMatrix, azimuth_deg: f64, separation_meters: f64) -> Self {
        let w = matrix.width as f64;
        let h = matrix.height as f64;
        let diagonal = (w.powi(2) + h.powi(2)).sqrt();

        let angle = azimuth_deg.to_radians();
        let (sin_a, cos_a) = angle.sin_cos();

        let dir_x_px = sin_a / matrix.size_x;
        let dir_y_px = -cos_a / matrix.size_y;
        let mag_dir = (dir_x_px.powi(2) + dir_y_px.powi(2)).sqrt();

        let perp_x_px = cos_a / matrix.size_x;
        let perp_y_px = sin_a / matrix.size_y;
        let mag_perp = (perp_x_px.powi(2) + perp_y_px.powi(2)).sqrt();

        let separation_px = separation_meters * mag_perp;

        Self {
            center_x: w / 2.0,
            center_y: h / 2.0,
            diagonal,
            dir_x: dir_x_px / mag_dir,
            dir_y: dir_y_px / mag_dir,
            perpendicular_x: perp_x_px / mag_perp,
            perpendicular_y: perp_y_px / mag_perp,
            separation_px,
            legs: (diagonal / separation_px).ceil() as i32,
        }
    }
}

// ------------------------------------------------------------
//  Construcción del path
// ------------------------------------------------------------

fn build_path(
    matrix: &DepthMatrix,
    geo: &RouteGeometry,
) -> (Vec<(usize, usize)>, Vec<std::ops::Range<usize>>) {
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut segments: Vec<std::ops::Range<usize>> = Vec::new();
    let mut previous_end: Option<(f64, f64)> = None;

    for leg in -geo.legs / 2..=geo.legs / 2 {
        let mut line = build_leg(matrix, geo, leg);
        if line.is_empty() {
            continue;
        }
        if leg % 2 != 0 {
            line.reverse();
        }

        if let Some(prev) = previous_end {
            let seg_start = path.len();
            connect(matrix, prev, line[0], &mut path);
            let seg_end = path.len();
            if seg_end > seg_start {
                segments.push(seg_start..seg_end);
            }
        }

        let seg_start = path.len();
        path.extend(
            line.iter()
                .map(|(x, y)| (x.round() as usize, y.round() as usize)),
        );
        let seg_end = path.len();
        if seg_end > seg_start {
            segments.push(seg_start..seg_end);
        }

        update_previous_end(&line, &mut previous_end);
    }

    (path, segments)
}

// ------------------------------------------------------------
//  Helpers
// ------------------------------------------------------------

/// Construye las piernas del recorrido
fn build_leg(matrix: &DepthMatrix, geo: &RouteGeometry, leg: i32) -> Vec<(f64, f64)> {
    let offset = leg as f64 * geo.separation_px;
    let origin_x = geo.center_x + geo.perpendicular_x * offset;
    let origin_y = geo.center_y + geo.perpendicular_y * offset;

    let mut line = Vec::new();
    let mut d = -geo.diagonal / 2.0;

    while d <= geo.diagonal / 2.0 {
        let x = origin_x + geo.dir_x * d;
        let y = origin_y + geo.dir_y * d;
        if valid(matrix, x, y) {
            line.push((x, y));
        }
        d += 1.0;
    }

    line
}

/// Conecta las piernas del recorrido
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

    if steps == 0 {
        return;
    }

    for current_step in 1..=steps {
        let t = current_step as f64 / steps as f64;
        let px = x0 + dx * t;
        let py = y0 + dy * t;
        if valid(matrix, px, py) {
            path.push((px.round() as usize, py.round() as usize));
        }
    }
}

/// Valida que el punto no se salga fuera de la matriz
fn valid(matrix: &DepthMatrix, x: f64, y: f64) -> bool {
    let xi = x.round() as isize;
    let yi = y.round() as isize;
    xi >= 0
        && yi >= 0
        && xi < matrix.width as isize
        && yi < matrix.height as isize
        && Some(matrix.data[yi as usize][xi as usize]) != matrix.no_data
}

/// Busca el final de la pierna anterior para poder crear la union desde ahi
fn update_previous_end(line: &[(f64, f64)], previous_end: &mut Option<(f64, f64)>) {
    if let Some(last_point) = line.last() {
        *previous_end = Some(*last_point);
    }
}
