#[derive(Clone)]
pub struct DepthMatrix {
    pub data: Vec<Vec<f64>>,
    pub width: usize,
    pub height: usize,
    pub no_data: Option<f64>,
    pub size_x: f64,
    pub size_y: f64,
    /// Los 6 coeficientes del geotransform del GeoTIFF original:
    /// [origen_x, ancho_pixel_x, rotación_x, origen_y, rotación_y, alto_pixel_y]
    /// Necesario para que los rasters generados por gdal_grid queden
    /// alineados/orientados igual que el GeoTIFF original.
    pub geo_transform: [f64; 6],
    /// Proyección del GeoTIFF original en formato WKT.
    pub projection: String,
}