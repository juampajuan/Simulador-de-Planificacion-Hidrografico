use crate::{structs::depth_matrix::DepthMatrix};

pub enum MeasureMode {
    Perpendicular { step_distance: f64 },
    Circular { angle: f64 },
}

pub fn find_measuring_points(path: &Vec<(usize, usize)>, distance_between_points: f64, matrix: &DepthMatrix) -> Vec<(usize, usize)> {
    
    //convertir distancias a metros

    let mut measuring_points: Vec<(usize, usize)> = Vec::new();

    let mut current_point = &(path[0]);
    
    let mut distance_progress: f64 = 0.0;
    
    measuring_points.push(*current_point);

    for next_point in path {

        let current_distance = calculate_distance_between_points(current_point, next_point, matrix);

        distance_progress += current_distance;

        if distance_progress >= distance_between_points{
            distance_progress -= distance_between_points;

            // Posible mejora para elegir cual de los dos pixeles tomar como punto segun a cuanto estan del punto real
            // let exceso = distance_progress - distance_between_points;
            // let falta = distance_between_points - (distance_progress - current_distance);

            measuring_points.push(*current_point);
        }
        current_point = next_point;
    }
    measuring_points
}

pub fn calculate_distance_between_points(point_a: &(usize, usize), point_b: &(usize, usize), matrix: &DepthMatrix) -> f64 {
    
    let a_x = point_a.0 as f64 * matrix.size_x;
    let a_y = point_a.1 as f64 * matrix.size_y;
    let b_x = point_b.0 as f64 * matrix.size_x;
    let b_y = point_b.1 as f64 * matrix.size_y;

    ((a_x - b_x).powi(2) + (a_y - b_y).powi(2)).sqrt()
}

pub fn get_measures(mode: MeasureMode, matrix: &DepthMatrix, measure_points: &Vec<(usize, usize)>, threshold: f64) -> Vec<Vec<f64>> {
    let mut resulting_measures: Vec<Vec<f64>> = vec![vec![0.0; matrix.width]; matrix.height];

    let mut previous_point: Option<&(usize, usize)> = None;
    let mut current_point = &(measure_points[0]);

    let get_group = |prev: Option<&(usize, usize)>, cur: &(usize, usize), next: Option<&(usize, usize)>| {
        match mode {
            MeasureMode::Perpendicular { step_distance } => get_points_perpendicular_to_this(prev, cur, next, step_distance, matrix),
            MeasureMode::Circular { angle } => get_points_circular_to_this(cur, angle, &matrix),
        }
    };

    for next_point in measure_points {
        let current_point_group = get_group(previous_point, current_point, Some(next_point));
        resulting_measures[current_point.1][current_point.0] = calculate_measure(current_point_group, matrix, threshold);
        previous_point = Some(current_point);
        current_point = next_point;
    }
    let last_point_group = get_group(previous_point, current_point, None);
    resulting_measures[current_point.1][current_point.0] = calculate_measure(last_point_group, matrix, threshold);

    resulting_measures
}

fn calculate_measure(points: Vec<(usize, usize)>, matrix: &DepthMatrix, threshold: f64) -> f64 {

    // let tiempoquetomamedicion = distancia/velocidaddelagua
    
    let mut measure = 0.0;
    for point in points {
        let x = point.0;
        let y = point.1;
        if x < matrix.width && y < matrix.height {
            let pixel_val = matrix.data[y][x];

            //Distnacia sobre velocidad del agua: metros sobre metros/segundo
            let pixel_time = pixel_val/1500.0;
            
            // Validamos que el píxel actual no sea un valor "null" / "no_data"
            if (Some(pixel_val) != matrix.no_data) && (pixel_time <= threshold) {
                if (pixel_val > measure) || (measure == 0.0){
                    measure = pixel_val;
                }
            } 
        }
    }

    measure
}

pub fn get_points_perpendicular_to_this(prev_point: Option<&(usize, usize)>, current_point: &(usize, usize), next_point: Option<&(usize, usize)>, step_distance: f64, matrix: &DepthMatrix) -> Vec<(usize, usize)> {
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



// 4. Longitud del pulso
// Determina la energía que se transmite al agua. Pulsos más largos → más energía → más alcance. Pulsos más cortos → mejor resolución vertical.
// 5. Potencia transmitida
// Debe mantenerse en el valor más bajo posible que aún permita detectar el fondo. Si se sube demasiado se generan ecos falsos.
// 6. Ganancia
// Amplificación del eco de retorno. Si es demasiado baja, el eco llega redondeado y el sondaje resulta mayor al real. Si es demasiado alta, se registran falsas reflexiones de peces, vegetación o ruido.

// punto de medición
//     → aplicar error por pulse_length: afecta resolución vertical (dos blancos < longitud de pulso no se distinguen)
//     → aplicar error por gain: si gain muy alta → ruido aleatorio; si muy baja → valor levemente mayor