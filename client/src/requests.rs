use yew::prelude::*; 
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use serde::Serialize;
use web_sys::{Url, Blob};
use js_sys::{Uint8Array, Array};

// --- IMPORTANTE: Usamos los tipos de la verdad única ---
use common::{
    StudentMeasuringParameters, 
    EchosounderParameters, 
    Boat, 
    PathParameters, 
    GnssType
};

#[derive(Default, Clone, PartialEq, Serialize)]
pub struct PathState {
    pub separacion: String,
    pub azimut: String,
    pub gnss_type: String,
}

#[derive(Default, Clone, PartialEq, Serialize)]
pub struct EchoState {
    pub boat: String,
    pub max_limit: String,
    pub min_limit: String,
    pub pulse_repetition_interval: String,
    pub pulse_length: String,
    pub transmited_potency: String,
    pub gain: String,
    pub echosounder_velocity: String,
    pub umbral: String,
    pub uses_mathegapher: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,
    pub uses_high_frecuency: bool,
    pub angle: f32,
}

// --- GENERACIÓN DE RECORRIDO ---

pub fn trigger_path_generation(
    state: PathState, 
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
) {
    if state.separacion.is_empty() || state.azimut.is_empty() { return; }
    mensaje.set("Generando recorrido...".to_string());

    // Mapeo a Common: Convertimos Strings a Tipos Reales y Enums
    let params = PathParameters {
        separacion: state.separacion.parse().unwrap_or(0.0),
        azimut: state.azimut.parse().unwrap_or(0.0),
        gnss_type: match state.gnss_type.as_str() {
            "Fase" => GnssType::PhaseCorrection,
            "DGPS" => GnssType::DGPSCorrection,
            _ => GnssType::NoCorrection,
        },
    };

    spawn_local(async move {
        if let Some(res) = post_json("http://localhost:3000/api/v1/create_path", &params, mensaje.clone()).await {
            let url = response_to_image_url(res).await;
            update_image_state(image_url, url);
            mensaje.set("Recorrido generado".to_string());
        }
    });
}

// --- EJECUCIÓN DE SIMULACIÓN ---

pub fn run_simulation(
    state: EchoState, 
    mensaje: UseStateHandle<String>,
    image_url: UseStateHandle<Option<String>>,
) {
    mensaje.set("Iniciando simulación...".to_string());
    
    // Mapeo a Common: Construimos la estructura anidada y los Enums con variantes
    let boat_speed = 0.005; // Valor base o parseado si lo agregas al UI

    let params = StudentMeasuringParameters {
        uses_mathegapher: state.uses_mathegapher,
        uses_sound_profiler: state.uses_sound_profiler,
        uses_inertial_sensor: state.uses_inertial_sensor,
        // Aquí resolvemos el error "expected struct variant"
        boat: match state.boat.as_str() {
            "Y" => Boat::Y { speed: boat_speed },
            _ => Boat::W { speed: boat_speed },
        },
        echo_sounder_parameters: EchosounderParameters {
            uses_monohaz: !state.uses_high_frecuency,
            mode: None, // El backend lo inicializa con .create_echosounder()
            max_limit: state.max_limit.parse().unwrap_or(0.0),
            min_limit: state.min_limit.parse().unwrap_or(0.0),
            pulse_repetition_interval: state.pulse_repetition_interval.parse().unwrap_or(0.0),
            pulse_length: state.pulse_length.parse().unwrap_or(0),
            uses_high_frecuency: state.uses_high_frecuency,
            transmited_potency: state.transmited_potency.parse().unwrap_or(0.0),
            gain: state.gain.parse().unwrap_or(0.0),
            echosounder_velocity: state.echosounder_velocity.parse().unwrap_or(0),
            threshold: state.umbral.parse().unwrap_or(0.0),
        },
    };

    spawn_local(async move {
        if let Some(res) = post_json("http://localhost:3000/api/v1/run_simulation", &params, mensaje.clone()).await {
            let url = response_to_image_url(res).await;
            update_image_state(image_url, url);
            mensaje.set("Simulación completada".to_string());
        }
    });
}

// --- HELPERS (Sin cambios, pero ahora reciben tipos de Common) ---

async fn post_json<T: Serialize>(url: &str, data: &T, mensaje: UseStateHandle<String>) -> Option<gloo_net::http::Response> {
    let body = serde_json::to_string(data).ok()?;
    match Request::post(url)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
        .send()
        .await 
    {
        Ok(res) if res.status() == 200 => Some(res),
        Ok(res) => { 
            mensaje.set(format!("Error {}: Datos incompatibles", res.status())); 
            None 
        },
        _ => { 
            mensaje.set("Error de conexión".to_string()); 
            None 
        }
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