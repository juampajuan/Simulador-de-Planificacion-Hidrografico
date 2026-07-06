use tiny_http::Request;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use chrono::Local;
use crate::logging::structs::{ThreadMessage, LogType};
use crate::logging::logger::{send_message_to_logger,debug_logger};
use crate::db::queries_interface::projects;
use crate::db::queries_interface::student;
use crate::db::queries_interface::student_simulations;
use crate::structs::filecache::FileCache;
use crate::structs::request::{HandlerResult, FullSimulationRequest, RequestContext};
use crate::requests::http_helper::{parse_json_body, create_png_response};
use crate::requests::endpoints::generic;
use crate::requests::endpoints::generic::{string_response};
use crate::db::engine::DBEngine;
use crate::structs::settings::Settings;
use crate::utils::helpers_endpoints::check_student_auth;
use crate::utils::helpers::random_letters;
use common::{PathParameters, SimulationBase64Response};
use simulations::structs::depth_matrix::DepthMatrix;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::io::Cursor;

/// Endpoint para la creacion del recorrido. 
/// Utiliza contenidos de la cache y la db, si los hay.
pub fn create_path(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>, tx: &Sender<ThreadMessage>) -> HandlerResult {
    
    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx);
    
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    // reutilizamos la depthmatrix o la creamos
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.file_path, tx) {
        Ok(m) => m,
        Err(err) => return generic::server_error(err),
    };

    // reutilizamos el path o lo creamos
    let path = match lock_get_or_create_path(&cache, ctx.student_id, &matrix, &ctx.data.path_parameters, tx) {
        Ok(p) => p,
        Err(err) => return generic::server_error(err),
    };

    let image = simulations::create_path_image(&matrix, &path, &log_debug);
    let response = create_png_response(image);

    (response.boxed(), 200, None)
}

/// Endpoint para la simulacion/interpolacion. 
/// Utiliza recursos de la db y cache si los hay, sino debera tambien crear el path.
/// Revisa y compara la cantidad de intentos realizados. 
pub fn run_simulation(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>,tx: &Sender<ThreadMessage>) -> HandlerResult {
    
    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx);

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
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.file_path,tx) {
        Ok(m) => m,
        Err(err) => return generic::server_error(err),
    };

    // reutilizamos el path o lo creamos
    let path = match lock_get_or_create_path(&cache, ctx.student_id, &matrix, &ctx.data.path_parameters,tx) {
        Ok(p) => p,
        Err(err) => return generic::server_error(err),
    };

    if path.is_empty() {
        return generic::server_error("Error: El Recorrido (Path) está vacío.".to_string());
    }
    
    let interpolation = match simulations::run_simulation(&matrix, &path, echo_parameters, settings.simulation_constants(), &log_debug) {
        Ok(interp) => interp,
        Err(e) => return generic::server_error(e),
    };
    
    // aca cambia con respecto a las otras req que usan blob
    // los pixeles rgb se pasan a bytes png y luego a strings de base 64, para mandarlos en el struct
    let (map_image, min_depth, max_depth) = simulations::create_simulation_image(&matrix, &interpolation, &log_debug);
    let scale_image = simulations::create_scale_pure_image(&log_debug);

    let mut map_bytes = Vec::new();
    let mut scale_bytes = Vec::new();
    let _ = map_image.write_to(&mut Cursor::new(&mut map_bytes), image::ImageFormat::Png);
    let _ = scale_image.write_to(&mut Cursor::new(&mut scale_bytes), image::ImageFormat::Png);

    let db_lock = db.lock().unwrap();
    let attempt_number = student_simulations::get_next_attempt_number(&db_lock, ctx.student_id).unwrap_or(1);
    drop(db_lock);

    // Guardamos el PNG de simulacion en storage/images/, ademas de mandarlo en base64
    // (por ahora dejo las dos formas para que funcione la simulacion jajajja).
    let fecha = Local::now().format("%Y%m%d").to_string();
    let sufijo_random = random_letters(5);
    let map_filename = format!("{}_{}_simulacion_{}_{}.png", fecha, sufijo_random, ctx.student_id, attempt_number);
    let map_path = format!("{}/images/{}", settings.storage_path, map_filename);
    let map_saved = match std::fs::write(&map_path, &map_bytes) {
        Ok(()) => Some(map_filename.as_str()),
        Err(e) => {
            send_message_to_logger(tx, format!("No se pudo guardar el PNG de simulacion en storage ({}): {}", map_path, e), LogType::Error);
            None
        },
    };

    let map_encoded = STANDARD.encode(map_bytes);
    let scale_encoded = STANDARD.encode(scale_bytes); //base64


    if let Err(e) = student_simulations::create_student_simulation_locked(
        &db,
        ctx.student_id,
        ctx.project_id,
        attempt_number,
        min_depth,
        max_depth,
        &ctx.data.path_parameters,
        &echo_parameters.transport_parameters,
        &echo_parameters.echo_sounder_parameters,
        map_saved
    ) {
        send_message_to_logger(tx, format!("Error al registrar el intento en la DB para el alumno {}: {}", ctx.student_id, e), LogType::Error);
        
        return generic::server_error(
            "Error al guardar el intento de simulación".to_string()
        );
    }

    let response_data = SimulationBase64Response {
        min_depth,
        max_depth,
        map_base64: map_encoded,
        scale_base64: scale_encoded,
    };

    let json_payload = match serde_json::to_string(&response_data) {
        Ok(json) => json,
        Err(_) => return generic::server_error(
            "Error al serializar la respuesta de la simulación".to_string()
        ),
    };

    send_message_to_logger(tx, "Simulación completada con éxito.".to_string(), LogType::Info);
    if let Err(e) = crate::db::queries_interface::student::increment_student_attempts_locked(&db, ctx.student_id) {
        send_message_to_logger(tx, format!("Error al registrar el intento en la DB para el alumno {}: {}", ctx.student_id, e), LogType::Error);
        return generic::server_error("Error interno al registrar el intento".to_string());
    }
    string_response(json_payload, 200)
}

