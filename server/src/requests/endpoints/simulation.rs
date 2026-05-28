use tiny_http::{Response, Request};
use std::sync::{Arc, Mutex};
use crate::structs::filecache::{FileCache, DepthMatrix};
use crate::structs::request::{HandlerResult};
use simulations;
use std::io::Cursor;
use image::ImageFormat;

use serde::Deserialize;

#[derive(Deserialize)]
struct PathRequest {
    separacion: f64,
    azimut: f64,
    gnss_type: String,
}

#[derive(Deserialize)]
struct SimulationRequest {
    uses_mathegapher: bool,
    uses_sound_profiler: bool,
    uses_inertial_sensor: bool,
    max_limit: f64,
    min_limit: f64,
    pulse_repetition_interval: usize,
    pulse_length: usize,
    uses_high_frecuency: bool,
    angle: f32,
    transmited_potency: f64,
    gain: f32,
    echosounder_velocity: usize,
    boat: String,
}
 
pub fn create_path(request: &mut Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {

    // Podes leer todo, pero no respondas aca. La respuesta, la creas y la retornas
    // Leo los params de requests


    let mut content = String::new();

    request.as_reader()
        .read_to_string(&mut content)
        .unwrap();

    println!("{}", content);

    let data = match serde_json::from_str::<PathRequest>(&content) {
        Ok(data) => data,
        Err(e) => {
            println!("Error al parsear JSON: {:?}", e);
            let response = Response::from_string("Invalid JSON")
                .with_status_code(400);
            return (response.boxed(), 400);
        }
    };

    // Chequeo si esta en el cache
    // let mut cache = cache.lock().unwrap();
    // cache.add(DepthMatrix { id: 1 });
    // println!("{:?}", cache.get(1));

    let tif = "Darsena_20cm_v2.tif";
 
    // Si no esta, la creo
    let matrix = simulations::create_depth_matrix(tif);

    let matrix = match matrix {
        Ok(matrix) => matrix,
        Err(e) => {
            println!("Error al crear la matriz de profundidad: {:?}", e);
            let response = Response::from_string("Error al procesar el archivo de profundidad")
                .with_status_code(500);
            return (response.boxed(), 500);
        }
    };

    let path = simulations::create_path(&matrix, data.azimut, data.separacion, data.gnss_type);
    
    let image =
        simulations::create_path_image(&matrix, &path);

    let mut bytes: Vec<u8> = Vec::new();

    image.write_to(
        &mut Cursor::new(&mut bytes),
        ImageFormat::Png
    ).unwrap();

    let response = Response::from_data(bytes)
        .with_status_code(200)
        .with_header(
            tiny_http::Header::from_bytes(
                "Content-Type",
                "image/png"
            ).unwrap()
        )
        .with_header(
            tiny_http::Header::from_bytes(
                "Access-Control-Allow-Origin",
                "*"
            ).unwrap()
        );

    (response.boxed(), 200)
}

pub fn run_simulation(request: &mut Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {

    // Podes leer todo, pero no respondas aca. La respuesta, la creas y la retornas
    // Leo los params de requests
 
    // Si no esta en cache, la creo
    // let matrix = simulations::create_depth_matrix(/* paso el path o algo */);

    // simulations::create_path(/* paramas */);
    // simulations::run_simulation(/* paramas */);
    // simulations::create_simulation_image();
 
    
    
    
    
    let mut content = String::new();

    request.as_reader()
    .read_to_string(&mut content)
        .unwrap();
    
    println!("{}", content);
    
    let data = match serde_json::from_str::<SimulationRequest>(&content) {
        Ok(data) => data,
        Err(e) => {
            println!("Error al parsear JSON: {:?}", e);
            let response = Response::from_string("Invalid JSON")
            .with_status_code(400);
        return (response.boxed(), 400);
        }
    };
    let response = Response::from_string(format!("Recibí la simulación con los siguientes parámetros: uses_mathegapher: {}, uses_sound_profiler: {}, uses_inertial_sensor: {}, max_limit: {}, min_limit: {}, pulse_repetition_interval: {}, pulse_length: {}, uses_high_frecuency: {}, angle: {}, transmited_potency: {}, gain: {}, echosounder_velocity: {}, boat: {}",
        data.uses_mathegapher,
        data.uses_sound_profiler,
        data.uses_inertial_sensor,
        data.max_limit,
        data.min_limit,
        data.pulse_repetition_interval,
        data.pulse_length,
        data.uses_high_frecuency,
        data.angle,
        data.transmited_potency,
        data.gain,
        data.echosounder_velocity,
        data.boat
    ))
    .with_status_code(200);

    (response.boxed(), 200)
}

 