use crate::structs::depth_matrix::DepthMatrix;
use crate::processing::measuring::calculate_distance_between_points;

pub fn interpolacion_jullen_theorem(measuring_points: &Vec<(usize, usize)>, measures: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    if measures.is_empty() {
        return vec![];
    }

    let rows = measures.len();
    let cols = 1635; // Tu ancho objetivo fijo

    println!("Filas (Alto): {}", rows);
    println!("Columnas (Ancho): {}", cols);

    // Inicializamos la matriz de forma estándar: [rows][columns]
    let mut interpolation = vec![vec![0.0; cols]; rows];

    for row in 0..rows {
        for column in 0..cols {
            let mut total_weight = 0.0;
            let mut weighted_sum = 0.0;
            let mut exact_match = false;

            for point in measuring_points {
                // ... (Dentro de tu bucle 'for point in path')
                let current_distance = calculate_distance_between_points(point, &(column, row));

                // Control estricto de coincidencia exacta
                if current_distance == 0.0 {
                    interpolation[row][column] = measures[point.1][point.0];
                    exact_match = true;
                    break;
                }

                // PARAMETROS DE SUAVIZADO:

                let p = 2.0;               // Exponente IDW (2.0 da transiciones mucho más suaves que 1.0)
                let smoothing = 5.0;       // Factor alfa. Sube este valor para difuminar más las manchas de color


                // Nueva fórmula matemática suavizada
                let weight = 1.0 / (current_distance.powf(p) + smoothing); 

                weighted_sum += measures[point.1][point.0] * weight;
                total_weight += weight;
                // ...
            }

            // Si no coincidió exactamente con un punto, calculamos el promedio ponderado
            if !exact_match {
                if total_weight > 0.0 {
                    interpolation[row][column] = weighted_sum / total_weight;
                } else {
                    interpolation[row][column] = 0.0;
                }
            }
        }
    }

    interpolation
}

pub fn interpolacion_reintento(
    measuring_points: &Vec<(usize, usize)>,
    matrix: &Vec<Vec<f64>>,
    geotiff: &DepthMatrix,
) -> Vec<Vec<f64>> {
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);
    let mut result = vec![vec![0.0; matrix[0].len()]; matrix.len()];

    for j in 0..matrix.len() {
        for i in 0..matrix[0].len() {
            if geotiff.data[j][i] == no_data {
                result[j][i] = no_data;
            } else if matrix[j][i] != 0.0 {
                result[j][i] = matrix[j][i];
            } else {
                let vecinos = pick_by_quadrant(measuring_points, matrix, (i, j));

                let mut weighted_sum = 0.0;
                let mut weight_total = 0.0;
                for (dist, val) in &vecinos {
                    if *dist == 0.0 { result[j][i] = *val; break; }
                    let weight = 1.0 / dist.powf(2.0);
                    weighted_sum += val * weight;
                    weight_total += weight;
                }
                if weight_total > 0.0 {
                    result[j][i] = weighted_sum / weight_total;
                }
            }
        }
    }
    result
}

fn calculate_estimated_value(
    measures: &Vec<(usize, usize)>,
    matrix: &Vec<Vec<f64>>,
    current_point: (usize, usize),
) -> f64 {
    let mut weighted_sum = 0.0;
    let mut weight_total = 0.0;

    for point in measures {
        let dx = current_point.0 as f64 - point.0 as f64;
        let dy = current_point.1 as f64 - point.1 as f64;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist == 0.0 {
            return matrix[point.1][point.0];
        }

        let weight = 1.0 / dist;
        weighted_sum += matrix[point.1][point.0] * weight;
        weight_total += weight;
    }

    weighted_sum / weight_total
}

fn pick_by_quadrant(
    measures: &Vec<(usize, usize)>,
    matrix: &Vec<Vec<f64>>,
    current_point: (usize, usize),
) -> Vec<(f64, f64)> {
    // NE, NW, SE, SW
    let mut quadrants: [Option<(f64, f64)>; 4] = [None; 4];

    for point in measures {
        let dx = point.0 as f64 - current_point.0 as f64;
        let dy = point.1 as f64 - current_point.1 as f64;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist == 0.0 {
            return vec![(0.0, matrix[point.1][point.0])];
        }

        let val = matrix[point.1][point.0];
        let idx = match (dx >= 0.0, dy >= 0.0) {
            (true,  true)  => 0, // SE
            (false, true)  => 1, // SW
            (true,  false) => 2, // NE
            (false, false) => 3, // NW
        };

        // Guardamos el más cercano de cada cuadrante
        if quadrants[idx].is_none() || dist < quadrants[idx].unwrap().0 {
            quadrants[idx] = Some((dist, val));
        }
    }

    quadrants.iter().filter_map(|q| *q).collect()
}