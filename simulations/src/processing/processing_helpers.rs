/*Archivo para funciones que se comparten en dos o mas modulos de processing */
use crate::DepthMatrix;

/// Verifica si el valor es valido segun la matriz de profundidad (no es no_data y es finito)
pub fn is_valid(val: f64, matrix: &DepthMatrix) -> bool {
    match matrix.no_data {
        Some(nd) => val != nd,
        None => val.is_finite(),
    }
}