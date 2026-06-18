use yew::prelude::UseStateHandle;
use serde::Serialize;
use serde::de::DeserializeOwned;
use web_sys::{RequestInit, RequestMode};
use crate::services::api_utils::{get_window_fetch, build_native_post_request, create_response_closure, create_bytes_closure, create_json_closure, create_error_closure, execute_promise_chain, create_login_closure, create_update_project_closure, create_new_project_closure, create_delete_project_closure, execute_simple_chain, create_logout_final_closure, create_new_student_closure};
use crate::structs::project::Project;

/// envía una petición POST con JSON y procesa la respuesta binaria (Blob) 
/// mapeando los resultados directamente a los estados de Yew.
pub fn send_blob_request<T: Serialize>(
    url: &str, 
    data: &T, 
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
    loading: UseStateHandle<bool>
) {
    // se prepara el json
    let body_str = match serde_json::to_string(data) {
        Ok(s) => s,
        Err(_) => {
            mensaje.set("Error de serialización".to_string());
            loading.set(false);
            return;
        }
    };

    // se construye la request
    let request = match build_native_post_request(url, &body_str) {
        Ok(r) => r,
        Err(_) => {
            mensaje.set("Error creando request".to_string());
            loading.set(false);
            return;
        }
    };

    // se lanza la request usando la API Fetch de JS a través de web-sys
    let request_promise = match get_window_fetch(&mensaje, &loading) {
        Some(w) => w.fetch_with_request(&request),
        None => return,
    };

    // creamos los 3 closures para manejar la respuesta, los bytes y los errores, respectivamente.
    let on_response = create_response_closure(mensaje.clone(), loading.clone(), true);
    let on_bytes_ready = create_bytes_closure(mensaje.clone(), image_url, loading.clone());
    let on_error = create_error_closure(mensaje, loading);

    // Ejecutamos el pipeline en JS
    // Y se ataja cualquier error obtenido en los closures.
    let _ = execute_promise_chain(&request_promise, &on_response, &on_bytes_ready, &on_error);

    // cedemos el control de la memoria a JavaScript de forma definitiva
    // se dejan vivos estos closures por si llega la respuesta.
    on_response.forget();
    on_bytes_ready.forget();
    on_error.forget();
}

// envía una peticion GET y procesa la respuesta JSON mapeando el resultado al estado de Yew.
// se usa para obtener los limites del sistema al cargar la página.
pub fn send_json_get_request<R: DeserializeOwned + 'static>(
    url: &str,
    state_handle: UseStateHandle<R>,
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>,
    mensaje_str: Option<String>
) {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(RequestMode::Cors);

    let request_promise = match get_window_fetch(&mensaje, &loading) {
        Some(w) => w.fetch_with_str_and_init(url, &opts),
        None => return,
    };

    // Reutilizamos closures: este pide text
    let on_response = create_response_closure(mensaje.clone(), loading.clone(), false); 
    let on_json_ready = create_json_closure(state_handle, mensaje.clone(), loading.clone(), mensaje_str);
    let on_error = create_error_closure(mensaje, loading);

    let _ = execute_promise_chain(&request_promise, &on_response, &on_json_ready, &on_error);

    on_response.forget();
    on_json_ready.forget();
    on_error.forget();
}

pub fn send_login_request<T: serde::Serialize>(
    url: &str, 
    data: &T, 
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>,
    display_name: String,
    redirection: String
) {
    let body_str = match serde_json::to_string(data) {
        Ok(s) => s,
        Err(_) => {
            mensaje.set("Error de serialización".to_string());
            loading.set(false);
            return;
        }
    };

    let request = match build_native_post_request(url, &body_str) {
        Ok(r) => r,
        Err(_) => {
            mensaje.set("Error creando request".to_string());
            loading.set(false);
            return;
        }
    };

    let request_promise = match get_window_fetch(&mensaje, &loading) {
        Some(w) => w.fetch_with_request(&request),
        None => return,
    };

    // false porque esperamos texto plano/OK del endpoint de login, no un Blob/imagen
    let on_response = create_response_closure(mensaje.clone(), loading.clone(), false);
    let on_success = create_login_closure(mensaje.clone(), loading.clone(), display_name, redirection);
    let on_error = create_error_closure(mensaje, loading);

    // Tu execute_promise_chain nativo se encarga de encadenar todo en JS
    let _ = execute_promise_chain(&request_promise, &on_response, &on_success, &on_error);

    on_response.forget();
    on_success.forget();
    on_error.forget();
}

pub fn send_project_update_request(
    url: &str,
    updated_project: Project,
    projects_state: UseStateHandle<Vec<Project>>,
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>
) {
    loading.set(true);

    let body_str = match serde_json::to_string(&updated_project) {
        Ok(s) => s,
        Err(_) => {
            mensaje.set("Error de serialización de datos".to_string());
            loading.set(false);
            return;
        }
    };

    let opts = RequestInit::new();
    opts.set_method("PUT");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&wasm_bindgen::JsValue::from_str(&body_str));

    let request_promise = match get_window_fetch(&mensaje, &loading) {
        Some(w) => w.fetch_with_str_and_init(url, &opts),
        None => return,
    };

    let on_response = create_response_closure(mensaje.clone(), loading.clone(), false); // false porque es texto plano
    let on_success = create_update_project_closure(projects_state, updated_project, loading.clone());
    let on_error = create_error_closure(mensaje, loading);

    let _ = execute_promise_chain(&request_promise, &on_response, &on_success, &on_error);

    on_response.forget();
    on_success.forget();
    on_error.forget();
}

