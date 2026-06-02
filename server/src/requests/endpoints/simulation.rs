use tiny_http::{Request};
use std::sync::{Arc, Mutex};
use crate::structs::filecache::FileCache;
use crate::structs::request::HandlerResult;
use crate::requests::http_helper::{parse_json_body, create_png_response};
use crate::requests::endpoints::errors;
use common::{StudentMeasuringParameters, PathParameters};

const TIF_ID: &str = "Darsena_20cm_v2.tif";

pub fn create_path(request: &mut Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {
    // Parseo y obtencion de pathParameters
    let data: PathParameters = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return errors::server_error(format!("Bad Request: {}", err)),
    };

    // Creacion de matriz de profundidad con simulations
    let matrix = match simulations::create_depth_matrix(TIF_ID) {
        Ok(m) => m,
        Err(_) => return errors::server_error("Error processing TIF (500)".to_string()),
    };

    let path = simulations::create_path(&matrix, data.azimut, data.separacion, data.gnss_type);
    
    // Actualizacion del cache para usar el path en run_simulation.
    {
        let mut lock = match cache.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        lock.update_path(TIF_ID.to_string(), matrix.clone(), path.clone());
    }

    // Se crea la imagen y la response.
    let image = simulations::create_path_image(&matrix, &path);
    let response = create_png_response(image);

    (response.boxed(), 200)
}

pub fn run_simulation(request: &mut Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {
    // Parseo y obtencion de studentMeasuringParameters
    let data: StudentMeasuringParameters = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return errors::server_error(format!("Bad Request: {}", err)),
    };

    // recuperamos la matriz y el path calculados previamente
    let mut lock = match cache.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };
    
    let cached_item = match lock.get(TIF_ID) {
        Some(item) => item,
        // Si no se encuentra, usamos el not_found de tu compañero
        None => return errors::not_found(),
    };

    let matrix = &cached_item.matrix;
    let path = &cached_item.last_path; 

    if path.is_empty() {
        return errors::server_error("Bad Request: Path vacío".to_string());
    }
    
    // hacemos la interpolacion y la imagen de la simulacion
    let interpolation = simulations::run_simulation(matrix, path, data);
    let rgb_image = simulations::create_simulation_image(matrix, &interpolation);

    let response = create_png_response(rgb_image);
    
    println!("Simulación completada.");
    (response.boxed(), 200)
}