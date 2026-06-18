use crate::services::api_client::{send_blob_request, send_json_get_request, send_login_request, send_project_update_request, send_project_create_request, send_project_delete_request, send_logout_request, send_student_delete_request};
use crate::parser::{parse_path_parameters, parse_echosounder_parameters, parse_transport_parameters};
use crate::structs::state::{PathState, EchoState, FullSimulationRequest, SimulationUiState, CreatePathRequest};
use crate::structs::limits::ConfigLimits;
use common::StudentMeasuringParameters;
use yew::prelude::UseStateHandle;
use crate::structs::project::Project;
use crate::services::utils::set_local_storage;
use crate::structs::student::Student;
use crate::structs::student::NewStudent;
use crate::structs::project::AdminProjectView;



pub fn delete_project(
    project_id: i64,
    projects_state: UseStateHandle<Vec<Project>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    send_project_delete_request(
        project_id,
        projects_state,
        ui_mensaje,
        ui_loading
    );
}


pub fn create_project(
    name: String,
    description: String,
    file: web_sys::File,
    attempts_limit: i64,
    weather: String,
    seabed_hardness: String,
    budget: f64,
    geotiff_min_depth: f64,
    geotiff_max_depth: f64,
    projects_state: UseStateHandle<Vec<Project>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let form_data = web_sys::FormData::new().unwrap();

    let metadata_json = serde_json::json!({
        "name": name,
        "description": if description.is_empty() { None } else { Some(description) },
        "filename": file.name(),
        "attempts_limit": attempts_limit,
        "weather": weather,
        "seabed_hardness": seabed_hardness,
        "budget": budget,
        "geotiff_min_depth": geotiff_min_depth,
        "geotiff_max_depth": geotiff_max_depth
    }).to_string();

    let _ = form_data.append_with_str("metadata", &metadata_json);
    let _ = form_data.append_with_blob("file", &file);

    send_project_create_request(
        "http://localhost:3000/api/v1/projects",
        form_data,
        projects_state,
        ui_mensaje,
        ui_loading
    );
}

pub fn update_project(
    project_id: i64,
    updated_project: Project,
    projects_state: UseStateHandle<Vec<Project>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let url = format!("http://localhost:3000/api/v1/projects/{}", project_id);
    send_project_update_request(
        &url,
        updated_project,
        projects_state,
        ui_mensaje,
        ui_loading
    );
}

pub fn get_all_projects(
    projects_handle: UseStateHandle<Vec<Project>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    ui_loading.set(true);
    send_json_get_request(
        "http://localhost:3000/api/v1/projects", 
        projects_handle, 
        ui_mensaje, 
        ui_loading,
        None
    );
}

pub fn get_system_limits(
    limits_handle: UseStateHandle<ConfigLimits>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    send_json_get_request(
        "http://localhost:3000/api/v1/limits", 
        limits_handle, 
        ui_mensaje, 
        ui_loading,
        Some("Seleccione parámetros para el recorrido".to_string())
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

pub fn trigger_logout() {
    set_local_storage("group_or_user_name", "");

    send_logout_request(
        "http://localhost:3000/api/v1/auth/close_session",
        "/login"
    );
}

pub fn get_all_students(
    students_handle: UseStateHandle<Vec<Student>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    ui_loading.set(true);
    send_json_get_request(
        "http://localhost:3000/api/v1/students", 
        students_handle, 
        ui_mensaje, 
        ui_loading,
        None
    );
}

pub fn create_student(
    name: String,
    project_id: i64,
    students_state: UseStateHandle<Vec<Student>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let payload = NewStudent { name, project_id };
    crate::services::api_client::send_student_create_request(
        "http://localhost:3000/api/v1/students",
        &payload,
        students_state,
        ui_mensaje,
        ui_loading,
    );
}

pub fn delete_student(
    student_id: i64,
    students_state: UseStateHandle<Vec<Student>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let url = format!("http://localhost:3000/api/v1/students/{}", student_id);
    
    send_student_delete_request(
        &url,
        students_state,
        ui_mensaje,
        ui_loading,
    );
}

pub fn update_student(
    student_id: i64,
    updated_name: String,
    updated_project_id: i64,
    students_state: UseStateHandle<Vec<Student>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let url = format!("http://localhost:3000/api/v1/students/{}", student_id);
    
    let body = serde_json::json!({
        "name": updated_name,
        "project_id": updated_project_id
    });

    let body_string = body.to_string();

    crate::services::api_client::send_student_update_request(
        &url,
        &body_string,
        students_state,
        ui_mensaje,
        ui_loading,
    );
}

pub fn get_student_project(
    project_handle: UseStateHandle<Option<AdminProjectView>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    ui_loading.set(true);
    
    send_json_get_request(
        "http://localhost:3000/api/v1/student_project", 
        project_handle, 
        ui_mensaje, 
        ui_loading,
        None
    );
}