#[derive(serde::Serialize)]
pub struct StudentProjectResponse {
    #[serde(flatten)]
    pub project: crate::db::queries::proyects::AdminProjectView, 
    pub attempts_spent: i64,
    pub coordinates: GeoCorners,
    pub maptiler_api_key: String,
}

#[derive(serde::Serialize)]
pub struct GeoCorners {
    pub sup_izq: (f64, f64),
    pub sup_der: (f64, f64),
    pub inf_izq: (f64, f64),
    pub inf_der: (f64, f64),
    pub centro: (f64, f64),
}