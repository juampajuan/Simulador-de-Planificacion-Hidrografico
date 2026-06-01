use kiddo::KdTree;
use kiddo::SquaredEuclidean;
use kiddo::NearestNeighbour;
use crate::structs::depth_matrix::DepthMatrix;

// ------------------------------------------------------------
//  Tipos y enum público
// ------------------------------------------------------------

#[allow(dead_code)]
pub enum InterpolationMethod {
    Idw,
    Kriging,
}

pub fn interpolate(
    method: InterpolationMethod,
    measuring_points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    geotiff: &DepthMatrix,
) -> Vec<Vec<f64>> {
    match method {
        InterpolationMethod::Idw     => interpolation_idw_kdtrees(measuring_points, matrix, geotiff),
        InterpolationMethod::Kriging => interpolation_kriging(measuring_points, matrix, geotiff),
    }
}

// ------------------------------------------------------------
//  KD-Tree compartido
//  Retorna:
//    - kdtree  : árbol de búsqueda; item = índice en values[]
//    - values  : profundidad de cada punto válido
//    - indices : posición original en measuring_points[] (necesario para Kriging)
// ------------------------------------------------------------

fn build_kdtree(
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

// ------------------------------------------------------------
//  IDW
// ------------------------------------------------------------

fn compute_idw(
    neighbours: &Vec<NearestNeighbour<f64, u64>>,
    values: &[f64],
) -> f64 {
    let mut weighted_sum = 0.0;
    let mut weight_total = 0.0;

    for neighbour in neighbours {
        let dist = neighbour.distance.sqrt();
        let val  = values[neighbour.item as usize];

        if dist == 0.0 {
            return val;
        }

        let weight = 1.0 / dist.powf(2.0);
        weighted_sum += val * weight;
        weight_total += weight;
    }

    if weight_total > 0.0 { weighted_sum / weight_total } else { 0.0 }
}

pub fn interpolation_idw_kdtrees(
    measuring_points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    geotiff: &DepthMatrix,
) -> Vec<Vec<f64>> {
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);
    let mut result = vec![vec![0.0; geotiff.width]; geotiff.height];

    let (kdtree, values, _indices) = build_kdtree(measuring_points, matrix, no_data);

    if values.is_empty() {
        return result;
    }

    for j in 0..matrix.len() {
        for i in 0..matrix[0].len() {
            if geotiff.data[j][i] == no_data {
                result[j][i] = no_data;
                continue;
            }
            if matrix[j][i] != 0.0 {
                result[j][i] = matrix[j][i];
                continue;
            }

            let neighbours = kdtree.nearest_n::<SquaredEuclidean>(
                &[i as f64, j as f64],
                8,
            );

            result[j][i] = compute_idw(&neighbours, &values);
        }
    }

    result
}

// ------------------------------------------------------------
//  Kriging
// ------------------------------------------------------------

//  Eliminación gaussiana con pivoteo parcial
//  Resuelve el sistema  A · x = b
//  Retorna Some(x) o None si la matriz es singular
fn gaussian_elimination(mat_a: &[Vec<f64>], vec_b: &[f64]) -> Option<Vec<f64>> {
    

    let n = vec_b.len();

    // Copia aumentada [A | b]
    let mut aug = vec![vec![0.0f64; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = mat_a[i][j];
        }
        aug[i][n] = vec_b[i];
    }

    for col in 0..n {
        // Buscar fila con mayor valor absoluto en esta columna (pivoteo parcial)
        let mut pivot_row = col;
        for row in (col + 1)..n {
            if aug[row][col].abs() > aug[pivot_row][col].abs() {
                pivot_row = row;
            }
        }

        aug.swap(col, pivot_row);

        let pivot = aug[col][col];
        if pivot.abs() < 1e-10 {
            return None; // Matriz singular
        }

        // Normalizar fila del pivote
        #[allow(clippy::needless_range_loop)]
        for j in col..=n {
            aug[col][j] /= pivot;
        }

        // Eliminar la columna en las demás filas
        for row in 0..n {
            if row == col { continue; }
            let factor = aug[row][col];
            #[allow(clippy::needless_range_loop)]
            for j in col..=n {
                aug[row][j] -= factor * aug[col][j];
            }
        }
    }

    // La solución queda en la última columna
    let mut result = vec![0.0f64; n];
    for i in 0..n {
        result[i] = aug[i][n];
    }
    Some(result)
}

