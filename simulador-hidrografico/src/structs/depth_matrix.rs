pub struct DepthMatrix {
    pub data: Vec<Vec<f64>>,
    pub width: usize,
    pub height: usize,
    pub no_data: Option<f64>,
}