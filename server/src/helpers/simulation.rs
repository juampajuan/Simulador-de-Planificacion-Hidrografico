use tiny_http::Request;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::io::Cursor;
use chrono::Local;
use crate::logging::structs::{ThreadMessage, LogType};
use crate::logging::logger::{send_message_to_logger, debug_logger};
use crate::db::queries_interface::projects;
use crate::db::queries_interface::student;
use crate::structs::filecache::FileCache;
use crate::structs::request::{HandlerResult, FullSimulationRequest, RequestContext};
use crate::requests::http_helper::parse_json_body;
use crate::requests::endpoints::generic;
use crate::db::engine::DBEngine;
use crate::structs::settings::Settings;
use crate::utils::helpers_endpoints::check_student_auth;
use crate::utils::helpers::random_letters;
use common::{PathParameters, StudentMeasuringParameters};
use simulations::structs::depth_matrix::DepthMatrix;
use simulations::structs::simulation_constants::SimulationConstants;

/// Toma los permisos y comprueba la existencia de alumno y proyecto en db. 
/// Extrae y prepara los datos esenciales de la request.
pub fn extract_request_context(request: &mut Request, db: &Arc<Mutex<DBEngine>>, settings: &Arc<Settings>) -> Result<RequestContext, HandlerResult> {

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

/// Genera y guarda en storage/images/ las 3 imagenes de un intento de
/// simulacion: simulacion, cobertura y diferencias.
/// Las 3 comparten el mismo prefijo (fecha + letras al azar), y solo cambia la palabra del medio
/// segun el tipo.
pub fn save_simulation_images(
    matrix: &DepthMatrix,
    interpolation: &[Vec<f64>],
    path: &Vec<(usize, usize)>,
    params: StudentMeasuringParameters,
    constants: SimulationConstants,
    map_bytes: &[u8],
    settings: &Arc<Settings>,
    student_id: i64,
    attempt_number: i64,
    tx: &Sender<ThreadMessage>,
    log_debug: &dyn Fn(&str),
) -> (Option<String>, Option<String>, Option<String>) {

    let fecha = Local::now().format("%Y%m%d").to_string();
    let sufijo_random = random_letters(5);
    let base = format!("{}_{}", fecha, sufijo_random);

    let save = |categoria: &str, bytes: &[u8]| -> Option<String> {
        let filename = format!("{}_{}_{}_{}.png", base, categoria, student_id, attempt_number);
        let file_path = format!("{}/images/{}", settings.storage_path, filename);
        match std::fs::write(&file_path, bytes) {
            Ok(()) => Some(filename),
            Err(e) => {
                send_message_to_logger(tx, format!("No se pudo guardar el PNG de {} en storage ({}): {}", categoria, file_path, e), LogType::Error);
                None
            },
        }
    };

    let map_saved = save("simulacion", map_bytes);

    let (coverage_image, _min, _max) = simulations::create_simulation_with_coverage(matrix, interpolation, path, params, constants, log_debug);
    let mut coverage_bytes = Vec::new();
    let _ = coverage_image.write_to(&mut Cursor::new(&mut coverage_bytes), image::ImageFormat::Png);
    let coverage_saved = save("cobertura", &coverage_bytes);

    let difference_matrix = simulations::generate_difference_matrix(matrix, interpolation.to_vec(), &log_debug);
    let difference_image = simulations::create_difference_png(matrix, difference_matrix, &log_debug);
    let mut difference_bytes = Vec::new();
    let _ = difference_image.write_to(&mut Cursor::new(&mut difference_bytes), image::ImageFormat::Png);
    let difference_saved = save("diferencias", &difference_bytes);

    (map_saved, coverage_saved, difference_saved)
}

/// Genera la matrix a partir del archivo
/// Primero intenta obtenerla desde la cache, si no la encuentra, ejecuta el metodo para crearla.
/// Una vez creada, si no existia, la agrega al cache, para futuros usos.
pub fn lock_get_or_create_matrix(cache: &Arc<Mutex<FileCache>>, file_path: &str, tx: &Sender<ThreadMessage>, student_name: &str) -> Result<DepthMatrix, String> {

    let mut lock = match cache.lock() {
        Ok(l) => l,
        Err(_) => return Err("Error interno: no se pudo acceder al cache".to_string()),
    };

    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx, student_name);

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
pub fn lock_get_or_create_path(cache: &Arc<Mutex<FileCache>>, cache_key: i64, matrix: &DepthMatrix, path_params: &PathParameters, tx: &Sender<ThreadMessage>, student_name: &str) -> Result<Vec<(usize, usize)>, String> {
    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx, student_name);

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