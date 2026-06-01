use yew::prelude::*; 
use serde::Serialize;
use crate::blob_client::send_blob_request;

use common::{
    StudentMeasuringParameters, 
    EchosounderParameters, 
    Boat, 
    PathParameters, 
    GnssType,
    EcosondaMode,
};

#[derive(Default, Clone, PartialEq, Serialize)]
pub struct PathState {
    pub separacion: String,
    pub azimut: String,
    pub gnss_type: String,
}

#[derive(Clone, PartialEq, Serialize)]
pub struct EchoState {
    pub boat: String,
    pub max_limit: String,
    pub min_limit: String,
    pub pulse_repetition_interval: String,
    pub pulse_length: String,
    pub transmited_potency: String,
    pub gain: String,
    pub echosounder_velocity: String,
    pub umbral: String,
    pub uses_mathegapher: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,
    pub uses_high_frecuency: bool,
    pub mode: EcosondaMode,
}

impl EchoState {
    pub fn new() -> Self {
        Self {
            boat: "W".to_string(),
            max_limit: "100".to_string(),
            min_limit: "0".to_string(),
            pulse_repetition_interval: "100".to_string(),
            pulse_length: "1".to_string(),
            transmited_potency: "220".to_string(),
            gain: "0".to_string(),
            echosounder_velocity: "1450".to_string(),
            umbral: "0.1".to_string(),
            uses_mathegapher: false,
            uses_sound_profiler: true,
            uses_inertial_sensor: false,
            uses_high_frecuency: true,
            mode: EcosondaMode::Monohaz,
        }
    }
}

pub fn trigger_path_generation(
    state: PathState, 
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
    loading: UseStateHandle<bool>
) {
    if state.separacion.is_empty() || state.azimut.is_empty() { return; }
    
    mensaje.set("Generando recorrido...".to_string());
    loading.set(true);

    let params = PathParameters {
        separacion: state.separacion.parse().unwrap_or(0.0),
        azimut: state.azimut.parse().unwrap_or(0.0),
        gnss_type: match state.gnss_type.as_str() {
            "Fase" => GnssType::PhaseCorrection,
            "DGPS" => GnssType::DGPSCorrection,
            _ => GnssType::NoCorrection,
        },
    };

    send_blob_request("http://localhost:3000/api/v1/create_path", &params, mensaje, image_url, loading);
}

pub fn run_simulation(
    state: EchoState, 
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
    loading: UseStateHandle<bool>
) {
    mensaje.set("Simulando medición...".to_string());
    loading.set(true);
    
    let boat_speed = 1.0; 

    let params = StudentMeasuringParameters {
        uses_mathegapher: state.uses_mathegapher,
        uses_sound_profiler: state.uses_sound_profiler,
        uses_inertial_sensor: state.uses_inertial_sensor,
        boat: match state.boat.as_str() {
            "Y" => Boat::Y { speed: boat_speed },
            _ => Boat::W { speed: boat_speed },
        },
        echo_sounder_parameters: EchosounderParameters {
            mode: state.mode,
            angle: 0.0,
            absortion_coefficient: 0.0,
            max_limit: state.max_limit.parse().unwrap_or(0.0),
            min_limit: state.min_limit.parse().unwrap_or(0.0),
            pulse_repetition_interval: state.pulse_repetition_interval.parse().unwrap_or(0.0),
            pulse_length: state.pulse_length.parse().unwrap_or(0),
            uses_high_frecuency: state.uses_high_frecuency,
            transmited_potency: state.transmited_potency.parse().unwrap_or(0.0),
            gain: state.gain.parse().unwrap_or(0.0),
            echosounder_velocity: state.echosounder_velocity.parse().unwrap_or(0),
            threshold: state.umbral.parse().unwrap_or(0.0),
        },
    };

    send_blob_request("http://localhost:3000/api/v1/run_simulation", &params, mensaje, image_url, loading);
}