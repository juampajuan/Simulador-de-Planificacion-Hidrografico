use kiddo::{NearestNeighbour, SquaredEuclidean};

use crate::{processing::interpolation::helpers::build_kdtree, structs::depth_matrix::DepthMatrix};

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

        let weight = 1.0 / dist;
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
