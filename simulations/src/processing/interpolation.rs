use kiddo::KdTree;
use kiddo::SquaredEuclidean;
use kiddo::NearestNeighbour;
use spade::{DelaunayTriangulation, FloatTriangulation, HasPosition, Point2, PositionInTriangulation, Triangulation};
use crate::structs::depth_matrix::DepthMatrix;

// ------------------------------------------------------------
//  Tipos y enum público
// ------------------------------------------------------------

#[allow(dead_code)]
pub enum InterpolationMethod {
    Idw,
    Kriging,
    Tin,
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
        InterpolationMethod::Tin     => interpolation_tin(measuring_points, matrix, geotiff),
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

        let weight = 1.0 / dist.powf(1.0);
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
// ------------------------------------------------------------
//  TIN — Triangulated Irregular Network
//
//  Estrategia:
//  1. Construir una triangulación de Delaunay con los puntos medidos.
//  2. Para cada píxel dentro del convex hull → interpolación baricéntrica
//     usando FloatTriangulation::barycentric_interpolation de spade.
//  3. Para píxeles fuera del convex hull → fallback IDW con los 4
//     vecinos más cercanos (evita dejar zonas en negro en los bordes).
// ------------------------------------------------------------

/// Vértice TIN: posición (x, y) en píxeles + profundidad z almacenada
/// como dato adjunto.
#[derive(Clone, Copy, Debug)]
struct TinVertex {
    position: Point2<f64>,
    depth:    f64,
}

impl HasPosition for TinVertex {
    type Scalar = f64;
    fn position(&self) -> Point2<f64> {
        self.position
    }
}

use std::thread;

// ... (Las importaciones y la inicialización de TIN se mantienen igual) ...

pub fn interpolation_tin(
    measuring_points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    geotiff: &DepthMatrix,
) -> Vec<Vec<f64>> {
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);
    let mut result = vec![vec![0.0_f64; geotiff.width]; geotiff.height];

    let (kdtree, depth_values, indices) = build_kdtree(measuring_points, matrix, no_data);

    if depth_values.is_empty() {
        return result;
    }

    let mut triangulation: DelaunayTriangulation<TinVertex> = DelaunayTriangulation::new();
    for (i, &idx) in indices.iter().enumerate() {
        let point = measuring_points[idx];
        let vertex = TinVertex {
            position: Point2::new(point.0 as f64, point.1 as f64),
            depth: depth_values[i],
        };
        let _ = triangulation.insert(vertex);
    }

    if triangulation.num_inner_faces() == 0 {
        return interpolation_kriging(measuring_points, matrix, geotiff);
    }

    let kriging_neighbors = 16; 

    // --- 3. Interpolar cada píxel con std::thread ---------------------
    
    // Obtenemos la cantidad de hilos lógicos disponibles en la CPU (o 4 por defecto)
    let num_threads = thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    
    // Calculamos cuántas filas procesará cada hilo
    let rows_per_thread = (geotiff.height + num_threads - 1) / num_threads;

    // Abrimos un "scope" para garantizar que los hilos mueran antes de que retorne la función
    thread::scope(|s| {
        
        // .chunks_mut() es la clave: divide la matriz en bloques independientes 
        // de filas para que cada hilo pueda escribir sin usar Mutex.
        for (thread_id, chunk) in result.chunks_mut(rows_per_thread).enumerate() {
            
            // Calculamos en qué fila global arranca este bloque
            let start_row = thread_id * rows_per_thread;

            // Prestamos las variables al scope (necesario para el closure del hilo)
            let kdtree_ref = &kdtree;
            let depth_values_ref = &depth_values;
            let indices_ref = &indices;
            let triangulation_ref = &triangulation;
            
            // Creamos el Builder con un nombre, como en la diapositiva
            let builder = thread::Builder::new()
                .name(format!("kriging_worker_{}", thread_id));

            // Usamos spawn_scoped en lugar de spawn regular
            builder.spawn_scoped(s, move || {
                
                // Iteramos sobre el bloque de filas que le tocó a este hilo
                for (local_j, row) in chunk.iter_mut().enumerate() {
                    let global_j = start_row + local_j; // Reconstruimos la coordenada 'Y' real

                    for i in 0..geotiff.width {
                        if geotiff.data[global_j][i] == no_data {
                            row[i] = no_data;
                            continue;
                        }
                        if matrix[global_j][i] != 0.0 {
                            row[i] = matrix[global_j][i];
                            continue;
                        }

                        let query = Point2::new(i as f64, global_j as f64);

                        match triangulation_ref.locate(query) {
                            PositionInTriangulation::OutsideOfConvexHull(_)
                            | PositionInTriangulation::NoTriangulation => {
                                row[i] = kriging_fallback(
                                    kdtree_ref, depth_values_ref, indices_ref, measuring_points, 
                                    i as f64, global_j as f64, kriging_neighbors
                                );
                            }
                            _ => {
                                if let Some(z) = triangulation_ref
                                    .natural_neighbor()
                                    .interpolate(|v| v.data().depth, query) {
                                    row[i] = z;
                                } else {
                                    row[i] = kriging_fallback(
                                        kdtree_ref, depth_values_ref, indices_ref, measuring_points, 
                                        i as f64, global_j as f64, kriging_neighbors
                                    );
                                }
                            }
                        }
                    }
                }
            }).unwrap();
        }
    });

    result

    
}

// --- Helpers -----------------------------------------------

/// Kriging local con los k vecinos más cercanos.
/// Reemplaza al idw_fallback para suavizar los bordes y evitar el "efecto estrella".
fn kriging_fallback(
    kdtree: &KdTree<f64, 2>,
    values: &[f64],
    indices: &[usize],
    measuring_points: &[(usize, usize)],
    x: f64,
    y: f64,
    k: usize,
) -> f64 {
    let neighbours = kdtree.nearest_n::<SquaredEuclidean>(&[x, y], k);
    
    if neighbours.is_empty() {
        return 0.0;
    }
    
    // Si caemos exactamente sobre un punto medido, evitamos invertir la matriz
    if neighbours[0].distance == 0.0 {
        return values[neighbours[0].item as usize];
    }
    
    compute_kriging(&neighbours, values, indices, measuring_points)
}