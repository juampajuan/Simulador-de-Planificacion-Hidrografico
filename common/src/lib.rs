use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum GnssType {
    NoCorrection,
    DGPSCorrection,
    PhaseCorrection
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Transport {
    Ship,
    Boat,
    Launch
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum EcosondaMode {
    Monohaz,       
    Multihaz,       
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PathParameters {
    pub separacion: f64,
    pub azimut: f64,
    pub gnss_type: GnssType,
}

#[derive(Debug, Serialize, Deserialize, Clone,Copy, PartialEq)]
pub struct EchosounderParameters {
    pub mode: EcosondaMode,
    pub angle: f64,
    pub absortion_coefficient: f64,
    pub max_limit: f64,
    pub min_limit: f64,
    pub pulse_repetition_interval: f64, // Hz, en simulation lo convertimos a segundos.
    pub uses_high_frecuency: bool,
    pub transmited_potency: f64,
    pub gain: f64,
    pub threshold: f64,
    pub sound_speed: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct TransportParameters {
    pub transport: Transport,
    pub speed: f64, // m/s
    pub uses_mareograph: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct StudentMeasuringParameters {
    pub echo_sounder_parameters: EchosounderParameters,
    pub transport_parameters: TransportParameters,
}