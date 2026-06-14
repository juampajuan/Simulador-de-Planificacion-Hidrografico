use crate::services::api_client::{send_blob_request, send_json_get_request, send_login_request};
use crate::parser::{parse_path_parameters, parse_echosounder_parameters, parse_transport_parameters};
use crate::structs::state::{PathState, EchoState, FullSimulationRequest, SimulationUiState, CreatePathRequest};
use crate::structs::limits::ConfigLimits;
use common::StudentMeasuringParameters;
use yew::prelude::UseStateHandle;

pub fn get_system_limits(
    limits_handle: UseStateHandle<ConfigLimits>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    send_json_get_request(
        "http://localhost:3000/api/v1/limits", 
        limits_handle, 
        ui_mensaje, 
        ui_loading
    );
}

pub fn trigger_path_generation(
    state: &PathState, 
    ui: SimulationUiState,
    limits: &ConfigLimits
) {
    if state.separacion.is_empty() || state.azimut.is_empty() { return; }

    let params = match parse_path_parameters(&state, &limits) {
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

pub fn run_simulation(echo_state: &EchoState, path_state: &PathState, ui: SimulationUiState, limits: &ConfigLimits) {
    let echo_params = match parse_echosounder_parameters(echo_state, &limits) {
        Ok(p) => p,
        Err(err) => { ui.mensaje.set(err); return; }
    };

    let path_params = match parse_path_parameters(path_state, &limits) {
        Ok(p) => p,
        Err(err) => { ui.mensaje.set(err); return; }
    };

    let transport_params = match parse_transport_parameters(echo_state, &limits) {
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

pub fn trigger_login(
    student_code: &str,
    teacher_user: &str,
    teacher_password: &str,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>
) {
    let (credentials, display_name,redirection) = if !student_code.is_empty() {
        (
            serde_json::json!({ "code": student_code }),
            student_code.to_string(),
            "/".to_string()
        )
    } else if !teacher_user.is_empty() && !teacher_password.is_empty() {
        (
            serde_json::json!({ "user": teacher_user, "pass": teacher_password }),
            teacher_user.to_string(),
            "/admin".to_string()
        )
    } else {
        return;
    };

    ui_mensaje.set("Autenticando...".to_string());
    ui_loading.set(true);

    send_login_request(
        "http://localhost:3000/api/v1/auth/login",
        &credentials,
        ui_mensaje,
        ui_loading,
        display_name,
        redirection
    );
}