use crate::structs::depth_matrix::DepthMatrix;
use crate::structs::measurement_type::MeasurementsType;
use super::points::calculate_distance_between_points;

#[allow(dead_code)]
pub enum MeasureMode {
    Perpendicular {  },
    Circular { angle: f64 },
}

pub fn get_measures(
    mode: MeasureMode,
    matrix: &DepthMatrix,
    measure_points: &Vec<(usize, usize)>,
) -> MeasurementsType {
    let mut resulting_measures: Vec<((usize, usize), f64)> = Vec::new();

    match mode {
        MeasureMode::Circular { angle } => {
            for point in measure_points {
                let group = get_points_circular_to_this(point, angle, matrix);
                resulting_measures.push(((point.0, point.1), calculate_measure(group, matrix)));
            }
            MeasurementsType::Monohaz { measurements: (resulting_measures) }
        },
        MeasureMode::Perpendicular { } => {

            let mut previous_point: Option<&(usize, usize)> = None;
            let mut current_point = &measure_points[0];

            let mut right_group: Vec<((usize, usize), f64)> = Vec::new();
            let mut left_group: Vec<((usize, usize), f64)> = Vec::new();

            for next_point in measure_points {
                let [left, center, right] = get_points_perpendicular_to_this(previous_point, current_point, Some(next_point), matrix);

                resulting_measures.extend(center);
                left_group.extend(left);
                right_group.extend(right);

                previous_point = Some(current_point);
                current_point = next_point;
            }

            let [left, center, right] = get_points_perpendicular_to_this(previous_point, current_point, None, matrix);
            resulting_measures.extend(center);
            left_group.extend(left);
            right_group.extend(right);
            MeasurementsType::Multihaz { central_measurments: (resulting_measures), paralel_measurment_1: (left_group), paralel_measurment_2: (right_group) }
        }
    }
}

fn calculate_measure(points: Vec<(usize, usize)>, matrix: &DepthMatrix) -> f64 {
    let mut measure = 0.0;

    for point in points {
        let (x, y) = (point.0, point.1);
        if x < matrix.width && y < matrix.height {
            let pixel_val = matrix.data[y][x];

            if Some(pixel_val) != matrix.no_data && (pixel_val < measure || measure == 0.0) {
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
    matrix: &DepthMatrix,
) -> [Vec<((usize,usize), f64)>; 3] {

    let reference = match (prev_point, next_point) {
        (Some(prev), Some(next)) => {
            let dist_to_prev = calculate_distance_between_points(current_point, prev, matrix);
            let dist_to_next = calculate_distance_between_points(current_point, next, matrix);
            if dist_to_prev >= dist_to_next { prev } else { next }
        }
        (Some(prev), None) => prev,
        (None, Some(next)) => next,
        (None, None) => { println!("Hola hubo error ambos son none"); return [vec![], vec![], vec![]]}
    };

    //Forma el vector
    let dx = reference.0 as f64 - current_point.0 as f64;
    let dy = reference.1 as f64 - current_point.1 as f64;
    let magnitude = (dx * dx + dy * dy).sqrt();
    if magnitude == 0.0 { println!("Hola hubo error magnitud es 0"); return [vec![], vec![], vec![]] }

    let dx_norm = dx / magnitude;
    let dy_norm = dy / magnitude;

    //90 grados
    let perp_x = -dy_norm;
    let perp_y = dx_norm;

    //Coordenadas actuales
    let cx = current_point.0 as f64;
    let cy = current_point.1 as f64;

    //Hay que hacer que se mida una cantidad de puntos ingresada por parametro y lo mismo con el salto entra cada punto

    let angle_deg:f64 = 60.0; // Ángulo del haz en grados
    let mitad_cobertura = (2.0*(matrix.data[current_point.1][current_point.0])*(angle_deg.to_radians()).tan())/2.0;

    let left_point_x = cx + mitad_cobertura * perp_x;
    let left_point_y = cy + mitad_cobertura * perp_y;

    let right_point_x = cx - mitad_cobertura * perp_x;
    let right_point_y = cy - mitad_cobertura * perp_y;
    

    let der_point: (usize, usize) = (right_point_x.round() as usize, right_point_y.round() as usize);
    let cent_point = (current_point.0, current_point.1);
    let izq_point = (left_point_x.round() as usize, left_point_y.round() as usize);

    let center_vector = vec![(cent_point, matrix.data[cent_point.1][cent_point.0])];

    let right_vector = get_points_on_line(cent_point, der_point, matrix);
    let left_vector = get_points_on_line(cent_point, izq_point, matrix);

    [left_vector, center_vector, right_vector]
}

pub fn get_points_on_line(
    starting_point: (usize, usize),
    ending_point: (usize, usize),
    matrix: &DepthMatrix,
) -> Vec<((usize, usize), f64)> {
    let mut points = Vec::new();

    let (x0, y0) = (starting_point.0 as i64, starting_point.1 as i64);
    let (x1, y1) = (ending_point.0 as i64, ending_point.1 as i64);

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx: i64 = if x1 >= x0 { 1 } else { -1 };
    let sy: i64 = if y1 >= y0 { 1 } else { -1 };

    let mut x = x0;
    let mut y = y0;
    let mut err = dx - dy;

    loop {
        if check_point_validity((x as usize, y as usize), matrix){
            points.push(((x as usize, y as usize), matrix.data[y as usize][x as usize]));
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x += sx;
        }

        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    points
}

fn check_point_validity(point: (usize, usize), matrix: &DepthMatrix) -> bool {
    let x = point.0;
    let y = point.1;

    if x < matrix.width && y < matrix.height{
        if let Some(no_data) = matrix.no_data {
            return matrix.data[y][x] != no_data;
        }
        return true;
    }

    false
}

fn calculate_covered_radius(current_point: &(usize, usize), angle_deg: f64, matrix: &DepthMatrix) -> f64 {
    let z = matrix.data[current_point.1][current_point.0];
    let a = z * (angle_deg.to_radians()/2.0).tan();
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