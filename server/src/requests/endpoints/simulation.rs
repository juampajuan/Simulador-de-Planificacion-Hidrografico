use tiny_http::{Request};
use std::sync::{Arc, Mutex};
use crate::structs::filecache::FileCache;
use crate::structs::request::HandlerResult;
use crate::requests::http_helper::{parse_json_body, create_png_response};
use crate::requests::endpoints::generic;
use common::{StudentMeasuringParameters, PathParameters};

const TIF_ID: &str = "Darsena_20cm_v2.tif";
#[derive(serde::Deserialize)]
pub struct FullSimulationRequest {
    #[serde(default)]
    pub echo_parameters: Option<StudentMeasuringParameters>,
    pub path_parameters: PathParameters,
} // Dejo el option para lo de measure porque puede ser que se llame a create_path sin echo_parameters.
// Si eso pasa, deja el echo en default, y continua.

pub fn create_path(request: &mut Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {
    // Parseo y obtencion de pathParameters
    let data: FullSimulationRequest = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return generic::server_error(format!("Bad Request: {}", err)),
    };

    // Creacion de matriz de profundidad con simulations
    let matrix = match simulations::create_depth_matrix(TIF_ID) {
        Ok(m) => m,
        Err(_) => return generic::server_error("Error processing TIF (500)".to_string()),
    };

    let path = simulations::create_path(&matrix, data.path_parameters.azimut, data.path_parameters.separacion, data.path_parameters.gnss_type);
    
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
    let data: FullSimulationRequest = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return generic::server_error(format!("Bad Request: {}", err)),
    };

    let mut lock = match cache.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };

    let (matrix, path) = match lock.get(TIF_ID) {
        Some(item) => (item.matrix.clone(), item.last_path.clone()),
        None => {
            println!("Caché vacío. No existe el TIF en el cache...");
            
            let generated_matrix = match simulations::create_depth_matrix(TIF_ID) {
                Ok(m) => m,
                Err(_) => return generic::server_error("Error crítico procesando TIF".to_string()),
            };
            
            let generated_path = simulations::create_path(
                &generated_matrix, 
                data.path_parameters.azimut, 
                data.path_parameters.separacion, 
                data.path_parameters.gnss_type
            );

            lock.update_path(TIF_ID.to_string(), generated_matrix.clone(), generated_path.clone());
            
            (generated_matrix, generated_path)
        }
    };

    if path.is_empty() {
        return generic::server_error("Error: El Path sigue estando vacío".to_string());
    }

    let echo_parameters = match data.echo_parameters {
        Some(params) => params,
        None => return generic::server_error("Faltan parámetros de ecosonda".to_string()),
    };
    
    let interpolation = simulations::run_simulation(&matrix, &path, echo_parameters);
    let rgb_image = simulations::create_simulation_image(&matrix, &interpolation);

    let response = create_png_response(rgb_image);
    
    println!("Simulación completada.");
    (response.boxed(), 200)
}