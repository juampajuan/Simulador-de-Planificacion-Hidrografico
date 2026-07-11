use yew::prelude::UseStateHandle;
use web_sys::{RequestInit, RequestMode, Response, Url, Blob};
use js_sys::{Function, Reflect, Uint8Array, Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn get_window_fetch(
    mensaje: Option<&UseStateHandle<String>>, 
    loading: Option<&UseStateHandle<bool>>
) -> Option<web_sys::Window> {
    match web_sys::window() {
        Some(w) => Some(w),
        None => {
            if let Some(msg) = mensaje {
                msg.set("Error crítico: No se detectó el entorno del navegador".to_string());
            }
            if let Some(load) = loading {
                load.set(false);
            }
            None
        }
    }
}

// Usamos `wasm_bindgen` como puente de comunicación para enlazar Rust con JavaScript.
// Usamos `web_sys` que nos provee las funciones nativas del navegador (como `fetch` o `window`) traducidas y tipadas en Rust. (porque wasm no nos provee de eso)
// Manejamos la asincronía de JS (Promesas) envolviendo nuestros callbacks de Rust en `Closure`
// y usando `.forget()` para mantenerlos vivos en la memoria de JS hasta que el servidor responda.

//envía peticiones de Texto o JSON Plano
pub fn send_native_request(
    url: &str,
    method: &str,
    body_json: Option<&str>,
    mensaje: Option<UseStateHandle<String>>,
    loading: Option<UseStateHandle<bool>>,
    on_success_cb: impl FnOnce(String) + 'static,
    on_error_cb: Option<impl FnOnce(u16) + 'static>,
) {
    if let Some(ref load) = loading {
        load.set(true);
    }
    
    let opts = RequestInit::new();
    opts.set_method(method);
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(RequestMode::Cors);
    
    if let Some(body) = body_json {
        let headers = web_sys::Headers::new().unwrap();
        headers.append("Content-Type", "application/json").unwrap();
        opts.set_headers(&headers);
        opts.set_body(&JsValue::from_str(body));
    }

    let window = match get_window_fetch(mensaje.as_ref(), loading.as_ref()) {
        Some(w) => w,
        None => return,
    };

    let request_promise = window.fetch_with_str_and_init(url, &opts);

    let msg_err = mensaje.clone();
    let load_err = loading.clone();
    let on_error = Closure::wrap(Box::new(move |_err: JsValue| {
        if let Some(ref msg) = msg_err { 
            msg.set("Error de conexión o datos inválidos".to_string()); 
        }
        if let Some(ref load) = load_err { load.set(false); }
    }) as Box<dyn FnMut(JsValue)>);

    let msg_resp = mensaje.clone();
    let load_resp = loading.clone();
    let mut on_success_opt = Some(on_success_cb);
    let mut on_error_opt = on_error_cb; // Capturamos el callback de negocio

    let on_success = Closure::wrap(Box::new(move |res: JsValue| {
        let response: Response = res.unchecked_into();
        let msg = msg_resp.clone();
        let load = load_resp.clone();
        let status = response.status();
        
        if status == 200 {
            let mut cb_container = on_success_opt.take();
            let text_promise = response.text().unwrap();
            
            let on_text_ready = Closure::wrap(Box::new(move |text: JsValue| {
                if let Some(ref l) = load { l.set(false); }
                if let Some(txt_str) = text.as_string() 
                    && let Some(cb) = cb_container.take() {
                        cb(txt_str); 
                }
            }) as Box<dyn FnMut(JsValue)>);

            let then_fn: Function = Reflect::get(&text_promise, &JsValue::from_str("then")).unwrap().dyn_into().unwrap();
            let _ = then_fn.call1(&text_promise, on_text_ready.as_ref());
            on_text_ready.forget();
        } else {
            if let Some(ref l) = load { l.set(false); }

            if (status == 401 || status == 403)
                && let Some(win) = web_sys::window()
            {
                if let Ok(Some(storage)) = win.local_storage() {
                    let _ = storage.remove_item("group_or_user_name");
                    let _ = storage.remove_item("user_role");
                }
            }
            
            if let Some(cb_err) = on_error_opt.take() {
                // Delegamos el estado al callback para que tu LoginPage muestre el mensaje correspondiente
                cb_err(status);
            } else if let Some(ref m) = msg {
                m.set(format!("Error del servidor: {}", status));
            }
        }
    }) as Box<dyn FnMut(JsValue)>);

    // function y reflect, para manejar callbacks y .then
    let then_fn: Function = Reflect::get(&request_promise, &JsValue::from_str("then")).unwrap().dyn_into().unwrap();
    let promise_2 = then_fn.call1(&request_promise, on_success.as_ref()).unwrap();
    let catch_fn: Function = Reflect::get(&promise_2, &JsValue::from_str("catch")).unwrap().dyn_into().unwrap();
    let _ = catch_fn.call1(&promise_2, on_error.as_ref());

    on_success.forget();
    on_error.forget();
}

//envia peticiones con archivos Binarios (Multipart FormData)
pub fn send_native_formdata_request(
    url: &str,
    form_data: web_sys::FormData,
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>,
    on_success_cb: impl FnOnce() + 'static,
) {
    loading.set(true);
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&form_data);

    let request_promise = match get_window_fetch(Some(&mensaje), Some(&loading)) {
        Some(w) => w.fetch_with_str_and_init(url, &opts),
        None => return,
    };

    let msg_err = mensaje.clone();
    let load_err = loading.clone();
    let on_error = Closure::wrap(Box::new(move |_err: JsValue| {
        msg_err.set("Error al procesar archivo en el servidor".to_string());
        load_err.set(false);
    }) as Box<dyn FnMut(JsValue)>);

    let msg_resp = mensaje.clone();
    let load_resp = loading.clone();
    let mut on_success_opt = Some(on_success_cb);

    let on_success = Closure::wrap(Box::new(move |res: JsValue| {
        let response: Response = res.unchecked_into();
        load_resp.set(false);
        if response.status() == 200 {
            if let Some(cb) = on_success_opt.take() {
                cb();
            }
        } else {
            msg_resp.set(format!("Error guardando mapa: {}", response.status()));
        }
    }) as Box<dyn FnMut(JsValue)>);

    let then_fn: Function = Reflect::get(&request_promise, &JsValue::from_str("then")).unwrap().dyn_into().unwrap();
    let promise_2 = then_fn.call1(&request_promise, on_success.as_ref()).unwrap();
    let catch_fn: Function = Reflect::get(&promise_2, &JsValue::from_str("catch")).unwrap().dyn_into().unwrap();
    let _ = catch_fn.call1(&promise_2, on_error.as_ref());

    on_success.forget();
    on_error.forget();
}

