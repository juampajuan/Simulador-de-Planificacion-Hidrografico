use yew::prelude::UseStateHandle;
use serde::Serialize;
use web_sys::{Url, Blob, Request, RequestInit, RequestMode, Response};
use js_sys::{Uint8Array, Array, Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
    let request = match build_native_request(url, &body_str) {
        Ok(r) => r,
        Err(_) => {
            mensaje.set("Error creando request".to_string());
            loading.set(false);
            return;
        }
    };

    // se lanza la request usando la API Fetch de JS a través de web-sys
    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    // creamos los 3 closures para manejar la respuesta, los bytes y los errores, respectivamente.
    let on_response = create_response_closure(mensaje.clone(), loading.clone());
    let on_bytes_ready = create_bytes_closure(mensaje.clone(), image_url, loading.clone());
    let on_error = create_error_closure(mensaje, loading);

    // Ejecutamos el pipeline en JS
    let _ = execute_promise_chain(&request_promise, &on_response, &on_bytes_ready, &on_error);

    // cedemos el control de la memoria a JavaScript de forma definitiva
    // se dejan vivos estos closures por si llega la respuesta.
    on_response.forget();
    on_bytes_ready.forget();
    on_error.forget();
}

// Construye un objeto Request con método POST, CORS y cuerpo JSON.
fn build_native_request(url: &str, body: &str) -> Result<Request, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(body));

    let request = Request::new_with_str_and_init(url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    Ok(request)
}

// el closure de la respuesta. Se encarga de verificar el status y extraer el ArrayBuffer del cuerpo.
fn create_response_closure(
    mensaje: UseStateHandle<String>, 
    loading: UseStateHandle<bool>
) -> Closure<dyn FnMut(JsValue) -> Result<JsValue, JsValue>> {
    Closure::wrap(Box::new(move |res: JsValue| -> Result<JsValue, JsValue> {
        let response: Response = res.dyn_into().unwrap();
        if response.status() == 200 {
            Ok(response.array_buffer().unwrap().into())
        } else {
            mensaje.set(format!("Error del servidor: {}", response.status()));
            loading.set(false);
            Err(JsValue::from_str("HTTP Status != 200"))
        }
    }) as Box<dyn FnMut(JsValue) -> Result<JsValue, JsValue>>)
}

// despues el closure que recibe el ArrayBuffer, lo convierte a Blob, genera una URL y la asigna al estado de Yew para mostrar la imagen.
// array -> -> blob -> url -> estado de Yew
fn create_bytes_closure(
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

// el closure del error
fn create_error_closure(
    mensaje: UseStateHandle<String>, 
    loading: UseStateHandle<bool>
) -> Closure<dyn FnMut(JsValue)> {
    Closure::wrap(Box::new(move |_err: JsValue| {
        mensaje.set("Error de conexión o datos inválidos".to_string());
        loading.set(false);
    }) as Box<dyn FnMut(JsValue)>)
}

// encadena las promises ejecutando los closures.
fn execute_promise_chain(
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