fn build_semivariogram_system(
    neighbours: &[NearestNeighbour<f64, u64>],
    indices: &[usize],
    measuring_points: &[(usize, usize)],
) -> (Vec<Vec<f64>>, Vec<f64>) {
    let n = neighbours.len();

    // Semivariograma lineal: γ(d²) = √d²  →  γ(d) = d
    let semivariograma = |dist_sq: f64| dist_sq.sqrt();

    // Matriz A de tamaño (n+1) x (n+1), inicializada en 1.0
    let mut mat_a = vec![vec![1.0f64; n + 1]; n + 1];
    mat_a[n][n] = 0.0;

    // Vector b de tamaño (n+1), inicializado en 1.0
    let mut vec_b = vec![1.0f64; n + 1];

    for row in 0..n {
        let p_row      = measuring_points[indices[neighbours[row].item as usize]];
        let coords_row = [p_row.0 as f64, p_row.1 as f64];

        // b[row] = γ(distancia del vecino al punto query)
        vec_b[row] = semivariograma(neighbours[row].distance);

        for col in 0..n {
            mat_a[row][col] = if row == col {
                0.0 // distancia a sí mismo
            } else {
                let p_col   = measuring_points[indices[neighbours[col].item as usize]];
                let dist_sq = (coords_row[0] - p_col.0 as f64).powi(2)
                            + (coords_row[1] - p_col.1 as f64).powi(2);
                semivariograma(dist_sq)
            };
        }
    }

    (mat_a, vec_b)
}

fn compute_kriging(
    neighbours: &[NearestNeighbour<f64, u64>],
    values: &[f64],
    indices: &[usize],
    measuring_points: &[(usize, usize)],
) -> f64 {
    let n = neighbours.len();
    let (mat_a, vec_b) = build_semivariogram_system(neighbours, indices, measuring_points);

    if let Some(weights) = gaussian_elimination(&mat_a, &vec_b) {
        // Estimación final: Σ λₖ · z(xₖ)
        (0..n)
            .map(|k| weights[k] * values[neighbours[k].item as usize])
            .sum()
    } else {
        // Fallback: promedio simple si la matriz es singular (puntos duplicados)
        let sum: f64 = (0..n)
            .map(|k| values[neighbours[k].item as usize])
            .sum();
        sum / n as f64
    }
}

pub fn interpolation_kriging(
    measuring_points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    geotiff: &DepthMatrix,
) -> Vec<Vec<f64>> {
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);
    let mut result = vec![vec![0.0; geotiff.width]; geotiff.height];

    let (kdtree, values, indices) = build_kdtree(measuring_points, matrix, no_data);

    if values.is_empty() {
        return result;
    }

    for j in 0..matrix.len() {
        for i in 0..matrix[0].len() {
            if geotiff.data[j][i] == no_data {
                result[j][i] = no_data;
                continue;
            }
            if matrix[j][i] != 0.0 {
                result[j][i] = matrix[j][i];
                continue;
            }

            let neighbours = kdtree.nearest_n::<SquaredEuclidean>(
                &[i as f64, j as f64],
                12,
            );

            // Coincidencia exacta: usar el valor directo
            if neighbours[0].distance == 0.0 {
                result[j][i] = values[neighbours[0].item as usize];
                continue;
            }

            result[j][i] = compute_kriging(&neighbours, &values, &indices, measuring_points);
        }
    }

    result
}