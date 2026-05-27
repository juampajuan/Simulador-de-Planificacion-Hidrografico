use yew::prelude::*; 
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use serde::{Serialize, Serializer};
use web_sys::{Url, Blob};
use js_sys::{Uint8Array, Array};


#[derive(Default, Clone, PartialEq, Serialize)]
pub struct PathState {
    #[serde(serialize_with = "serialize_str_to_f64")]
    pub separacion: String,
    #[serde(serialize_with = "serialize_str_to_f64")]
    pub azimut: String,
    pub gnss_type: String,
}

#[derive(Default, Clone, PartialEq, Serialize)]
pub struct EchoState {
    pub boat: String,
    #[serde(serialize_with = "serialize_str_to_f64")]
    pub max_limit: String,
    #[serde(serialize_with = "serialize_str_to_f64")]
    pub min_limit: String,
    #[serde(serialize_with = "serialize_str_to_usize")]
    pub pulse_repetition_interval: String,
    #[serde(serialize_with = "serialize_str_to_usize")]
    pub pulse_length: String,
    #[serde(serialize_with = "serialize_str_to_f64")]
    pub transmited_potency: String,
    #[serde(serialize_with = "serialize_str_to_f32")]
    pub gain: String,
    #[serde(serialize_with = "serialize_str_to_usize")]
    pub echosounder_velocity: String,
    #[serde(serialize_with = "serialize_str_to_f64")]
    pub umbral: String,
    pub uses_mathegapher: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,

    // Hay que ver esto cuando se decida que onda el back
    #[serde(skip_serializing)] // No lo mandamos directo, usamos el helper abajo
    pub frecuencia: String,
    pub uses_high_frecuency: bool,
    pub angle: f32,
}


fn serialize_str_to_f64<S>(value: &String, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    s.serialize_f64(value.parse::<f64>().unwrap_or(0.0))
}

fn serialize_str_to_f32<S>(value: &String, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    s.serialize_f32(value.parse::<f32>().unwrap_or(0.0))
}

fn serialize_str_to_usize<S>(value: &String, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    s.serialize_u64(value.parse::<usize>().unwrap_or(0) as u64)
}


pub fn trigger_path_generation(
    state: PathState, 
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
) {
    if state.separacion.is_empty() || state.azimut.is_empty() { return; }
    mensaje.set("Generando recorrido...".to_string());

    spawn_local(async move {
        if let Some(res) = post_json("http://localhost:3000/api/v1/create_path", &state, mensaje.clone()).await {
            let url = response_to_image_url(res).await;
            update_image_state(image_url, url);
            mensaje.set("Recorrido generado".to_string());
        }
    });
}

pub fn run_simulation(
    mut state: EchoState, 
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
) {
    mensaje.set("Iniciando simulación...".to_string());
    
    // Seteamos valores técnicos requeridos por el backend
    state.uses_high_frecuency = true;
    state.angle = 0.0;

    spawn_local(async move {
        if let Some(res) = post_json("http://localhost:3000/api/v1/run_simulation", &state, mensaje.clone()).await {
            let url = response_to_image_url(res).await;
            update_image_state(image_url, url);
            mensaje.set("Simulación completada".to_string());
        }
    });
}

async fn post_json<T: Serialize>(url: &str, data: &T, mensaje: UseStateHandle<String>) -> Option<gloo_net::http::Response> {
    let body = serde_json::to_string(data).ok()?;
    match Request::post(url).header("Content-Type", "application/json").body(body).unwrap().send().await {
        Ok(res) if res.status() == 200 => Some(res),
        Ok(res) => { mensaje.set(format!("Error {}: Server rechazó datos", res.status())); None },
        _ => { mensaje.set("Error de conexión".to_string()); None }
    }
}

async fn response_to_image_url(res: gloo_net::http::Response) -> String {
    let bytes = res.binary().await.unwrap();
    let array = Array::of1(&Uint8Array::from(bytes.as_slice()).buffer());
    let blob = Blob::new_with_u8_array_sequence(&array).unwrap();
    Url::create_object_url_with_blob(&blob).unwrap()
}

fn update_image_state(handle: UseStateHandle<Option<String>>, new_url: String) {
    if let Some(old_url) = (*handle).clone() {
        let _ = Url::revoke_object_url(&old_url);
    }
    handle.set(Some(new_url));
}