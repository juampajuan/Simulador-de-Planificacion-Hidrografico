use yew::prelude::*; 
use serde::Serialize;
use crate::blob_client::send_blob_request;
use crate::parser::{parse_path_parameters, parse_echosounder_parameters};

use common::{
    StudentMeasuringParameters, 
    Boat, 
    EcosondaMode,
    PathParameters
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
    pub uses_mareograph: bool,
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
            uses_mareograph: false,
            uses_sound_profiler: true,
            uses_inertial_sensor: false,
            uses_high_frecuency: true,
            mode: EcosondaMode::Monohaz,
        }
    }
}

#[derive(serde::Serialize)]
pub struct FullSimulationRequest {
    pub echo_parameters: StudentMeasuringParameters,
    pub path_parameters: PathParameters,
}

pub fn trigger_path_generation(
    state: PathState, 
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
    loading: UseStateHandle<bool>
) {
    if state.separacion.is_empty() || state.azimut.is_empty() { return; }

    let params = match parse_path_parameters(&state) {
        Ok(p) => p,
        Err(err_msg) => {
            mensaje.set(err_msg);
            return;
        }
    };

    mensaje.set("Generando recorrido...".to_string());
    loading.set(true);

    send_blob_request("http://localhost:3000/api/v1/create_path", &params, mensaje, image_url, loading);
}

pub fn run_simulation(
    echo_state: EchoState, 
    path_state: PathState,
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
    loading: UseStateHandle<bool>
) {
    let echo_params = match parse_echosounder_parameters(&echo_state) {
        Ok(p) => p,
        Err(err) => { mensaje.set(err); return; }
    };

    let path_params = match parse_path_parameters(&path_state) {
        Ok(p) => p,
        Err(err) => { mensaje.set(err); return; }
    };

    mensaje.set("Simulando medición...".to_string());
    loading.set(true);
    
    let boat_speed = 1.0; 
    let simulation_params = FullSimulationRequest {
        echo_parameters: StudentMeasuringParameters {
            uses_mareograph: echo_state.uses_mareograph,
            uses_sound_profiler: echo_state.uses_sound_profiler,
            uses_inertial_sensor: echo_state.uses_inertial_sensor,
            boat: match echo_state.boat.as_str() {
                "Y" => Boat::Y { speed: boat_speed },
                _ => Boat::W { speed: boat_speed },
            },
            echo_sounder_parameters: echo_params,
        },
        path_parameters: path_params, // por si el cache del back no ofrece el path.
    };

    send_blob_request("http://localhost:3000/api/v1/run_simulation", &simulation_params, mensaje, image_url, loading);
}