// En api_client.rs
pub fn send_project_create_request(
    url: &str,
    form_data: web_sys::FormData,
    projects_state: UseStateHandle<Vec<Project>>,
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>,
) {
    loading.set(true);

    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(web_sys::RequestMode::Cors);
    opts.set_body(&form_data); 

    let request_promise = match get_window_fetch(&mensaje, &loading) {
        Some(w) => w.fetch_with_str_and_init(url, &opts),
        None => return,
    };

    let on_response = create_response_closure(mensaje.clone(), loading.clone(), true);
    let on_success = create_new_project_closure(projects_state, loading.clone(), mensaje.clone());
    let on_error = create_error_closure(mensaje, loading);

    let _ = execute_promise_chain(&request_promise, &on_response, &on_success, &on_error);

    on_response.forget();
    on_success.forget();
    on_error.forget();
}

pub fn send_project_delete_request(
    project_id: i64,
    projects_state: UseStateHandle<Vec<Project>>,
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>,
) {
    loading.set(true);

    let url = format!("http://localhost:3000/api/v1/projects/{}", project_id);

    // Configuración nativa del Request en la capa cliente
    let opts = RequestInit::new();
    opts.set_method("DELETE");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(RequestMode::Cors);

    let request_promise = match get_window_fetch(&mensaje, &loading) {
        Some(w) => w.fetch_with_str_and_init(&url, &opts),
        None => return,
    };

    let on_response = create_response_closure(mensaje.clone(), loading.clone(), true);
    let on_success = create_delete_project_closure(project_id, projects_state, loading.clone(), mensaje.clone());
    let on_error = create_error_closure(mensaje, loading);

    let _ = execute_promise_chain(&request_promise, &on_response, &on_success, &on_error);

    on_response.forget();
    on_success.forget();
    on_error.forget();
}

pub fn send_logout_request(url: &str, redirection: &str) {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(RequestMode::Cors);

    let request_promise = match web_sys::window() {
        Some(w) => w.fetch_with_str_and_init(url, &opts),
        None => return,
    };

    let on_success = create_logout_final_closure(redirection.to_string());
    let on_error = create_logout_final_closure(redirection.to_string());

    let _ = execute_simple_chain(&request_promise, &on_success, &on_error);

    on_success.forget();
    on_error.forget();
}

pub fn send_student_create_request(
    url: &str,
    new_student: &crate::structs::student::NewStudent,
    students_state: UseStateHandle<Vec<crate::structs::student::Student>>,
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>,
) {
    loading.set(true);
    let body_str = serde_json::to_string(new_student).unwrap_or_default();

    let request = match build_native_post_request(url, &body_str) {
        Ok(r) => r,
        Err(_) => {
            mensaje.set("Error al preparar la request".to_string());
            loading.set(false);
            return;
        }
    };

    let request_promise = match get_window_fetch(&mensaje, &loading) {
        Some(w) => w.fetch_with_request(&request),
        None => return,
    };

    let on_success = create_new_student_closure(students_state, loading.clone(), mensaje.clone());
    let on_error = create_error_closure(mensaje, loading);

    let _ = execute_simple_chain(&request_promise, &on_success, &on_error);

    // Olvidamos solo los dos closures usados
    on_success.forget();
    on_error.forget();
}

pub fn send_student_delete_request(
    url: &str,
    students_state: UseStateHandle<Vec<crate::structs::student::Student>>,
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>,
) {
    loading.set(true);

    let opts = web_sys::RequestInit::new();
    opts.set_method("DELETE");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(RequestMode::Cors);

    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            mensaje.set("Error: No se encontró la ventana del navegador".to_string());
            loading.set(false);
            return;
        }
    };

    let request_promise = window.fetch_with_str_and_init(url, &opts);

    let on_success = crate::services::api_utils::create_delete_student_closure(
        students_state, 
        loading.clone(), 
        mensaje.clone()
    );
    let on_error = crate::services::api_utils::create_error_closure(mensaje, loading);

    let _ = crate::services::api_utils::execute_simple_chain(&request_promise, &on_success, &on_error);

    on_success.forget();
    on_error.forget();
}

pub fn send_student_update_request(
    url: &str,
    body_json: &str,
    students_state: UseStateHandle<Vec<crate::structs::student::Student>>,
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>,
) {
    loading.set(true);

    let opts = web_sys::RequestInit::new();
    opts.set_method("PUT");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    
    let headers = web_sys::Headers::new().unwrap();
    headers.append("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);
    
    let body_js = wasm_bindgen::JsValue::from_str(body_json);
    opts.set_body(&body_js);

    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            mensaje.set("Error: No se encontró la ventana del navegador".to_string());
            loading.set(false);
            return;
        }
    };

    let request_promise = window.fetch_with_str_and_init(url, &opts);

    let on_success = crate::services::api_utils::create_update_student_closure(
        students_state, 
        loading.clone(), 
        mensaje.clone()
    );
    let on_error = crate::services::api_utils::create_error_closure(mensaje, loading);

    let _ = crate::services::api_utils::execute_simple_chain(&request_promise, &on_success, &on_error);

    on_success.forget();
    on_error.forget();
}