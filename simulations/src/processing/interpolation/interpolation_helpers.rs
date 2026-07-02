/*Archivo para funciones que se comparten unicamente en el modulo de interpolacion */

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