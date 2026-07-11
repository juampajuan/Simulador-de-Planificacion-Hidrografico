#[derive(Debug)]
pub enum InterpolationMethod {
    Idw,
    Kriging,
    Tin,
    GdalTin,
}
