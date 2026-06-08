use crate::services::blob_client::send_blob_request;
use crate::parser::{parse_path_parameters, parse_echosounder_parameters, parse_transport_parameters};
use crate::structs::state::{PathState, EchoState, FullSimulationRequest, SimulationUiState, CreatePathRequest};

use common::StudentMeasuringParameters;

pub fn trigger_path_generation(
    state: &PathState, 
    ui: SimulationUiState
) {
    if state.separacion.is_empty() || state.azimut.is_empty() { return; }

    let params = match parse_path_parameters(&state) {
        Ok(p) => p,
        Err(err_msg) => {
            ui.mensaje.set(err_msg);
            return;
        }
    };

    ui.mensaje.set("Generando recorrido...".to_string());
    ui.loading.set(true);

    let request_body = CreatePathRequest {path_parameters: params};

    send_blob_request("http://localhost:3000/api/v1/create_path", &request_body, ui.mensaje, ui.image_url, ui.loading);
}

pub fn run_simulation(echo_state: &EchoState, path_state: &PathState, ui: SimulationUiState) {
    let echo_params = match parse_echosounder_parameters(echo_state) {
        Ok(p) => p,
        Err(err) => { ui.mensaje.set(err); return; }
    };

    let path_params = match parse_path_parameters(path_state) {
        Ok(p) => p,
        Err(err) => { ui.mensaje.set(err); return; }
    };

    let transport_params = match parse_transport_parameters(echo_state) {
        Ok(t) => t,
        Err(e) => { ui.mensaje.set(e); return; }
    };

    ui.mensaje.set("Simulando medición...".to_string());
    ui.loading.set(true);

    let simulation_params = FullSimulationRequest {
        echo_parameters: StudentMeasuringParameters {
            transport_parameters: transport_params,
            echo_sounder_parameters: echo_params,
        },
        path_parameters: path_params,
    };

    send_blob_request("http://localhost:3000/api/v1/run_simulation",&simulation_params, ui.mensaje, ui.image_url, ui.loading);
}