use kiddo::KdTree;
use kiddo::SquaredEuclidean;
use kiddo::NearestNeighbour;
use spade::{DelaunayTriangulation, FloatTriangulation, HasPosition, Point2, PositionInTriangulation, Triangulation};
use crate::structs::depth_matrix::DepthMatrix;
use rand::RngExt;

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
    let (new_points, new_matrix) = reduce_measuring_points(measuring_points, matrix, geotiff, 75);

    match method {
        InterpolationMethod::Idw     => interpolation_idw_kdtrees(&new_points, &new_matrix, geotiff),
        InterpolationMethod::Kriging => interpolation_kriging(&new_points, &new_matrix, geotiff),
        InterpolationMethod::Tin     => interpolation_tin(&new_points, &new_matrix, geotiff),
    }
}

// ------------------------------------------------------------
//  Reducción de puntos medidos por celdas
//
//  Divide la grilla en celdas de `cell_size` × `cell_size` píxeles.
//  Por cada celda:
//    1. Recopila en orden temporal los puntos válidos que caen dentro.
//    2. Los separa en tramos usando un umbral de distancia automático:
//       si la distancia entre dos puntos consecutivos supera 3× la
//       mediana del espaciado global, se considera una pasada nueva.
//    3. Por cada tramo, toma el punto del medio como posición
//       representativa y calcula el promedio ponderado por distancia
//       a ese punto.
//    4. Genera un punto en new_points por cada tramo encontrado.
//
//  Retorna (new_points, new_matrix)
// ------------------------------------------------------------

/// Calcula la mediana del espaciado entre puntos consecutivos del recorrido.
/// Se usa como referencia para detectar saltos entre pasadas distintas.
fn median_consecutive_spacing(measuring_points: &[(usize, usize)]) -> f64 {
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

/// Dado un tramo (slice de puntos en orden temporal), calcula el promedio
/// ponderado por distancia al punto del medio y devuelve (posición, valor).
fn representative_point_for_segment(
    segment: &[(usize, usize)],
    matrix: &[Vec<f64>],
) -> (usize, usize, f64) {
    // Elegimos un índice aleatorio dentro del tramo en vez del centro fijo,
    // para que los puntos representativos de piernas adyacentes no queden
    // alineados y no formen líneas visibles en la interpolación.
    let random_index = rand::rng().random_range(0..segment.len());
    let (middle_x, middle_y) = segment[random_index];

    let mut weighted_sum = 0.0_f64;
    let mut weight_total = 0.0_f64;

    for &(px, py) in segment {
        let depth = matrix[py][px];

        let dx   = px as f64 - middle_x as f64;
        let dy   = py as f64 - middle_y as f64;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist == 0.0 {
            // El punto coincide con el punto medio: peso infinito,
            // usamos su valor directo y descartamos el resto.
            weighted_sum = depth;
            weight_total = 1.0;
        } else {
            let weight = 1.0 / dist;
            weighted_sum += depth * weight;
            weight_total += weight;
        }
    }

    (middle_x, middle_y, weighted_sum / weight_total)
}

fn reduce_measuring_points(
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
    let median_spacing  = median_consecutive_spacing(measuring_points);
    let gap_threshold   = median_spacing * 3.0;

    // Número de celdas en cada dimensión
    let n_cells_y = (height + cell_size - 1) / cell_size;
    let n_cells_x = (width  + cell_size - 1) / cell_size;

    for cell_row in 0..n_cells_y {
        for cell_col in 0..n_cells_x {
            // Límites de la celda (clipeados al borde del raster)
            let y0 = cell_row * cell_size;
            let x0 = cell_col * cell_size;
            let y1 = (y0 + cell_size).min(height);
            let x1 = (x0 + cell_size).min(width);

            // Recopilar en orden temporal los puntos válidos de esta celda.
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

            // Separar los puntos en tramos: cada vez que la distancia entre
            // dos puntos consecutivos supera gap_threshold, empieza un tramo nuevo.
            let mut current_segment: Vec<(usize, usize)> = vec![points_in_cell[0]];

            for consecutive_pair in points_in_cell.windows(2) {
                let (prev_x, prev_y) = consecutive_pair[0];
                let (next_x, next_y) = consecutive_pair[1];

                let dx           = next_x as f64 - prev_x as f64;
                let dy           = next_y as f64 - prev_y as f64;
                let gap_distance = (dx * dx + dy * dy).sqrt();

                if gap_distance > gap_threshold {
                    // Salto grande: el tramo actual termina aquí, lo procesamos
                    // y empezamos uno nuevo con el punto siguiente.
                    let (rep_x, rep_y, rep_depth) =
                        representative_point_for_segment(&current_segment, matrix);
                    new_matrix[rep_y][rep_x] = rep_depth;
                    new_points.push((rep_x, rep_y));

                    current_segment = vec![*consecutive_pair.last().unwrap()];
                } else {
                    // Distancia normal: el punto pertenece al mismo tramo.
                    current_segment.push((next_x, next_y));
                }
            }

            // Procesar el último tramo (o el único, si no hubo saltos).
            let (rep_x, rep_y, rep_depth) =
                representative_point_for_segment(&current_segment, matrix);
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

    for j in 0..geotiff.height {
        for i in 0..geotiff.width {
            if geotiff.data[j][i] == no_data {
                result[j][i] = no_data;
                continue;
            }
            if matrix[j][i] != 0.0 {
                result[j][i] = matrix[j][i];
                continue;
            }

            let query = Point2::new(i as f64, j as f64);
            
            match triangulation.locate(query) {
                PositionInTriangulation::OutsideOfConvexHull(_) => {
            
                    result[j][i] = no_data;

                }
                _ => {
                    if let Some(z) = triangulation
                        .natural_neighbor()
                        .interpolate(|v| v.data().depth, query)
                    {

                        result[j][i] = z;
                    } 
                }
            }
        }
    }

    result
}

