// pub static API_URL: &str = "http://localhost:8080/api/v1";
use yew::prelude::*; 
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;

// TODO: Hacer un metodo para las requests aca.
// Asi esta separado del resto, solamente lo llamas.
// pub fn handle_request(method: unEnum, url, parametros) -> Option<> 

use web_sys::{Url, Blob};
use js_sys::{Uint8Array, Array};

pub fn trigger_path_generation(
    separacion: String,
    azimut: String,
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
) {
    if separacion.is_empty() || azimut.is_empty() {
        return;
    }

    mensaje.set("Generando recorrido...".to_string());

    spawn_local(async move {
        let body = format!(
            r#"{{"separacion": {}, "azimut": {}}}"#,
            separacion, azimut
        );

        let res = Request::post("http://localhost:3000/api/v1/create_path")
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
            .send()
            .await;

        match res {
            Ok(response) => {
                let bytes = response.binary().await.unwrap();
                let uint8_array = Uint8Array::from(bytes.as_slice());
                let array = Array::new();
                array.push(&uint8_array.buffer());

                let blob = Blob::new_with_u8_array_sequence(&array).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();

                if let Some(old_url) = (*image_url).clone() {
                    let _ = Url::revoke_object_url(&old_url);
                }

                image_url.set(Some(url));
                mensaje.set("Imagen generada".to_string());
            }
            Err(_) => {
                mensaje.set("Error en el servidor".to_string());
            }
        }
    });
}