// envía peticiones que descargan Blobs de Imágenes
pub fn send_native_blob_request(
    url: &str,
    body_json: &str,
    image_url: UseStateHandle<Option<String>>,
    mensaje: UseStateHandle<String>,
    loading: UseStateHandle<bool>,
) {
    loading.set(true);
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_credentials(web_sys::RequestCredentials::Include);
    opts.set_mode(RequestMode::Cors);
    
    let headers = web_sys::Headers::new().unwrap();
    headers.append("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);
    opts.set_body(&JsValue::from_str(body_json));

    let request_promise = match get_window_fetch(Some(&mensaje), Some(&loading)) {
        Some(w) => w.fetch_with_str_and_init(url, &opts),
        None => return,
    };

    let msg_err = mensaje.clone();
    let load_err = loading.clone();
    let on_error = Closure::wrap(Box::new(move |_err: JsValue| {
        msg_err.set("Error de renderizado gráfico de simulación".to_string());
        load_err.set(false);
    }) as Box<dyn FnMut(JsValue)>);

    let msg_resp = mensaje.clone();
    let load_resp = loading.clone();
    let img_resp = image_url.clone();

    let on_success = Closure::wrap(Box::new(move |res: JsValue| {
        let response: Response = res.unchecked_into();
        let msg = msg_resp.clone();
        let load = load_resp.clone();
        let img = img_resp.clone();

        if response.status() == 200 {
            let buffer_promise = response.array_buffer().unwrap();
            let on_buffer_ready = Closure::wrap(Box::new(move |buffer: JsValue| {
                let uint8_array = Uint8Array::new(&buffer);
                let array = Array::of1(&uint8_array.buffer());
                
                if let Ok(blob) = Blob::new_with_u8_array_sequence(&array) 
                    && let Ok(url) = Url::create_object_url_with_blob(&blob) {
                        if let Some(old_url) = (*img).clone() {
                            let _ = Url::revoke_object_url(&old_url);
                        }
                        img.set(Some(url));
                        msg.set(String::new());
                }
                
                load.set(false);
            }) as Box<dyn FnMut(JsValue)>);

            let then_fn: Function = Reflect::get(&buffer_promise, &JsValue::from_str("then")).unwrap().dyn_into().unwrap();
            let _ = then_fn.call1(&buffer_promise, on_buffer_ready.as_ref());
            on_buffer_ready.forget();
        } else {
            msg.set(format!("Error de simulación: {}", response.status()));
            load.set(false);
        }
    }) as Box<dyn FnMut(JsValue)>);

    let then_fn: Function = Reflect::get(&request_promise, &JsValue::from_str("then")).unwrap().dyn_into().unwrap();
    let promise_2 = then_fn.call1(&request_promise, on_success.as_ref()).unwrap();
    let catch_fn: Function = Reflect::get(&promise_2, &JsValue::from_str("catch")).unwrap().dyn_into().unwrap();
    let _ = catch_fn.call1(&promise_2, on_error.as_ref());

    on_success.forget();
    on_error.forget();
}