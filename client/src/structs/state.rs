use serde::Serialize;
use common::{StudentMeasuringParameters, PathParameters, Transport, EcosondaMode};
use crate::UseStateHandle;

#[derive(Default, Clone, PartialEq, Serialize)]
pub struct PathState {
    pub separacion: String,
    pub azimut: String,
    pub gnss_type: String,
}

#[derive(Clone, PartialEq, Serialize)]
pub struct EchoState {
    pub transport: Transport,
    pub speed: String,
    pub max_limit: String,
    pub min_limit: String,
    pub pulse_repetition_interval: String,
    pub transmited_potency: String,
    pub gain: String,
    pub sound_speed: String,
    pub umbral: String,
    pub uses_mareograph: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,
    pub uses_high_frecuency: bool,
    pub mode: EcosondaMode,
}

impl EchoState {
    pub fn new() -> Self {
        Self {
            transport: Transport::Ship,
            speed: "1.0".to_string(),
            max_limit: "100".to_string(),
            min_limit: "1".to_string(),
            pulse_repetition_interval: "20".to_string(),
            transmited_potency: "50".to_string(),
            gain: "12".to_string(),
            sound_speed: "1450".to_string(),
            umbral: "10".to_string(),
            uses_mareograph: false,
            uses_sound_profiler: false,
            uses_inertial_sensor: false,
            uses_high_frecuency: true,
            mode: EcosondaMode::Monohaz,
        }
    }
}

#[derive(Serialize)]
pub struct FullSimulationRequest {
    pub echo_parameters: StudentMeasuringParameters,
    pub path_parameters: PathParameters,
}

#[derive(Serialize)]
pub struct CreatePathRequest {
    pub path_parameters: PathParameters,
}

#[derive(Clone, PartialEq)]
pub struct SimulationUiState {
    pub mensaje: UseStateHandle<String>,
    pub image_url: UseStateHandle<Option<String>>,
    pub loading: UseStateHandle<bool>,
}