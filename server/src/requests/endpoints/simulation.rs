use tiny_http::{Response, Request};
use std::sync::{Arc, Mutex};
use crate::structs::filecache::{FileCache, DepthMatrix};
use crate::structs::request::{HandlerResult};
use simulations;
use std::io::Cursor;
use image::ImageFormat;
use simulations::structs::echosonder::EcosondaMode;

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
    let mut content = String::new();
    if let Err(_) = request.as_reader().read_to_string(&mut content) {
        return (Response::from_string("Error reading body").with_status_code(400).boxed(), 400);
    }

    let data: PathRequest = match serde_json::from_str(&content) {
        Ok(d) => d,
        Err(_) => return (Response::from_string("Invalid JSON").with_status_code(400).boxed(), 400),
    };

    let tif_id = "Darsena_20cm_v2.tif";
 
    // Llamada a la crate externa
    let matrix = match simulations::create_depth_matrix(tif_id) {
        Ok(m) => m,
        Err(_) => return (Response::from_string("Error processing TIF").with_status_code(500).boxed(), 500),
    };

    let path = simulations::create_path(&matrix, data.azimut, data.separacion, data.gnss_type);
    
    // Guardar en el cache
    {
        let mut lock = match cache.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        lock.update_path(tif_id.to_string(), matrix.clone(), path.clone());
    }

    let image = simulations::create_path_image(&matrix, &path);
    let mut bytes = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png).expect("Failed to encode PNG");

    let response = Response::from_data(bytes)
        .with_status_code(200)
        .with_header(tiny_http::Header::from_bytes("Content-Type", "image/png").unwrap())
        .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap());

    (response.boxed(), 200)
}

pub fn run_simulation(request: &mut Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {
    let mut content = String::new();
    if let Err(_) = request.as_reader().read_to_string(&mut content) {
        return (Response::from_string("Error leyendo body").with_status_code(400).boxed(), 400);
    }

// 1. Parsear el JSON (lo que ya tenés)
    let data: SimulationRequest = match serde_json::from_str(&content) {
        Ok(d) => d,
        Err(e) => {
            println!("Error JSON: {}", e);
            return (Response::from_string("JSON inválido").with_status_code(400).boxed(), 400);
        }
    };

    // 2. Acceder al caché de forma segura (MANEJO DE POISON ERROR)
    let mut lock = match cache.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            println!("Mutex envenenado por un panic previo. Recuperando acceso...");
            poisoned.into_inner()
        }
    };
    
    let tif_id = "Darsena_20cm_v2.tif"; 

    let cached_item = match lock.get(tif_id) {
        Some(item) => item,
        None => {
            return (
                Response::from_string("Error: No se encontró la matriz. Genere el recorrido primero.")
                    .with_status_code(404)
                    .boxed()
                    .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap()), 
                404
            );
        }
    };

// 3. Clonar y limpiar el recorrido (Path)
    let matrix = &cached_item.matrix;
    let mut path = cached_item.last_path.clone(); 
    
    // Eliminamos duplicados (esto despues debo llevarlo al back)
    path.dedup(); 

    if path.is_empty() {
        let resp = Response::from_string("Error: El recorrido está vacío")
            .with_status_code(400)
            .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap());
        return (resp.boxed(), 400);
    }
    // 4. Determinar el modo de la ecosonda
    let mode = if data.uses_high_frecuency {
        EcosondaMode::Multihaz
    } else {
        EcosondaMode::Monohaz
    };

// 5. Ejecutar la simulación pesada
    println!("Iniciando simulación para: {}", tif_id);
    
    // Importante: El pánico ocurre DENTRO de esta función.
    // Al haber limpiado el path arriba, ya no debería suceder.
    let interpolation = simulations::run_simulation(
        matrix, 
        &path, 
        20.0, 
        mode
    );

    // 6. Generar la imagen PNG de la simulación
    let rgb_image = simulations::create_simulation_image(matrix, &interpolation);

    let mut image_bytes: Vec<u8> = Vec::new();
    if let Err(e) = rgb_image.write_to(&mut Cursor::new(&mut image_bytes), ImageFormat::Png) {
        println!("Error generando PNG: {}", e);
        return (Response::from_string("Error generando imagen").with_status_code(500).boxed(), 500);
    }

    // 7. Responder con la imagen y los HEADERS DE CORS (Vital para que no falle el navegador)
    let response = Response::from_data(image_bytes)
        .with_status_code(200)
        .with_header(tiny_http::Header::from_bytes("Content-Type", "image/png").unwrap())
        .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap())
        .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Methods", "POST, OPTIONS").unwrap())
        .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Headers", "Content-Type").unwrap());

    println!("Simulación completada con éxito.");
    (response.boxed(), 200)
}