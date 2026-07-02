use yew::prelude::UseStateHandle;
use crate::parser::{parse_path_parameters, parse_echosounder_parameters, parse_transport_parameters};
use crate::structs::state::{PathState, EchoState, FullSimulationRequest, SimulationUiState, CreatePathRequest};
use crate::structs::limits::ConfigLimits;
use crate::structs::student::{Student, NewStudent};
use crate::structs::project::{AdminProjectView, NewProject, Project};
use common::{StudentMeasuringParameters, SimulationBase64Response, PathParameters, TransportParameters, EchosounderParameters};
use crate::services::api_client::{send_native_request, send_native_formdata_request, send_native_blob_request};
use crate::services::api_utils::{process_local_login, process_local_logout};
use crate::pages::student::components::measure_params::AttemptsState;

#[derive(serde::Deserialize, Clone, PartialEq, Debug)]
pub struct StudentProjectResponse {
    #[serde(flatten)]
    pub project: AdminProjectView,  // toda la info de proyecto
    pub attempts_spent: i64,
    pub coordinates: GeoCorners,
    pub maptiler_api_key: String,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct GeoCorners {
    pub sup_izq: (f64, f64),
    pub sup_der: (f64, f64),
    pub inf_izq: (f64, f64),
    pub inf_der: (f64, f64),
    pub centro: (f64, f64),
}

#[derive(serde::Deserialize, Clone, PartialEq, Debug)]
pub struct StudentSimulation {
    pub id: i64,
    pub selected: bool,
    pub result_min_depth: f64,
    pub result_max_depth: f64,
    pub student_id: i64,
    pub project_id: i64,
    pub path_parameters: PathParameters,
    pub transport_parameters: TransportParameters,
    pub echosounder_parameters: EchosounderParameters,
}

/// Obtiene el historial de simulaciones/intentos.
/// Si `target_student_id` es `Some(id)`, se concatena como query string (usado por docentes).
/// Si es `None`, se pide la ruta limpia (usado por el alumno autenticado).
pub fn get_student_simulations_history(
    target_student_id: Option<i64>,
    history_handle: UseStateHandle<Vec<StudentSimulation>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let url = match target_student_id {
        Some(id) => format!("/api/v1/exams/my_simulations?student_id={}", id),
        None => "/api/v1/exams/my_simulations".to_string(),
    };

    ui_mensaje.set("Cargando historial de intentos...".to_string());

    let msg_for_error = ui_mensaje.clone();

    send_native_request(
        &url,
        "GET",
        None,
        Some(ui_mensaje.clone()),
        Some(ui_loading),
        move |response_text| {
            if let Ok(historial) = serde_json::from_str::<Vec<StudentSimulation>>(&response_text) {
                history_handle.set(historial);
                ui_mensaje.set(String::new()); 
            } else {
                ui_mensaje.set("Error al interpretar el historial de simulaciones".to_string());
            }
        },
        Some(move |status_code: u16| {
            msg_for_error.set(format!("Error al obtener el historial. Código del servidor: {}", status_code));
        })
    );
}

/// Registra o remueve la entrega de una simulación en el servidor.
/// Si `simulation_id` es `Some(id)`, se entrega ese intento. Si es `None`, se quita la entrega actual.
pub fn select_exam_delivery(
    simulation_id: Option<i64>,
    ui_mensaje: UseStateHandle<String>,
) {
    let payload = serde_json::json!({ "simulation_id": simulation_id }).to_string();

    send_native_request(
        "/api/v1/exams/select_delivery",
        "POST",
        Some(&payload),
        Some(ui_mensaje),
        None,
        move |_| {},
        Some(|_| {})
    );
}

// Devuelve todos los estudiantes (grupo o individuo).
pub fn get_all_students(
    students_handle: UseStateHandle<Vec<Student>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    send_native_request(
        "/api/v1/students",
        "GET",
        None,
        Some(ui_mensaje.clone()),
        Some(ui_loading),
        move |response_text| {
            if let Ok(lista) = serde_json::from_str::<Vec<Student>>(&response_text) {
                students_handle.set(lista);
                ui_mensaje.set(String::new());
            } else {
                ui_mensaje.set("Error de lectura de datos de estudiantes".to_string());
            }
        },
        Some(|_| {})
    );
}

// Crea estudiante (grupo o individuo) asociando a un proyecto.
pub fn create_student(
    name: String,
    project_id: i64,
    students_state: UseStateHandle<Vec<Student>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let payload = serde_json::to_string(&NewStudent { name, project_id }).unwrap_or_default();
    send_native_request(
        "/api/v1/students",
        "POST",
        Some(&payload),
        Some(ui_mensaje.clone()),
        Some(ui_loading.clone()),
        move |_| {
            ui_mensaje.set("Grupo creado con éxito".to_string());
            get_all_students(students_state, ui_mensaje, ui_loading);
        },
        Some(|_| {})
    );
}

// Actualiza nombre del estudiante asociado a proyecto o proyecto asignado, dependiendo el caso.
pub fn update_student(
    student_id: i64,
    updated_name: String,
    updated_project_id: i64,
    students_state: UseStateHandle<Vec<Student>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let url = format!("/api/v1/students/{}", student_id);
    let body = serde_json::json!({ "name": updated_name, "project_id": updated_project_id }).to_string();

    send_native_request(
        &url,
        "PUT",
        Some(&body),
        Some(ui_mensaje.clone()),
        Some(ui_loading.clone()),
        move |_| {
            ui_mensaje.set("Estudiante actualizado con éxito".to_string());
            get_all_students(students_state, ui_mensaje, ui_loading);
        },
        Some(|_| {})
    );
}

// Elimina estudiante
pub fn delete_student(
    student_id: i64,
    students_state: UseStateHandle<Vec<Student>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let url = format!("/api/v1/students/{}", student_id);
    send_native_request(
        &url,
        "DELETE",
        None,
        Some(ui_mensaje.clone()),
        Some(ui_loading.clone()),
        move |_| {
            ui_mensaje.set("Grupo eliminado con éxito".to_string());
            get_all_students(students_state, ui_mensaje, ui_loading);
        },
        Some(|_| {})
    );
}

// Devuelve todos los proyectos (asignados o no)
pub fn get_all_projects(
    projects_handle: UseStateHandle<Vec<Project>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    send_native_request(
        "/api/v1/projects",
        "GET",
        None,
        Some(ui_mensaje.clone()),
        Some(ui_loading),
        move |response_text| {
            if let Ok(lista) = serde_json::from_str::<Vec<Project>>(&response_text) {
                projects_handle.set(lista);
                ui_mensaje.set(String::new());
            }
        },
        Some(|_| {})
    );
}

// Crea proyecto
pub fn create_project(
    project: NewProject,
    projects_state: UseStateHandle<Vec<Project>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let NewProject {
        name,
        description,
        file,
        attempts_limit,
        weather,
        seabed_hardness,
        budget,
        geotiff_min_depth,
        geotiff_max_depth,
    } = project;

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

    let msg = ui_mensaje.clone();
    let load = ui_loading.clone();
    send_native_formdata_request(
        "/api/v1/projects",
        form_data,
        ui_mensaje,
        ui_loading,
        move || {
            get_all_projects(projects_state, msg, load);
        }
    );
}

// Actualiza info de proyecto
pub fn update_project(
    project_id: i64,
    updated_project: Project,
    projects_state: UseStateHandle<Vec<Project>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let url = format!("/api/v1/projects/{}", project_id);
    let body = serde_json::to_string(&updated_project).unwrap_or_default();

    send_native_request(
        &url,
        "PUT",
        Some(&body),
        Some(ui_mensaje.clone()),
        Some(ui_loading),
        move |_| {
            let mut list = (*projects_state).clone();
            if let Some(pos) = list.iter().position(|p| p.id == updated_project.id) {
                list[pos] = updated_project;
                projects_state.set(list);
            }
            ui_mensaje.set("Proyecto modificado con éxito".to_string());
        },
        Some(|_| {})
    );
}

// Borra proyecto
pub fn delete_project(
    project_id: i64,
    projects_state: UseStateHandle<Vec<Project>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    let url = format!("/api/v1/projects/{}", project_id);
    send_native_request(
        &url,
        "DELETE",
        None,
        Some(ui_mensaje.clone()),
        Some(ui_loading),
        move |_| {
            let mut current_list = (*projects_state).clone();
            current_list.retain(|p| p.id != project_id);
            projects_state.set(current_list);
            ui_mensaje.set(String::new());
        },
        Some(|_| {})
    );
}

// Obtiene los limites de parámetros configurables.
pub fn get_system_limits(
    limits_handle: UseStateHandle<ConfigLimits>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    send_native_request(
        "/api/v1/limits",
        "GET",
        None,
        Some(ui_mensaje.clone()),
        Some(ui_loading),
        move |response_text| {
            if let Ok(parsed) = serde_json::from_str::<ConfigLimits>(&response_text) {
                limits_handle.set(parsed);
                ui_mensaje.set("Seleccione parámetros para el recorrido".to_string());
            }
        },
        Some(|_| {})
    );
}

// Obtiene el proyecto de un estudiante.
pub fn get_student_project(
    project_handle: UseStateHandle<Option<StudentProjectResponse>>,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>,
) {
    send_native_request(
        "/api/v1/student_project",
        "GET",
        None,
        Some(ui_mensaje.clone()),
        Some(ui_loading),
        move |response_text| {
            if let Ok(parsed) = serde_json::from_str::<StudentProjectResponse>(&response_text) {
                project_handle.set(Some(parsed));
                ui_mensaje.set(String::new());
            } else {
                ui_mensaje.set("Error al interpretar los datos del proyecto y sus intentos".to_string());
            }
        },
        Some(|_| {})
    );
}

// Pide la creación de un path. Usando CreatePathRequest
pub fn trigger_path_generation(state: &PathState, ui: SimulationUiState, limits: &ConfigLimits) {
    if state.separacion.is_empty() || state.azimut.is_empty() { return; }
    let params = match parse_path_parameters(state, limits) {
        Ok(p) => p,
        Err(err_msg) => { ui.mensaje.set(err_msg); return; }
    };
    ui.map_base64.set(None);
    ui.scale_base64.set(None);
    ui.min_depth.set(0.0);
    ui.max_depth.set(0.0);
    ui.mensaje.set("Generando recorrido...".to_string());
    let request_body = serde_json::to_string(&CreatePathRequest { path_parameters: params }).unwrap_or_default();

    send_native_blob_request("/api/v1/create_path", &request_body, ui.image_url, ui.mensaje, ui.loading);
}

// Pide la ejecución de la simulación. Usando tanto echo como path state.
pub fn run_simulation(
    echo_state: &EchoState, 
    path_state: &PathState, 
    ui: SimulationUiState, 
    limits: &ConfigLimits,
    attempts_handle: UseStateHandle<AttemptsState>,
) {
    let echo_params = match parse_echosounder_parameters(echo_state, limits) {
        Ok(p) => p,
        Err(err) => { ui.mensaje.set(err); return; }
    };
    let path_params = match parse_path_parameters(path_state, limits) {
        Ok(p) => p,
        Err(err) => { ui.mensaje.set(err); return; }
    };
    let transport_params = match parse_transport_parameters(echo_state, limits) {
        Ok(t) => t,
        Err(e) => { ui.mensaje.set(e); return; }
    };
    ui.image_url.set(None);
    ui.mensaje.set("Simulando medición...".to_string());
    ui.loading.set(true);

    let simulation_params = FullSimulationRequest {
        echo_parameters: StudentMeasuringParameters { transport_parameters: transport_params, echo_sounder_parameters: echo_params },
        path_parameters: path_params,
    };
    let request_body = serde_json::to_string(&simulation_params).unwrap_or_default();

    let map_handle = ui.map_base64.clone();
    let scale_handle = ui.scale_base64.clone();
    let min_handle = ui.min_depth.clone();
    let max_handle = ui.max_depth.clone();
    let msg_handle_success = ui.mensaje.clone(); 
    let msg_handle_error = ui.mensaje.clone();
    
    let attempts_handle_success = attempts_handle.clone(); 

    send_native_request(
        "/api/v1/run_simulation",
        "POST",
        Some(&request_body),
        Some(ui.mensaje.clone()),
        Some(ui.loading.clone()),
        move |response_text| {
            if let Ok(data) = serde_json::from_str::<SimulationBase64Response>(&response_text) {
                map_handle.set(Some(data.map_base64));
                scale_handle.set(Some(data.scale_base64));
                min_handle.set(data.min_depth);
                max_handle.set(data.max_depth);
                msg_handle_success.set(String::new());

                let mut current_attempts = (*attempts_handle_success).clone();
                current_attempts.spent += 1;
                attempts_handle_success.set(current_attempts);
            } else {
                msg_handle_success.set("Error al interpretar la respuesta de simulación".to_string());
            }
        },
        Some(move |status_code| {
            if status_code == 403 {
                msg_handle_error.set("Has alcanzado el límite máximo de intentos permitidos para este proyecto.".to_string());
            } else {
                msg_handle_error.set(format!("Error en el servidor: Código {}", status_code));
            }
        })
    );
}

// Pide la creacion del area de cobertura, usando tanto echo como path state.
pub fn run_coverage(echo_state: &EchoState, path_state: &PathState, ui: SimulationUiState, limits: &ConfigLimits) {
    let echo_params = match parse_echosounder_parameters(echo_state, limits) {
        Ok(p) => p,
        Err(err) => { ui.mensaje.set(err); return; }
    };
 
    let path_params = match parse_path_parameters(path_state, limits) {
        Ok(p) => p,
        Err(err) => { ui.mensaje.set(err); return; }
    };
 
    let transport_params = match parse_transport_parameters(echo_state, limits) {
        Ok(t) => t,
        Err(e) => { ui.mensaje.set(e); return; }
    };
    ui.map_base64.set(None);
    ui.scale_base64.set(None);
    ui.min_depth.set(0.0);
    ui.max_depth.set(0.0);
    ui.mensaje.set("Calculando cobertura...".to_string());
    ui.loading.set(true);
 
    let simulation_params = FullSimulationRequest {
        echo_parameters: StudentMeasuringParameters {
            transport_parameters: transport_params,
            echo_sounder_parameters: echo_params,
        },
        path_parameters: path_params,
    };

    match serde_json::to_string(&simulation_params) {
        Ok(body_json) => {
            send_native_blob_request(
                "/api/v1/coverage_image", 
                &body_json, 
                ui.image_url, 
                ui.mensaje, 
                ui.loading
            );
        },
        Err(_) => {
            ui.mensaje.set("Error interno al preparar los datos de simulación".to_string());
            ui.loading.set(false);
        }
    }
}

// Ejecuta el login, para ambos roles, si la información es correcta.
pub fn trigger_login(
    student_code: &str,
    teacher_user: &str,
    teacher_password: &str,
    ui_mensaje: UseStateHandle<String>,
    ui_loading: UseStateHandle<bool>
) {
    let (credentials, display_name, redirection) = if !student_code.is_empty() {
        (serde_json::json!({ "code": student_code }), student_code.to_string(), "/".to_string())
    } else if !teacher_user.is_empty() && !teacher_password.is_empty() {
        (serde_json::json!({ "user": teacher_user, "pass": teacher_password }), teacher_user.to_string(), "/admin".to_string())
    } else {
        return;
    };

    ui_mensaje.set("Autenticando...".to_string());
    let msg_err = ui_mensaje.clone();
    let redirection_clone = redirection.clone();
    send_native_request(
        "/api/v1/auth/login",
        "POST",
        Some(&credentials.to_string()),
        Some(ui_mensaje.clone()),
        Some(ui_loading),
        move |response_text| { 
            let real_name = if !response_text.trim().is_empty() {
                response_text.trim().to_string()
            } else {
                display_name
            };

            if let Some(window) = web_sys::window() 
                && let Ok(Some(storage)) = window.local_storage() {
                    let role = if redirection_clone == "/admin" { "admin" } else { "student" };
                    let _ = storage.set_item("user_role", role);
                    let _ = storage.set_item("group_or_user_name", &real_name);
                }
            

            process_local_login(&real_name, &redirection, ui_mensaje);
        },
        Some(move |status_code| {
            if status_code == 401 {
                msg_err.set("Error de conexión o credenciales inválidas".to_string());
            } else {
                msg_err.set(format!("Error de autenticación: {}", status_code));
            }
        })
    );
}

// Ejecuta el logout
pub fn trigger_logout() {
    send_native_request(
        "/api/v1/auth/close_session",
        "POST",
        None,
        None,
        None,
        move |_| {
            process_local_logout("/login");
        },
        Some(|_| {})
    );
}