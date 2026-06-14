use yew::prelude::UseStateHandle;
use serde::Serialize;
use serde::de::DeserializeOwned;
use web_sys::{RequestInit, RequestMode};
use crate::services::api_utils::{get_window_fetch, build_native_post_request, create_response_closure, create_bytes_closure, create_json_closure, create_error_closure, execute_promise_chain, create_login_closure};

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
    loading: UseStateHandle<bool>
) {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request_promise = match get_window_fetch(&mensaje, &loading) {
        Some(w) => w.fetch_with_str_and_init(url, &opts),
        None => return,
    };

    // Reutilizamos closures: este pide text
    let on_response = create_response_closure(mensaje.clone(), loading.clone(), false); 
    let on_json_ready = create_json_closure(state_handle, mensaje.clone(), loading.clone());
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