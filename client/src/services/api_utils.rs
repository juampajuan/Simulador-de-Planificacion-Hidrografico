use yew::prelude::UseStateHandle;
use serde::de::DeserializeOwned;
use web_sys::{Url, Blob, Request, RequestInit, RequestMode, Response};
use js_sys::{Uint8Array, Array, Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::structs::project::Project;


pub fn get_window_fetch(mensaje: &UseStateHandle<String>, loading: &UseStateHandle<bool>) -> Option<web_sys::Window> {
    match web_sys::window() {
        Some(w) => Some(w),
        None => {
            mensaje.set("Error crítico: No se detectó el entorno del navegador".to_string());
            loading.set(false);
            None
        }
    }
}

// Construye un objeto Request con método POST, CORS y cuerpo JSON.
pub fn build_native_post_request(url: &str, body: &str) -> Result<Request, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(body));

    let request = Request::new_with_str_and_init(url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    Ok(request)
}

pub fn create_login_closure(
    mensaje: UseStateHandle<String>, 
    loading: UseStateHandle<bool>,
    display_name: String,
    redirection: String
) -> Closure<dyn FnMut(JsValue) -> JsValue> {
    Closure::wrap(Box::new(move |_text_val: JsValue| -> JsValue {
        mensaje.set("Login exitoso. Redirigiendo...".to_string());
        loading.set(false);

        if let Some(w) = web_sys::window() {
            if let Ok(Some(storage)) = w.local_storage() {
                let _ = storage.set_item("group_or_user_name", &display_name);
                let _ = w.location().set_pathname(&redirection); 
            }
        }
        JsValue::UNDEFINED
    }) as Box<dyn FnMut(JsValue) -> JsValue>)
}

// el closure de la respuesta. Se encarga de verificar el status y extraer el ArrayBuffer del cuerpo.
pub fn create_response_closure(
    mensaje: UseStateHandle<String>, 
    loading: UseStateHandle<bool>,
    is_blob: bool 
) -> Closure<dyn FnMut(JsValue) -> Result<JsValue, JsValue>> {
    Closure::wrap(Box::new(move |res: JsValue| -> Result<JsValue, JsValue> {
        let response: Response = match res.dyn_into() {
            Ok(r) => r,
            Err(_) => {
                mensaje.set("Error procesando la respuesta del servidor".to_string());
                loading.set(false);
                return Err(JsValue::from_str("No se pudo convertir a Response"));
            }
        };
        if response.status() == 200 {
            if is_blob {
                response.array_buffer().map(|p| p.into()).map_err(|_| JsValue::from_str("No ArrayBuffer"))
            } else {
                response.text().map(|p| p.into()).map_err(|_| JsValue::from_str("No Text"))
            }
        } else {
            mensaje.set(format!("Error del servidor: {}", response.status()));
            loading.set(false);
            Err(JsValue::from_str("HTTP Status != 200"))
        }
    }) as Box<dyn FnMut(JsValue) -> Result<JsValue, JsValue>>)
}

// despues el closure que recibe el ArrayBuffer, lo convierte a Blob, genera una URL y la asigna al estado de Yew para mostrar la imagen.
// array -> -> blob -> url -> estado de Yew
pub fn create_bytes_closure(
    mensaje: UseStateHandle<String>, 
    image_url: UseStateHandle<Option<String>>,
    loading: UseStateHandle<bool>
) -> Closure<dyn FnMut(JsValue) -> JsValue> {
    Closure::wrap(Box::new(move |buffer: JsValue| -> JsValue {
        let uint8_array = Uint8Array::new(&buffer);
        let array = Array::of1(&uint8_array.buffer());
        
        if let Ok(blob) = Blob::new_with_u8_array_sequence(&array) {
            if let Ok(url) = Url::create_object_url_with_blob(&blob) {
                if let Some(old_url) = (*image_url).clone() {
                    let _ = Url::revoke_object_url(&old_url);
                }
                image_url.set(Some(url));
                mensaje.set("Operación exitosa".to_string());
            }
        }
        loading.set(false);
        JsValue::UNDEFINED
    }) as Box<dyn FnMut(JsValue) -> JsValue>)
}

// recibe el string json, lo deserializa al tipo esperado y lo asigna al estado de Yew.
pub fn create_json_closure<R: DeserializeOwned + 'static>(
    state_handle: UseStateHandle<R>,
    mensaje: UseStateHandle<String>, 
    loading: UseStateHandle<bool>,
    success_msg: Option<String>
) -> Closure<dyn FnMut(JsValue) -> JsValue> {
    Closure::wrap(Box::new(move |text_val: JsValue| -> JsValue {
        if let Some(json_str) = text_val.as_string() {
            match serde_json::from_str::<R>(&json_str) {
                Ok(parsed_data) => {
                    state_handle.set(parsed_data);
                    
                    if let Some(ref msg) = success_msg {
                        mensaje.set(msg.clone());
                    } else {
                        mensaje.set(String::new());
                    }
                },
                Err(_) => {
                    mensaje.set("Error al deserializar la respuesta del sistema".to_string());
                }
            }
        }
        loading.set(false);
        JsValue::UNDEFINED
    }) as Box<dyn FnMut(JsValue) -> JsValue>)
}

pub fn create_update_project_closure(
    projects_state: UseStateHandle<Vec<Project>>,
    updated_project: Project,
    loading: UseStateHandle<bool>
) -> Closure<dyn FnMut(JsValue) -> JsValue> {
    Closure::wrap(Box::new(move |_text_val: JsValue| -> JsValue {
        let mut list = (*projects_state).clone();
        
        if let Some(pos) = list.iter().position(|p| p.id == updated_project.id) {
            list[pos] = updated_project.clone();
            projects_state.set(list);
        }
        loading.set(false);
        JsValue::UNDEFINED
    }) as Box<dyn FnMut(JsValue) -> JsValue>)
}

// el closure del error
pub fn create_error_closure(
    mensaje: UseStateHandle<String>, 
    loading: UseStateHandle<bool>
) -> Closure<dyn FnMut(JsValue)> {
    Closure::wrap(Box::new(move |_err: JsValue| {
        mensaje.set("Error de conexión o datos inválidos".to_string());
        loading.set(false);
    }) as Box<dyn FnMut(JsValue)>)
}

// encadena las promises ejecutando los closures.
pub fn execute_promise_chain(
    root_promise: &JsValue,
    cb_resp: &Closure<dyn FnMut(JsValue) -> Result<JsValue, JsValue>>,
    cb_bytes: &Closure<dyn FnMut(JsValue) -> JsValue>,
    cb_err: &Closure<dyn FnMut(JsValue)>
) -> Result<(), JsValue> {
    let then_fn: Function = Reflect::get(root_promise, &JsValue::from_str("then"))?.dyn_into()?;
    let promise_2 = then_fn.call1(root_promise, cb_resp.as_ref())?;
    let promise_3 = then_fn.call1(&promise_2, cb_bytes.as_ref())?;
    
    let catch_fn: Function = Reflect::get(&promise_3, &JsValue::from_str("catch"))?.dyn_into()?;
    catch_fn.call1(&promise_3, cb_err.as_ref())?;
    Ok(())
}