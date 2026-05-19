use crate::structs::depth_matrix::DepthMatrix;
use crate::processing::measuring::calculate_distance_between_points;

pub fn interpolacion_jullen_theorem(path: &Vec<(usize, usize)>, measures: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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

            for point in path {
                // ... (Dentro de tu bucle 'for point in path')
                let current_distance = calculate_distance_between_points(point, &(column, row));

                // Control estricto de coincidencia exacta
                if current_distance == 0.0 {
                    interpolation[row][column] = measures[point.1][point.0];
                    exact_match = true;
                    break;
                }

                // PARAMETROS DE SUAVIZADO:
                let p = 4.0;               // Exponente IDW (2.0 da transiciones mucho más suaves que 1.0)
                let smoothing = 1.0;       // Factor alfa. Sube este valor para difuminar más las manchas de color

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