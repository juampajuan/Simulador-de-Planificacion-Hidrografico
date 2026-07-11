use crate::structs::depth_matrix::DepthMatrix;

///Dados una ruta de puntos representados como un vector de tuplas, y una distancia determinada, calcula cuales de los puntos de la ruta seran en los que se toma una medicion.
pub fn find_measuring_points(
    path: &Vec<(usize, usize)>,
    distance_between_points: f64,
    matrix: &DepthMatrix,
) -> Vec<(usize, usize)> {
    let mut measuring_points: Vec<(usize, usize)> = Vec::new();
    let mut current_point = &path[0];
    let mut distance_progress: f64 = 0.0;

    measuring_points.push(*current_point);

    for next_point in path {
        let current_distance = calculate_distance_between_points(current_point, next_point, matrix);
        distance_progress += current_distance;

        if distance_progress >= distance_between_points {
            distance_progress -= distance_between_points;
            if measuring_points.last() != Some(next_point) {
                measuring_points.push(*next_point);
            }
        }
        current_point = next_point;
    }

    measuring_points
}

///Da la distancia entre dos puntos
pub fn calculate_distance_between_points(
    point_a: &(usize, usize),
    point_b: &(usize, usize),
    matrix: &DepthMatrix,
) -> f64 {
    let a_x = point_a.0 as f64 * matrix.size_x;
    let a_y = point_a.1 as f64 * matrix.size_y;
    let b_x = point_b.0 as f64 * matrix.size_x;
    let b_y = point_b.1 as f64 * matrix.size_y;

    ((a_x - b_x).powi(2) + (a_y - b_y).powi(2)).sqrt()
}

//No se usa esta funcion.
#[allow(dead_code)]
fn calculate_effective_measurement_distance(
    distance_between_points: &f64,
    pixel_size: &f64,
) -> f64 {
    let min_pixels = 50.0;
    let min_distance = min_pixels * pixel_size;
    distance_between_points * min_distance / 0.1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matriz_simple(width: usize, height: usize) -> DepthMatrix {
        // size_x=size_y=1.0 -> cada pixel equivale a 1 metro, asi las
        // distancias en los tests son faciles de calcular a mano.
        DepthMatrix {
            data: vec![vec![5.0; width]; height],
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
    fn toma_un_punto_cada_distancia_pedida() {
        let matrix = matriz_simple(20, 5);

        // linea recta de (0,0) a (10,0): un punto por pixel
        let path: Vec<(usize, usize)> = (0..=10).map(|x| (x, 0)).collect();

        let puntos = find_measuring_points(&path, 5.0, &matrix);

        // primer punto siempre entra, despues cada 5 metros: 0, 5, 10
        assert_eq!(puntos, vec![(0, 0), (5, 0), (10, 0)]);
    }

    #[test]
    fn no_duplica_puntos_consecutivos_iguales() {
        let matrix = matriz_simple(20, 5);

        // el barco se queda "frenado" en el mismo pixel varias veces seguidas
        let mut path = vec![(0, 0), (0, 0), (0, 0)];
        path.extend((1..=5).map(|x| (x, 0)));

        let puntos = find_measuring_points(&path, 1.0, &matrix);

        // no debe haber dos elementos consecutivos iguales
        for ventana in puntos.windows(2) {
            assert_ne!(
                ventana[0], ventana[1],
                "se colaron puntos duplicados: {:?}",
                puntos
            );
        }
    }

    #[test]
    fn distancia_entre_puntos_es_la_euclidea_en_metros() {
        let matrix = matriz_simple(20, 20);

        // (0,0) a (3,4) -> triangulo 3-4-5
        let d = calculate_distance_between_points(&(0, 0), &(3, 4), &matrix);
        assert!((d - 5.0).abs() < 1e-9); // pequeña diferencia cuentas con floats
    }
}
