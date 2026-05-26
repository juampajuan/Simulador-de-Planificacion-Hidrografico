use crate::structs::gnss_type::GnssType;
pub struct StudentPathParameters{
    pub azimuth_deg: f64, 
    pub separation_meters :f64,
    pub gnss_type: GnssType
}