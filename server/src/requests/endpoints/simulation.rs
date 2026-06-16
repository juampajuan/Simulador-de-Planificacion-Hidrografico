use tiny_http::Request;
use std::sync::{Arc, Mutex};
use crate::db::queries_interface::projects;
use crate::structs::filecache::FileCache;
use crate::structs::request::HandlerResult;
use crate::requests::http_helper::{parse_json_body, create_png_response};
use crate::requests::endpoints::generic;
use crate::requests::endpoints::auth::check_student_auth;
use crate::db::engine::DBEngine;
use crate::structs::settings::Settings;
use common::{StudentMeasuringParameters, PathParameters};
use simulations::structs::depth_matrix::DepthMatrix;

#[derive(serde::Deserialize)]
pub struct FullSimulationRequest {
    #[serde(default)]
    pub echo_parameters: Option<StudentMeasuringParameters>,
    pub path_parameters: PathParameters,
}

struct RequestContext {
    cache_key: String,
    file_path: String,
    data: FullSimulationRequest,
}

pub fn create_path(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>) -> HandlerResult {
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    // reutilizamos la depthmatrix o la creamos
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.cache_key, &ctx.file_path) {
        Ok(m) => m,
        Err(err) => return generic::server_error(err),
    };

    // reutilizamos el path o lo creamos
    let path = lock_get_or_create_path(&cache, &ctx.cache_key, &matrix, &ctx.data.path_parameters);

    let image = simulations::create_path_image(&matrix, &path);
    let response = create_png_response(image);

    (response.boxed(), 200)
}

pub fn run_simulation(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>) -> HandlerResult {
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    let echo_parameters = match ctx.data.echo_parameters {
        Some(params) => params,
        None => return generic::server_error("Faltan parámetros de ecosonda".to_string()),
    };

    // reutilizamos la depth matrix o la creamos
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.cache_key, &ctx.file_path) {
        Ok(m) => m,
        Err(err) => return generic::server_error(err),
    };

    // reutilizamos el path o lo creamos
    let path = lock_get_or_create_path(&cache, &ctx.cache_key, &matrix, &ctx.data.path_parameters);

    if path.is_empty() {
        return generic::server_error("Error: El Recorrido (Path) está vacío.".to_string());
    }
    
    let interpolation = simulations::run_simulation(&matrix, &path, echo_parameters.clone());
    let png_bytes = simulations::create_simulation_image(&matrix, &interpolation);

    let response = create_png_response(png_bytes);
    println!("Simulación completada con éxito.");
    
    (response.boxed(), 200)
}


fn extract_request_context(request: &mut Request, db: &Arc<Mutex<DBEngine>>, settings: &Arc<Settings>) -> Result<RequestContext, HandlerResult> {
    let student_id = match check_student_auth(request, db) {
        Ok(Some(id)) => id,
        Ok(None) => return Err(generic::string_response("Sin autorizar".to_string(), 401)),
        Err(err) => return Err(generic::server_error(err)),
    };

    let project_id = match projects::get_project_id_by_student_locked(db, student_id) {
        Ok(Some(id)) => id,
        Ok(None) => return Err(generic::string_response("Proyecto no encontrado".to_string(), 404)),
        Err(_) => return Err(generic::server_error("Error al obtener el proyecto del estudiante".to_string())),
    };

    let filename = match projects::get_project_by_id_locked(db, project_id) {
        Ok(Some(project)) => project.filename,
        Ok(None) => return Err(generic::string_response("Proyecto no encontrado".to_string(), 404)),
        Err(_) => return Err(generic::server_error("Error al obtener el proyecto".to_string())),
    };

    let file_path = format!("{}/geotiffs/{}", settings.upload_path, filename);

    let data: FullSimulationRequest = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return Err(generic::server_error(format!("Bad Request: {}", err))),
    };

    let cache_key = format!("{}-{}", student_id, project_id);

    Ok(RequestContext { cache_key, file_path, data })
}

// Reutilizan, o crean y guardan.
fn lock_get_or_create_matrix(cache: &Arc<Mutex<FileCache>>, cache_key: &str, file_path: &str) -> Result<DepthMatrix, String> {
    let mut lock = cache.lock().unwrap();
    if let Some(m) = lock.get_map(cache_key, file_path) {
        println!("Re-utilizando depth matrix del cache...");
        return Ok(m.clone());
    }
    drop(lock);
    
    let m = match simulations::create_depth_matrix(file_path) {
        Ok(mat) => mat,
        Err(_) => return Err("Error processing TIF (500)".to_string()),
    };
    
    let mut relock = cache.lock().unwrap();
    relock.update_map(cache_key.to_string(), file_path.to_string(), m.clone());
    Ok(m)
}

fn lock_get_or_create_path(cache: &Arc<Mutex<FileCache>>, cache_key: &str, matrix: &DepthMatrix, path_params: &PathParameters) -> Vec<(usize, usize)> {
    let mut lock = cache.lock().unwrap();
    
    if let Some(path_coor) = lock.get_path_if_valid(cache_key, path_params) {
        println!("Re-utilizando path del cache...");
        path_coor
    } else {
        drop(lock);
        
        println!("Calculando nuevo recorrido...");
        let p = simulations::create_path(
            matrix, 
            path_params.azimut, 
            path_params.separacion, 
            path_params.gnss_type
        );
        
        let mut lock = cache.lock().unwrap();
        lock.update_path(cache_key.to_string(), p.clone(), path_params.clone());
        p
    }
}