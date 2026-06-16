use kiddo::KdTree;
use crate::structs::depth_matrix::DepthMatrix;
/// Crea una nueva matriz con 0 en las celdas y le inserta las profundidades medidas
/// No agrega aquellos puntos que quedaron en None
pub fn create_matrix_with_measurments_and_eliminate_none_points (measurements: &Vec<((usize, usize), Option<f64>)>, geotiff: &DepthMatrix) -> (Vec<(usize, usize)>, Vec<Vec<f64>>) {
    let mut matrix_with_measured_points: Vec<Vec<f64>> = vec![vec![0.0f64; geotiff.width]; geotiff.height];
    let mut points_validos: Vec<(usize, usize)> = Vec::new();

    for (punto, z_obs) in measurements {
        if let Some(z) = z_obs {
            matrix_with_measured_points[punto.1][punto.0] = *z;
            points_validos.push(*punto);
        }
    }

    (points_validos, matrix_with_measured_points)
}

/// Calcula la mediana del espaciado entre puntos consecutivos del recorrido.
/// Se usa como referencia para detectar saltos entre pasadas distintas.
#[allow(dead_code)]
pub fn median_consecutive_spacing(measuring_points: &[(usize, usize)]) -> f64 {
    if measuring_points.len() < 2 {
        return f64::MAX;
    }

    let mut spacings: Vec<f64> = measuring_points
        .windows(2)
        .map(|pair| {
            let dx = pair[1].0 as f64 - pair[0].0 as f64;
            let dy = pair[1].1 as f64 - pair[0].1 as f64;
            (dx * dx + dy * dy).sqrt()
        })
        .collect();

    spacings.sort_by(|a, b| a.partial_cmp(b).unwrap());
    spacings[spacings.len() / 2]
}

/// Dado una pierna, elige un índice basado en la posición de la celda y el número de tramo dentro de ella.
///
/// Por qué no el centro fijo: si dos piernas pasan por la misma celda,
/// los centros de los tramos quedan alineados, formando una línea visible en la interpolación.
///
/// Por qué no aleatorio puro: cada corrida daría una interpolación
/// diferente aunque los parámetros sean los mismos.
///
/// Solución: Celdas adyacentes tienen semillas distintas => índices distintos =>
/// no se alinean. Misma celda siempre elige el mismo índice => reproducible.
#[allow(dead_code)]
pub fn representative_point_for_segment(
    segment: &[(usize, usize)],
    matrix: &[Vec<f64>],
    cell_row: usize,
    cell_col: usize,
    segment_index: usize,
) -> (usize, usize, f64) {
    // Hash de la posición de la celda y el tramo
    let seed = (cell_row.wrapping_mul(2654435761))
        ^ (cell_col.wrapping_mul(2246822519))
        ^ (segment_index.wrapping_mul(374761393));
    let chosen_index = seed % segment.len();

    let (middle_x, middle_y) = segment[chosen_index];


    (middle_x, middle_y, matrix[middle_y][middle_x])
}

#[allow(dead_code)]
pub fn reduce_measuring_points(
    measuring_points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    geotiff: &DepthMatrix,
    cell_size: usize,
) -> (Vec<(usize, usize)>, Vec<Vec<f64>>) {
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);
    let height  = geotiff.height;
    let width   = geotiff.width;

    let mut new_matrix: Vec<Vec<f64>> = vec![vec![0.0; width]; height];
    let mut new_points: Vec<(usize, usize)> = Vec::new();

    // Umbral para detectar saltos entre pasadas distintas del zigzag:
    // si dos puntos consecutivos dentro de una celda están más de 3×
    // la mediana del espaciado global, son pasadas distintas.
    let median_spacing = median_consecutive_spacing(measuring_points);
    let gap_threshold  = median_spacing * 3.0;

    let n_cells_y = (height + cell_size - 1) / cell_size;
    let n_cells_x = (width  + cell_size - 1) / cell_size;

    for cell_row in 0..n_cells_y {
        for cell_col in 0..n_cells_x {
            let y0 = cell_row * cell_size;
            let x0 = cell_col * cell_size;
            let y1 = (y0 + cell_size).min(height);
            let x1 = (x0 + cell_size).min(width);

            let points_in_cell: Vec<(usize, usize)> = measuring_points
                .iter()
                .filter(|&&(px, py)| {
                    px >= x0 && px < x1 && py >= y0 && py < y1
                        && matrix[py][px] != 0.0
                        && matrix[py][px] != no_data
                })
                .copied()
                .collect();

            if points_in_cell.is_empty() {
                continue;
            }

            let mut current_segment: Vec<(usize, usize)> = vec![points_in_cell[0]];
            let mut segment_index = 0;

            for consecutive_pair in points_in_cell.windows(2) {
                let (prev_x, prev_y) = consecutive_pair[0];
                let (next_x, next_y) = consecutive_pair[1];

                let dx           = next_x as f64 - prev_x as f64;
                let dy           = next_y as f64 - prev_y as f64;
                let gap_distance = (dx * dx + dy * dy).sqrt();

                if gap_distance > gap_threshold {
                    let (rep_x, rep_y, rep_depth) = representative_point_for_segment(
                        &current_segment, matrix, cell_row, cell_col, segment_index,
                    );
                    new_matrix[rep_y][rep_x] = rep_depth;
                    new_points.push((rep_x, rep_y));

                    current_segment = vec![consecutive_pair[1]];
                    segment_index += 1;
                } else {
                    current_segment.push((next_x, next_y));
                }
            }

            // Último tramo
            let (rep_x, rep_y, rep_depth) = representative_point_for_segment(
                &current_segment, matrix, cell_row, cell_col, segment_index,
            );
            new_matrix[rep_y][rep_x] = rep_depth;
            new_points.push((rep_x, rep_y));
        }
    }

    (new_points, new_matrix)
}

// ------------------------------------------------------------
//  KD-Tree compartido
//  Retorna:
//    - kdtree  : árbol de búsqueda; item = índice en values[]
//    - values  : profundidad de cada punto válido
//    - indices : posición original en measuring_points[]
// ------------------------------------------------------------

pub fn build_kdtree(
    measuring_points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    no_data: f64,
) -> (KdTree<f64, 2>, Vec<f64>, Vec<usize>) {
    let mut kdtree:  KdTree<f64, 2> = KdTree::new();
    let mut values:  Vec<f64>       = Vec::new();
    let mut indices: Vec<usize>     = Vec::new();

    for (idx, point) in measuring_points.iter().enumerate() {
        let depth = matrix[point.1][point.0];
        if depth != 0.0 && depth != no_data {
            kdtree.add(&[point.0 as f64, point.1 as f64], values.len() as u64);
            values.push(depth);
            indices.push(idx);
        }
    }

    (kdtree, values, indices)
}
