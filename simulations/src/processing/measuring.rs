use crate::{processing::measuring, structs::depth_matrix::DepthMatrix, structs::student_measuring_parameters::StudentMeasuringParameters, structs::professor_parameters::ProfessorParameters};

pub enum MeasureMode {
    Perpendicular { step_distance: f64 },
    Circular { angle: f64 },
}

//reemplazar nombre a calculoar puntos Y tomr las mediciones.
pub fn find_measuring_points(path: &Vec<(usize, usize)>, distance_between_points: f64) -> Vec<(usize, usize)> {
    
    //convertir distancias a metros

    let mut measuring_points: Vec<(usize, usize)> = Vec::new();

    let mut current_point = &(path[0]);
    
    let mut distance_progress: f64 = 0.0;
    
    measuring_points.push(*current_point);

    for next_point in path {

        let current_distance = calculate_distance_between_points(current_point, next_point);

        distance_progress += current_distance;

        if distance_progress >= distance_between_points{
            distance_progress -= distance_between_points;

            // Posible mejora para elegir cual de los dos pixeles tomar como punto segun a cuanto estan del punto real
            // let exceso = distance_progress - distance_between_points;
            // let falta = distance_between_points - (distance_progress - current_distance);
            //Aca quiza entra lo que habia dicho el porfesor, de elegir cual de los dos puntos esta mas cerca realmente.
            //Con esto me refiero a la charla del 13 de mayo, que Juampa estuvo casi solo.

            //Funcion aqui:
            measuring_points.push(*current_point);
        }
        current_point = next_point;
    }
    measuring_points
}

pub fn calculate_distance_between_points(point_a: &(usize, usize), point_b: &(usize, usize)) -> f64 {
    
    let a_x = point_a.0 as f64;
    let a_y = point_a.1 as f64;
    let b_x = point_b.0 as f64;
    let b_y = point_b.1 as f64;

    ((a_x - b_x).powi(2) + (a_y - b_y).powi(2)).sqrt()
}

pub fn get_measures(mode: MeasureMode, matrix: &DepthMatrix, measure_points: &Vec<(usize, usize)>) -> Vec<Vec<f64>> {
    let mut resulting_measures: Vec<Vec<f64>> = vec![vec![0.0; matrix.width]; matrix.height];

    let mut previous_point: Option<&(usize, usize)> = None;
    let mut current_point = &(measure_points[0]);

    let get_group = |prev: Option<&(usize, usize)>, cur: &(usize, usize), next: Option<&(usize, usize)>| {
        match mode {
            MeasureMode::Perpendicular { step_distance } => get_points_perpendicular_to_this(prev, cur, next, step_distance),
            MeasureMode::Circular { angle } => get_points_circular_to_this(cur, angle, &matrix),
        }
    };

    for next_point in measure_points {
        let current_point_group = get_group(previous_point, current_point, Some(next_point));
        resulting_measures[current_point.1][current_point.0] = calculate_measure(current_point_group, matrix);
        previous_point = Some(current_point);
        current_point = next_point;
    }
    let last_point_group = get_group(previous_point, current_point, None);
    resulting_measures[current_point.1][current_point.0] = calculate_measure(last_point_group, matrix);

    resulting_measures
}

fn calculate_measure(points: Vec<(usize, usize)>, matrix: &DepthMatrix) -> f64 {

    // EN esta funcion se deberia añadir las perturbaciones
    
    let mut measure = 0.0;
    let mut measures_counter = 0.0;
    for point in points {
        let x = point.0;
        let y = point.1;
        if x < matrix.width && y < matrix.height {
            let pixel_val = matrix.data[y][x];
            
            // Validamos que el píxel actual no sea un valor "null" / "no_data"
            if Some(pixel_val) != matrix.no_data {
                measure += pixel_val;
                measures_counter += 1.0;
            } 
        }
    }

    measure / measures_counter
}

