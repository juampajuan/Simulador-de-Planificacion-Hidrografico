use crate::structs::depth_matrix::DepthMatrix;
use crate::structs::measurement_type::MeasurementsType;
use super::points::calculate_distance_between_points;

#[allow(dead_code)]
pub enum MeasureMode {
    Perpendicular { angle: f64 },
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
        MeasureMode::Perpendicular { angle } => {
            //el avg_depth enviado como None en los parametros hace que se use la profundidad real de cada punto, en vez de una fija para todos
            //Se utilizaria una profundidad fija solamente para ver la cobertura de la sonda en toda la matriz, sin importar la profundidad real de cada punto.
            //Osea el boton de cobertura en la aplicacion.
            get_perpendicular_measurements(measure_points, angle, None, matrix)
        }
    }
}

pub fn get_perpendicular_measurements(
    measure_points: &Vec<(usize, usize)>,
    angle_deg: f64,
    avg_depth: Option<f64>,
    matrix: &DepthMatrix,
) -> MeasurementsType {
    let mut resulting_measures: Vec<((usize, usize), f64)> = Vec::new();
    let mut previous_point: Option<&(usize, usize)> = None;
    let mut current_point = &measure_points[0];
 
    let mut right_group: Vec<((usize, usize), f64)> = Vec::new();
    let mut left_group: Vec<((usize, usize), f64)> = Vec::new();
 
    for next_point in measure_points {
        let z = avg_depth.unwrap_or_else(|| matrix.data[current_point.1][current_point.0]);
        let [left, center, right] = get_points_perpendicular_to_this(previous_point, current_point, Some(next_point), angle_deg, z, matrix);
 
        resulting_measures.extend(center);
        left_group.extend(left);
        right_group.extend(right);
 
        previous_point = Some(current_point);
        current_point = next_point;
    }
 
    let z = avg_depth.unwrap_or_else(|| matrix.data[current_point.1][current_point.0]);
    let [left, center, right] = get_points_perpendicular_to_this(previous_point, current_point, None, angle_deg, z, matrix);
    resulting_measures.extend(center);
    left_group.extend(left);
    right_group.extend(right);
 
    MeasurementsType::Multihaz { central_measurments: (resulting_measures), paralel_measurment_1: (left_group), paralel_measurment_2: (right_group) }
}

///Esta funcion seria la simulacion de la sonda en un punto de medicion. Dados todos los puntos que registra la sonda, le da valor a la medicion del punto.
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

///Dados el punto actual, el anterior y el siguiente, determina la direccion actual en el punto y en base a eso reune todas las mediciones de la zona perpendicular, para Multihaz
fn get_points_perpendicular_to_this(
    prev_point: Option<&(usize, usize)>,
    current_point: &(usize, usize),
    next_point: Option<&(usize, usize)>,
    angle_deg: f64,
    z: f64,
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
        (None, None) => {return [vec![], vec![], vec![]]}
    };
 
    //Forma el vector
    let dx = reference.0 as f64 - current_point.0 as f64;
    let dy = reference.1 as f64 - current_point.1 as f64;
    let magnitude = (dx * dx + dy * dy).sqrt();
 
    let dx_norm = dx / magnitude;
    let dy_norm = dy / magnitude;
 
    //90 grados
    let perp_x = -dy_norm;
    let perp_y = dx_norm;
 
    //Coordenadas actuales
    let cx = current_point.0 as f64;
    let cy = current_point.1 as f64;
 
    //Hay que hacer que se mida una cantidad de puntos ingresada por parametro y lo mismo con el salto entra cada punto
    let mitad_cobertura = (2.0*z*((angle_deg/ 2.0).to_radians()).tan())/2.0/ matrix.size_x; 
 
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

///Reune los puntos que pertenecen a la recta formada entre dos puntos.
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

///Verifica que un punto tomado para medicion sea valido: Valor real, posicion real.
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

///Calcula el area cubierta para la sonda Monohaz
pub fn calculate_covered_radius(z: f64, angle_deg: f64, matrix: &DepthMatrix) -> f64 {
    let a = z * (angle_deg.to_radians()/2.0).tan();
    (a/matrix.size_x).abs()
}

