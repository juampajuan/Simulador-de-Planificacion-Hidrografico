/// Respuesta del endpoint `/student_project`: los datos del proyecto (aplanados en el JSON),
/// los intentos ya gastados por el alumno, las coordenadas geográficas del geotiff y la
/// API key de MapTiler para dibujar el mapa de fondo.
#[derive(serde::Serialize)]
pub struct StudentProjectResponse {
    #[serde(flatten)]
    pub project: crate::db::queries::proyects::AdminProjectView,
    pub attempts_spent: i64,
    pub coordinates: GeoCorners,
    pub maptiler_api_key: String,
}

/// Coordenadas (lat, lon) de las cuatro esquinas del geotiff y su centro,
/// usadas por el cliente para centrar y ajustar el zoom del mapa de fondo.
#[derive(serde::Serialize)]
pub struct GeoCorners {
    pub sup_izq: (f64, f64),
    pub sup_der: (f64, f64),
    pub inf_izq: (f64, f64),
    pub inf_der: (f64, f64),
    pub centro: (f64, f64),
}
