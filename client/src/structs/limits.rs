use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ConfigLimits {
    pub azimut_min: f64,
    pub azimut_max: f64,
    pub separation_min: f64,
    pub transport_speed_min: f64,
    pub transport_speed_max: f64,
    pub echo_depth_min: f64,
    pub echo_depth_max: f64,
    pub echo_pulse_min: f64,
    pub echo_pulse_max: f64,
    pub echo_umbral_min: f64,
    pub echo_umbral_max: f64,
    pub sound_speed_min: f64,
    pub sound_speed_max: f64,
}

impl Default for ConfigLimits {
    fn default() -> Self {
        Self {
            azimut_min: 0.0,
            azimut_max: 360.0,
            separation_min: 1.0,
            transport_speed_min: 1.0,
            transport_speed_max: 12.0,
            echo_depth_min: 1.0,
            echo_depth_max: 6000.0,
            echo_pulse_min: 1.0,
            echo_pulse_max: 20.0,
            echo_umbral_min: 10.0,
            echo_umbral_max: 90.0,
            sound_speed_min: 1450.0,
            sound_speed_max: 1500.0,
        }
    }
}