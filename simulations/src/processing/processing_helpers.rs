/// Verifica si el valor es valido segun la matriz de profundidad (no es no_data y es finito)
pub fn is_valid(val: f64, no_data: Option<f64>) -> bool {
    match no_data {
        Some(nd) => val != nd,
        None => val.is_finite(),
    }
}
