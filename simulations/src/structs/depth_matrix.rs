#[derive(Clone)]
pub struct DepthMatrix {
    pub data: Vec<Vec<f64>>,
    pub width: usize,
    pub height: usize,
    pub no_data: Option<f64>,
    pub size_x: f64,
    pub size_y: f64,
    pub geo_transform: [f64; 6],
    pub projection: String,
}
