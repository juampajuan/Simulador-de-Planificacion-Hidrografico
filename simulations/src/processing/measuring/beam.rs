use crate::structs::depth_matrix::DepthMatrix;
use super::points::calculate_distance_between_points;

const SOUND_VELOCITY :f64 = 1500.0;

#[allow(dead_code)]
pub enum MeasureMode {
    Perpendicular { step_distance: f64 },
    Circular { angle: f64 },
}

pub fn get_measures(
    mode: MeasureMode,
    matrix: &DepthMatrix,
    measure_points: &Vec<(usize, usize)>,
    threshold: f64,
) -> Vec<Vec<f64>> {
    let mut resulting_measures: Vec<Vec<f64>> = vec![vec![0.0; matrix.width]; matrix.height];

    let mut previous_point: Option<&(usize, usize)> = None;
    let mut current_point = &measure_points[0];

    let get_group = |prev: Option<&(usize, usize)>, cur: &(usize, usize), next: Option<&(usize, usize)>| {
        match mode {
            MeasureMode::Perpendicular { step_distance } =>
                get_points_perpendicular_to_this(prev, cur, next, step_distance, matrix),
            MeasureMode::Circular { angle } =>
                get_points_circular_to_this(cur, angle, matrix),
        }
    };

    for next_point in measure_points {
        let group = get_group(previous_point, current_point, Some(next_point));
        resulting_measures[current_point.1][current_point.0] =
            calculate_measure(group, matrix, threshold);
        previous_point = Some(current_point);
        current_point = next_point;
    }

    let last_group = get_group(previous_point, current_point, None);
    resulting_measures[current_point.1][current_point.0] =
        calculate_measure(last_group, matrix, threshold);

    resulting_measures
}

fn calculate_measure(points: Vec<(usize, usize)>, matrix: &DepthMatrix, threshold: f64) -> f64 {
    let mut measure = 0.0;

    for point in points {
        let (x, y) = (point.0, point.1);
        if x < matrix.width && y < matrix.height {
            let pixel_val = matrix.data[y][x];
            let pixel_time = pixel_val / SOUND_VELOCITY;

            if Some(pixel_val) != matrix.no_data && pixel_time <= threshold && (pixel_val < measure || measure == 0.0) {
                measure = pixel_val;
            }
        }
    }

    measure
}

fn get_points_perpendicular_to_this(
    prev_point: Option<&(usize, usize)>,
    current_point: &(usize, usize),
    next_point: Option<&(usize, usize)>,
    step_distance: f64,
    matrix: &DepthMatrix,
) -> Vec<(usize, usize)> {
    // Elegir referencia según cuál existe y cuál está más lejos

    let reference = match (prev_point, next_point) {
        (Some(prev), Some(next)) => {
            let dist_to_prev = calculate_distance_between_points(current_point, prev, matrix);
            let dist_to_next = calculate_distance_between_points(current_point, next, matrix);
            if dist_to_prev >= dist_to_next { prev } else { next }
        }
        (Some(prev), None) => prev,
        (None, Some(next)) => next,
        (None, None) => return Vec::new(),
    };

    //Forma el vector
    let dx = reference.0 as f64 - current_point.0 as f64;
    let dy = reference.1 as f64 - current_point.1 as f64;
    let magnitude = (dx * dx + dy * dy).sqrt();
    if magnitude == 0.0 { return Vec::new(); }

    let dx_norm = dx / magnitude;
    let dy_norm = dy / magnitude;

    //90 grados
    let perp_x = -dy_norm;
    let perp_y = dx_norm;

    //Coordenadas actuales
    let cx = current_point.0 as f64;
    let cy = current_point.1 as f64;
    let mut points = Vec::new();

    //Hay que hacer que se mida una cantidad de puntos ingresada por parametro y lo mismo con el salto entra cada punto
    for i in 1..=5_i32 {
        let dist = i as f64 * step_distance;

        let lx = cx + perp_x * dist;
        let ly = cy + perp_y * dist;
        if lx >= 0.0 && ly >= 0.0 {
            points.push((lx.round() as usize, ly.round() as usize));
        }

        let rx = cx - perp_x * dist;
        let ry = cy - perp_y * dist;
        if rx >= 0.0 && ry >= 0.0 {
            points.push((rx.round() as usize, ry.round() as usize));
        }
    }

    points
}

fn calculate_covered_radius(current_point: &(usize, usize), angle_deg: f64, matrix: &DepthMatrix) -> f64 {
    let z = matrix.data[current_point.1][current_point.0];
    let a = z * (angle_deg).tan();
    a.abs()
}

fn get_points_circular_to_this(
    current_point: &(usize, usize),
    angle: f64,
    matrix: &DepthMatrix,
) -> Vec<(usize, usize)> {
    let center_x = current_point.0 as f64;
    let center_y = current_point.1 as f64;
    let radius = calculate_covered_radius(current_point, angle, matrix);
    let squared_radius = radius * radius;

    // Definimos los límites de búsqueda controlando que no bajen de 0 (protección contra underflow)
    let min_x = if center_x > radius { (center_x - radius).floor() as usize } else { 0 };
    let max_x = (center_x + radius).ceil() as usize;
    let min_y = if center_y > radius { (center_y - radius).floor() as usize } else { 0 };
    let max_y = (center_y + radius).ceil() as usize;

    let mut points = Vec::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let dx = x as f64 - center_x;
            let dy = y as f64 - center_y;

            // Condición del círculo: Pitágoras
            if dx * dx + dy * dy <= squared_radius {
                points.push((x, y));
            }
        }
    }

    points
}