pub fn get_points_perpendicular_to_this(prev_point: Option<&(usize, usize)>, current_point: &(usize, usize), next_point: Option<&(usize, usize)>, step_distance: f64) -> Vec<(usize, usize)> {
    // Elegir referencia según cuál existe y cuál está más lejos
    let reference = match (prev_point, next_point) {
        (Some(prev), Some(next)) => {
            let dist_to_prev = calculate_distance_between_points(current_point, prev);
            let dist_to_next = calculate_distance_between_points(current_point, next);
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
    if magnitude == 0.0 {
        return Vec::new();
    }
    let dx_norm = dx / magnitude;
    let dy_norm = dy / magnitude;

    //90 grados
    let perp_x = -dy_norm;
    let perp_y = dx_norm;

    //Coordenadas actuales
    let cx = current_point.0 as f64;
    let cy = current_point.1 as f64;

    let mut points = Vec::new();

    //Hayq eu hacer que se mida una cantidad de puntos ingresada por paramtro y lo mismo con el salto entra cada punto
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

pub fn calculate_covered_radius(current_point: &(usize, usize), angle: f64, matrix: &DepthMatrix) -> f64{
    //Para esta nueva version de los puntos circulares, se utilizara la formula de las diapositivas de Fernando.
    //a = 2 z tan(fi/2) -> Esto es el diametro, asi que lo divido por 2 para el radio
    //A = pi(z tan(fi/2))**2
    //z es la profundidad del punto central

    let z = matrix.data[current_point.1][current_point.0];

    //ojo al usar radius, porque hay que ver si el alumno entiene que da solo el radio del cono o el diametro.
    let a = z * (angle).tan();

    a.abs()
}

pub fn get_points_circular_to_this(current_point: &(usize, usize), angle: f64, matrix: &DepthMatrix) -> Vec<(usize, usize)> {
    let mut points = Vec::new();

    let center_x = current_point.0 as f64;
    let center_y = current_point.1 as f64;
    
    let radius = calculate_covered_radius(current_point, angle, matrix);

    let squared_radius = radius * radius;
    
    // Definimos los límites de búsqueda controlando que no bajen de 0 (protección contra underflow)
    let min_x = if center_x > radius { (center_x - radius).floor() as usize } else { 0 };
    let max_x = (center_x + radius).ceil() as usize;
    
    let min_y = if center_y > radius { (center_y - radius).floor() as usize } else { 0 };
    let max_y = (center_y + radius).ceil() as usize;
    
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

// pub struct StudentMeasuringParameters {
//     pub uses_mathegapher: bool,
//     pub uses_sound_profiler: bool,
//     pub uses_inertial_sensor: bool,
//     pub echo_sounder_parameters: EchosounderParameters,
//     pub boat: Boat
// }

// pub struct EchosounderParameters {
//     pub max_limit: f64,
//     pub min_limit: f64,
//     pub pulse_repetition_interval: usize,
//     pub pulse_length: usize,
//     pub uses_high_frecuency: bool,
//     pub angle: f32,
//     pub transmited_potency: f64,
//     pub gain: f32,
//     pub echosounder_velocity: usize,
// }


// pub enum Boat {
//     Small { speed: f64, balance_index: usize },
//     Medium { speed: f64, balance_index: usize},
//     Large { speed: f64, balance_index: usize}
// }

pub fn make_measurement(matrix: &DepthMatrix, current_point: &(usize, usize), student_parameters: StudentMeasuringParameters, teacher_parameters: ProfessorParameters) -> Option<f64>{

    let depth = matrix.data[current_point.1][current_point.0];
    let echosounder_parameters = student_parameters.echo_sounder_parameters;

    //El t este es puramente para darle cierto 
    let t = depth/1500.0;

    let pc = (t * echosounder_parameters.echosounder_velocity as f64)/2.0 - teacher_parameters.tide;

    if (pc > student_parameters.echo_sounder_parameters.max_limit) || (pc < student_parameters.echo_sounder_parameters.min_limit){
        return None;
    }


    Some(pc)
}