/// Genera la imagen de cubrimiento del recorrido con los parametros actuales. 
/// Sirve para ver de manera preliminar que areas cubre el recorrido. 
pub fn create_coverage_image(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>,tx: &Sender<ThreadMessage>) -> HandlerResult {
    
    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx);
    
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };
 
    let echo_parameters = match ctx.data.echo_parameters {
        Some(params) => params,
        None => return generic::server_error("Faltan parámetros de ecosonda".to_string()),
    };
 
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.file_path, tx) {
        Ok(m) => m,
        Err(err) => return generic::server_error(err),
    };
 
    let path = match lock_get_or_create_path(&cache, ctx.student_id, &matrix, &ctx.data.path_parameters,tx) {
        Ok(p) => p,
        Err(err) => return generic::server_error(err),
    };
 
    if path.is_empty() {
        return generic::server_error("Error: El Recorrido (Path) está vacío.".to_string());
    }
 
    let image = simulations::create_path_with_coverage(&matrix, &path, echo_parameters, settings.simulation_constants(), &log_debug);
    let response = create_png_response(image);
 
    send_message_to_logger(tx, "Imagen de cobertura generada.".to_string(), LogType::Debug);

    (response.boxed(), 200, None)
}
 

/// Toma los permisos y comprueba la existencia de alumno y proyecto en db. 
/// Extrae y prepara los datos esenciales de la request.
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

    let file_path = format!("{}/geotiffs/{}", settings.storage_path, project.filename);

    let data: FullSimulationRequest = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return Err(generic::server_error(format!("Bad Request: {}", err))),
    };

    Ok(RequestContext {
        file_path, 
        data, 
        student_id, 
        student, 
        project,
        project_id, 
    })
}

/// Genera la matrix a partir del archivo
/// Primero intenta obtenerla desde la cache, si no la encuentra, ejecuta el metodo para crearla.
/// Una vez creada, si no existia, la agrega al cache, para futuros usos.
fn lock_get_or_create_matrix(cache: &Arc<Mutex<FileCache>>, file_path: &str, tx: &Sender<ThreadMessage>) -> Result<DepthMatrix, String> {
    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx);

    let mut lock = match cache.lock() {
        Ok(l) => l,
        Err(_) => return Err("Error interno: no se pudo acceder al cache".to_string()),
    };

    // La nueva estructura busca mapas globalmente usando el file_path
    if let Some(m) = lock.get_map(file_path) {
        send_message_to_logger(tx, "Re-utilizando depth matrix del cache...".to_string(), LogType::Debug);
        return Ok(m.clone());
    }
    
    drop(lock);
    
    let m = match simulations::create_depth_matrix(file_path, &log_debug) {
        Ok(mat) => mat,
        Err(e) => return Err(e),
    };
    
    let mut relock = match cache.lock() {
        Ok(l) => l,
        Err(_) => return Err("Error interno: no se pudo acceder al cache".to_string()),
    };

    // Verificación de doble check ante concurrencia
    if let Some(existing_map) = relock.get_map(file_path) {
        return Ok(existing_map.clone());
    }

    relock.update_map(file_path.to_string(), m.clone());
    Ok(m)
}

/// Genera el recorrido a partir de una matrix profesada
/// Primero intenta obtenerlo desde la cache, si no lo encuentra, ejecuta el metodo para crearlo.
/// Una vez creado, lo agrega al cache, para futuros usos.
fn lock_get_or_create_path(cache: &Arc<Mutex<FileCache>>, cache_key: i64, matrix: &DepthMatrix, path_params: &PathParameters, tx: &Sender<ThreadMessage>) -> Result<Vec<(usize, usize)>, String> {
    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx);

    let mut lock = match cache.lock() {
        Ok(l) => l,
        Err(_) => return Err("Error interno: no se pudo acceder al cache".to_string()),
    };
    
    if let Some(path_coor) = lock.get_path_if_valid(cache_key, path_params) {
        send_message_to_logger(tx, "Re-utilizando path del cache...".to_string(), LogType::Debug);
        Ok(path_coor)
    } else {
        drop(lock);
        let p = simulations::create_path(
            matrix, 
            path_params.azimut, 
            path_params.separacion, 
            path_params.gnss_type,
            &log_debug
        );
        
        let mut lock = match cache.lock() {
            Ok(l) => l,
            Err(_) => return Err("Error interno: no se pudo acceder al cache (mutex corrupto)".to_string()),
        };
        lock.update_path(cache_key, p.clone(), path_params.clone());
        Ok(p)
    }
}