/// Función central: retorna los puntos dentro de un radio específico.
pub fn get_points_in_radius(
    current_point: &(usize, usize),
    radius: f64,
    matrix: &DepthMatrix,
) -> Vec<(usize, usize)> {
    let center_x = current_point.0 as f64;
    let center_y = current_point.1 as f64;
    let squared_radius = radius * radius;

    // Definimos los límites de búsqueda controlando que no bajen de 0
    let min_x = if center_x > radius { (center_x - radius).floor() as usize } else { 0 };
    let max_x = ((center_x + radius).ceil() as usize).min(matrix.width - 1);
    let min_y = if center_y > radius { (center_y - radius).floor() as usize } else { 0 };
    let max_y = ((center_y + radius).ceil() as usize).min(matrix.height - 1);

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

/// Retorna todos los puntos en un area cercanos a un punto especifico,
/// calculando el radio en base a la profundidad de ese punto.
pub fn get_points_circular_to_this(
    current_point: &(usize, usize),
    angle: f64,
    matrix: &DepthMatrix,
) -> Vec<(usize, usize)> {
    let radius = calculate_covered_radius(matrix.data[current_point.1][current_point.0], angle, matrix);
    get_points_in_radius(current_point, radius, matrix)
}

#[cfg(test)]
mod tests {
    use super::*;
 
    // Matriz chica con profundidad grande y size_x grande tambien. 
    // El radio del haz en pixeles termina siendo enorme
    // en relacion al tamaño de la matriz
    fn matriz_chica_y_profunda(width: usize, height: usize, depth: f64, size_x: f64) -> DepthMatrix {
        DepthMatrix {
            data: vec![vec![depth; width]; height],
            width,
            height,
            no_data: None,
            size_x,
            size_y: size_x,
            geo_transform: [0.0, size_x, 0.0, 0.0, 0.0, -size_x],
            projection: String::new(),
        }
    }
 
    #[test]
    fn no_genera_puntos_fuera_de_los_limites_de_la_matriz() {
        // Caso real que hacia panic con GEBCO: punto pegado al borde
        // derecho/inferior, con un radio de cobertura mas grande que la
        // distancia al borde.
        let matrix = matriz_chica_y_profunda(101, 67, 2500.0, 413.0);
        let punto_en_el_borde = (100, 66); // ultima columna, ultima fila
 
        let puntos = get_points_circular_to_this(&punto_en_el_borde, 60.0, &matrix);
 
        for (x, y) in puntos {
            assert!(x < matrix.width, "x={x} se paso del ancho={}", matrix.width);
            assert!(y < matrix.height, "y={y} se paso del alto={}", matrix.height);
        }
    }
 
    #[test]
    fn el_punto_central_siempre_queda_incluido() {
        let matrix = matriz_chica_y_profunda(101, 67, 2500.0, 413.0);
        let centro = (50, 33);
 
        let puntos = get_points_circular_to_this(&centro, 60.0, &matrix);
 
        assert!(puntos.contains(&centro));
    }

    // Matriz donde la mitad izquierda es muy profunda y la mitad derecha
    // muy poco profunda -- para poder distinguir si el ancho del swath usa
    // la profundidad real de cada punto o una fija para todos.
    fn matriz_con_dos_profundidades(width: usize, height: usize) -> DepthMatrix {
        let mut data = vec![vec![0.0; width]; height];
        for row in data.iter_mut() {
            for (x, cell) in row.iter_mut().enumerate() {
                *cell = if x < width / 2 { 100.0 } else { 1.0 };
            }
        }
        DepthMatrix {
            data,
            width,
            height,
            no_data: None,
            size_x: 1.0,
            size_y: 1.0,
            geo_transform: [0.0, 1.0, 0.0, 0.0, 0.0, -1.0],
            projection: String::new(),
        }
    }

    #[test]
    fn con_profundidad_fija_el_ancho_del_swath_es_igual_en_toda_la_matriz() {
        let matrix = matriz_con_dos_profundidades(100, 20);
        let path: Vec<(usize, usize)> = (10..90).map(|x| (x, 10)).collect();

        let result = get_perpendicular_measurements(&path, 60.0, Some(50.0), &matrix);
        let MeasurementsType::Multihaz { paralel_measurment_1: left, .. } = result else {
            panic!("se esperaba Multihaz");
        };

        let ancho_en_x = |target_x: usize| -> usize {
            left.iter().filter(|(p, _)| p.0 == target_x).count()
        };
        let ancho_zona_profunda = ancho_en_x(30);
        let ancho_zona_poco_profunda = ancho_en_x(70);

        assert!(
            (ancho_zona_profunda as i64 - ancho_zona_poco_profunda as i64).abs() <= 1,
            "el ancho deberia ser igual con profundidad fija: profunda={ancho_zona_profunda}, poco profunda={ancho_zona_poco_profunda}"
        );
    }

    #[test]
    fn con_profundidad_real_el_ancho_del_swath_varia_segun_la_zona() {
        let matrix = matriz_con_dos_profundidades(100, 20);
        let path: Vec<(usize, usize)> = (10..90).map(|x| (x, 10)).collect();

        let result = get_perpendicular_measurements(&path, 60.0, None, &matrix);
        let MeasurementsType::Multihaz { paralel_measurment_1: left, .. } = result else {
            panic!("se esperaba Multihaz");
        };

        let ancho_en_x = |target_x: usize| -> usize {
            left.iter().filter(|(p, _)| p.0 == target_x).count()
        };
        let ancho_zona_profunda = ancho_en_x(30);
        let ancho_zona_poco_profunda = ancho_en_x(70);

        assert!(
            ancho_zona_profunda > ancho_zona_poco_profunda * 2,
            "con profundidad real se esperaba una diferencia notable: profunda={ancho_zona_profunda}, poco profunda={ancho_zona_poco_profunda}"
        );
    }
}