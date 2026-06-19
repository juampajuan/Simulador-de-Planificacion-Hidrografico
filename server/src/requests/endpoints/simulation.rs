use tiny_http::Request;
use std::sync::{Arc, Mutex};
use crate::db::queries_interface::projects;
use crate::db::queries_interface::student;
use crate::structs::filecache::FileCache;
use crate::structs::request::{HandlerResult, FullSimulationRequest, RequestContext};
use crate::requests::http_helper::{parse_json_body, create_png_response};
use crate::requests::endpoints::generic;
use crate::requests::endpoints::generic::{string_response};
use crate::db::engine::DBEngine;
use crate::structs::settings::Settings;
use crate::utils::helpers_endpoints::check_student_auth;
use common::{PathParameters, SimulationBase64Response};
use simulations::structs::depth_matrix::DepthMatrix;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::io::Cursor;

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

    (response.boxed(), 200, None)
}

pub fn run_simulation(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>) -> HandlerResult {
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    let limit = ctx.project.metadata.attempts_limit;
    if limit != -1 && ctx.student.attempts >= limit {
        return generic::string_response(
            "Has alcanzado el límite máximo de intentos permitidos para este proyecto.".to_string(), 
            403 
        );
    }

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
    
    // aca cambia con respecto a las otras req que usan blob
    // los pixeles rgb se pasan a bytes png y luego a strings de base 64, para mandarlos en el struct
    let (map_image, min_depth, max_depth) = simulations::create_simulation_image(&matrix, &interpolation);
    let scale_image = simulations::create_scale_pure_image();

    let mut map_bytes = Vec::new();
    let mut scale_bytes = Vec::new();
    let _ = map_image.write_to(&mut Cursor::new(&mut map_bytes), image::ImageFormat::Png);
    let _ = scale_image.write_to(&mut Cursor::new(&mut scale_bytes), image::ImageFormat::Png);

    let map_encoded = STANDARD.encode(map_bytes);
    let scale_encoded = STANDARD.encode(scale_bytes); //base64

    let response_data = SimulationBase64Response {
        min_depth,
        max_depth,
        map_base64: map_encoded,
        scale_base64: scale_encoded,
    };

    let json_payload = serde_json::to_string(&response_data).unwrap();
    println!("Simulación completada con éxito.");
    if let Err(e) = crate::db::queries_interface::student::increment_student_attempts_locked(&db, ctx.student_id) {
        eprintln!("Error al registrar el intento en la DB para el alumno {}: {}", ctx.student_id, e);
        return generic::server_error("Error interno al registrar el intento".to_string());
    }
    string_response(json_payload, 200)
}

pub fn create_coverage_image(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>) -> HandlerResult {
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };
 
    let echo_parameters = match ctx.data.echo_parameters {
        Some(params) => params,
        None => return generic::server_error("Faltan parámetros de ecosonda".to_string()),
    };
 
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.cache_key, &ctx.file_path) {
        Ok(m) => m,
        Err(err) => return generic::server_error(err),
    };
 
    let path = lock_get_or_create_path(&cache, &ctx.cache_key, &matrix, &ctx.data.path_parameters);
 
    if path.is_empty() {
        return generic::server_error("Error: El Recorrido (Path) está vacío.".to_string());
    }
 
    let image = simulations::create_path_with_shadows(&matrix, &path, echo_parameters);
    let response = create_png_response(image);
 
    println!("Imagen de cobertura generada.");
    (response.boxed(), 200, None)
}
 


fn extract_request_context(request: &mut Request, db: &Arc<Mutex<DBEngine>>, settings: &Arc<Settings>) -> Result<RequestContext, HandlerResult> {
    let student_id = match check_student_auth(request, db) {
        Ok(Some(id)) => id,
        Ok(None) => return Err(generic::string_response("Sin autorizar".to_string(), 401)),
        Err(err) => return Err(generic::server_error(err)),
    };

    let student = match student::get_student_by_id_locked(db, student_id) {
        Ok(Some(s)) => s,
        Ok(None) => return Err(generic::string_response("Estudiante no encontrado".to_string(), 404)),
        Err(_) => return Err(generic::server_error("Error al obtener datos del estudiante".to_string())),
    };

    let project_id = match projects::get_project_id_by_student_locked(db, student_id) {
        Ok(Some(id)) => id,
        Ok(None) => return Err(generic::string_response("Proyecto no encontrado".to_string(), 404)),
        Err(_) => return Err(generic::server_error("Error al obtener el proyecto del estudiante".to_string())),
    };

    let project = match projects::get_project_by_id_locked(db, project_id) {
        Ok(Some(project)) => project,
        Ok(None) => return Err(generic::string_response("Proyecto no encontrado".to_string(), 404)),
        Err(_) => return Err(generic::server_error("Error al obtener el proyecto".to_string())),
    };

    let file_path = format!("{}/geotiffs/{}", settings.upload_path, project.filename);

    let data: FullSimulationRequest = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return Err(generic::server_error(format!("Bad Request: {}", err))),
    };

    let cache_key = format!("{}-{}", student_id, project_id);

    Ok(RequestContext { 
        cache_key, 
        file_path, 
        data, 
        student_id, 
        student, 
        project 
    })
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