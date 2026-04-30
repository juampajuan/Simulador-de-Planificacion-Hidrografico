pub struct DepthMatrix {
    pub data: Vec<Vec<f64>>,
    pub width: usize,
    pub heigth: usize,
    pub no_data: Option<f64>,
}