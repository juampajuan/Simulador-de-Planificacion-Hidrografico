use tiny_http::{Response, Request};
use std::sync::{Arc, Mutex};
use crate::structs::filecache::{FileCache};
use crate::structs::request::{HandlerResult};
use std::io::Cursor;
use image::ImageFormat;

// 1. Importamos los tipos de la verdad única (common)
use common::{StudentMeasuringParameters, PathParameters};

// --- CREATE PATH ---

pub fn create_path(request: &mut Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {
    let mut content = String::new();
    if let Err(_) = request.as_reader().read_to_string(&mut content) {
        return (Response::from_string("Error reading body").with_status_code(400).boxed(), 400);
    }

    // Usamos PathParameters de common
    let data: PathParameters = match serde_json::from_str(&content) {
        Ok(d) => d,
        Err(_) => return (Response::from_string("Invalid JSON").with_status_code(400).boxed(), 400),
    };

    let tif_id = "Darsena_20cm_v2.tif";
 
    let matrix = match simulations::create_depth_matrix(tif_id) {
        Ok(m) => m,
        Err(_) => return (Response::from_string("Error processing TIF").with_status_code(500).boxed(), 500),
    };

    // Llamamos a simulations con el enum de common
    let path = simulations::create_path(&matrix, data.azimut, data.separacion, data.gnss_type);
    
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

// --- RUN SIMULATION ---

pub fn run_simulation(request: &mut Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {
    let mut content = String::new();
    if let Err(_) = request.as_reader().read_to_string(&mut content) {
        return (Response::from_string("Error leyendo body").with_status_code(400).boxed(), 400);
    }

    // 1. Usamos StudentMeasuringParameters de common directamente
    let data: StudentMeasuringParameters = match serde_json::from_str(&content) {
        Ok(d) => d,
        Err(e) => {
            println!("Error JSON (common migration): {}", e);
            return (Response::from_string("JSON inválido").with_status_code(400).boxed(), 400);
        }
    };

    // 2. Acceder al caché
    let mut lock = match cache.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };
    
    let tif_id = "Darsena_20cm_v2.tif"; 

    let cached_item = match lock.get(tif_id) {
        Some(item) => item,
        None => {
            return (
                Response::from_string("Error: No se encontró la matriz.")
                    .with_status_code(404)
                    .boxed()
                    .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap()), 
                404
            );
        }
    };

    let matrix = &cached_item.matrix;
    let path = &cached_item.last_path; 

    if path.is_empty() {
        return (Response::from_string("Path vacío").with_status_code(400).boxed(), 400);
    }

    println!("Iniciando simulación con parámetros de common para el barco {:?}", data.boat);
    
    // 3. Ejecutar simulación pasando el struct completo de common
    let interpolation = simulations::run_simulation(
        matrix, 
        path, 
        data // Pasamos el struct de common
    );

    // 4. Generar la imagen
    let rgb_image = simulations::create_simulation_image(matrix, &interpolation);

    let mut image_bytes: Vec<u8> = Vec::new();
    rgb_image.write_to(&mut Cursor::new(&mut image_bytes), ImageFormat::Png).unwrap();

    let response = Response::from_data(image_bytes)
        .with_status_code(200)
        .with_header(tiny_http::Header::from_bytes("Content-Type", "image/png").unwrap())
        .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap())
        .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Methods", "POST, OPTIONS").unwrap())
        .with_header(tiny_http::Header::from_bytes("Access-Control-Allow-Headers", "Content-Type").unwrap());

    println!("Simulación completada.");
    (response.boxed(), 200)
}