use tiny_http::{Request};
use std::sync::{Arc, Mutex};
use crate::db::queries_interface::projects;
use crate::structs::filecache::FileCache;
use crate::structs::request::HandlerResult;
use crate::requests::http_helper::{parse_json_body, create_png_response};
use crate::requests::endpoints::generic;
use crate::db::engine::DBEngine;
use crate::structs::settings::Settings;
use crate::utils::helpers_endpoints::check_student_auth;
use common::{StudentMeasuringParameters, PathParameters};
 
#[derive(serde::Deserialize)]
pub struct FullSimulationRequest {
    #[serde(default)]
    pub echo_parameters: Option<StudentMeasuringParameters>,
    pub path_parameters: PathParameters,
} // Dejo el option para lo de measure porque puede ser que se llame a create_path sin echo_parameters.
// Si eso pasa, deja el echo en default, y continua.

pub fn create_path(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>) -> HandlerResult {

    // TODO: Es muy grande la funcion, capaz lo mejor seria mover este pedazo en una funcion aparte 
    // que obtenga el path del tif a partir del id del estudiante.
    // -------------------------------------------------------------------------------------------------------
    let student_id = match check_student_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return generic::string_response("Sin autorizar".to_string(), 401),
        Err(err) => return generic::server_error(err),
    };

    let project_id = match projects::get_project_id_by_student_locked(&db, student_id) {
        Ok(Some(id)) => id,
        Ok(None) => return generic::string_response("Proyecto no encontrado".to_string(), 404),
        Err(_) => return generic::server_error("Error al obtener el proyecto del estudiante".to_string()),
    };

    let filename = match projects::get_project_by_id_locked(&db, project_id) {
        Ok(Some(project)) => project.filename,
        Ok(None) => return generic::string_response("Proyecto no encontrado".to_string(), 404),
        Err(_) => return generic::server_error("Error al obtener el proyecto".to_string()),
    };

    let file_path = format!("{}/geotiffs/{}", settings.upload_path, filename);
    // -------------------------------------------------------------------------------------------------------

    println!("filename {}", filename);

    // Parseo y obtencion de pathParameters
    let data: FullSimulationRequest = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return generic::server_error(format!("Bad Request: {}", err)),
    };

    // Creacion de matriz de profundidad con simulations
    let matrix = match simulations::create_depth_matrix(&file_path) {
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
        lock.update_path(file_path.clone(), matrix.clone(), path.clone());
    }

    // Se crea la imagen y la response.
    let image = simulations::create_path_image(&matrix, &path);
    let response = create_png_response(image);

    (response.boxed(), 200)
}


pub fn run_simulation(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>) -> HandlerResult {

    let student_id = match check_student_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return generic::string_response("Sin autorizar".to_string(), 401),
        Err(err) => return generic::server_error(err),
    };

    let project_id = match projects::get_project_id_by_student_locked(&db, student_id) {
        Ok(Some(id)) => id,
        Ok(None) => return generic::string_response("Proyecto no encontrado".to_string(), 404),
        Err(_) => return generic::server_error("Error al obtener el proyecto del estudiante".to_string()),
    };

    let filename = match projects::get_project_by_id_locked(&db, project_id) {
        Ok(Some(project)) => project.filename,
        Ok(None) => return generic::string_response("Proyecto no encontrado".to_string(), 404),
        Err(_) => return generic::server_error("Error al obtener el proyecto".to_string()),
    };

    let file_path = format!("{}/geotiffs/{}", settings.upload_path, filename);
    // -------------------------------------------------------------------------------------------------------

    let data: FullSimulationRequest = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return generic::server_error(format!("Bad Request: {}", err)),
    };

    let mut lock = match cache.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };

    let (matrix, path) = match lock.get(&file_path) {
        Some(item) => (item.matrix.clone(), item.last_path.clone()),
        None => {
            println!("Caché vacío. No existe el TIF en el cache...");
            
            let generated_matrix = match simulations::create_depth_matrix(&file_path) {
                Ok(m) => m,
                Err(_) => return generic::server_error("Error crítico procesando TIF".to_string()),
            };
            
            let generated_path = simulations::create_path(
                &generated_matrix, 
                data.path_parameters.azimut, 
                data.path_parameters.separacion, 
                data.path_parameters.gnss_type
            );

            lock.update_path(file_path.clone(), generated_matrix.clone(), generated_path.clone());
            
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