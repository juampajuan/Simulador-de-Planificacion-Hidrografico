use tiny_http::Request;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::logging::structs::{ThreadMessage, LogType};
use crate::logging::logger::{send_message_to_logger,debug_logger};
use crate::db::queries_interface::student_simulations;
use crate::structs::filecache::FileCache;
use crate::structs::request::HandlerResult;
use crate::requests::http_helper::create_png_response;
use crate::requests::endpoints::generic;
use crate::requests::endpoints::generic::{string_response};
use crate::helpers::simulation::{extract_request_context, save_simulation_images, lock_get_or_create_matrix, lock_get_or_create_path};
use crate::db::engine::DBEngine;
use crate::structs::settings::Settings;
use common::SimulationBase64Response;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::io::Cursor;

/// Endpoint para la creacion del recorrido. 
/// Utiliza contenidos de la cache y la db, si los hay.
pub fn create_path(request: &mut Request, cache: Arc<Mutex<FileCache>>, db: Arc<Mutex<DBEngine>>, settings: Arc<Settings>, tx: &Sender<ThreadMessage>) -> HandlerResult {
    
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx, &ctx.student.name);

    // reutilizamos la depthmatrix o la creamos
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.file_path, tx, &ctx.student.name) {
        Ok(m) => m,
        Err(err) => return generic::server_error(err),
    };

    // reutilizamos el path o lo creamos
    let path = match lock_get_or_create_path(&cache, ctx.student_id, &matrix, &ctx.data.path_parameters, tx, &ctx.student.name) {
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
    
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx, &ctx.student.name);

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
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.file_path,tx, &ctx.student.name) {
        Ok(m) => m,
        Err(err) => return generic::server_error(err),
    };

    // reutilizamos el path o lo creamos
    let path = match lock_get_or_create_path(&cache, ctx.student_id, &matrix, &ctx.data.path_parameters,tx, &ctx.student.name) {
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

    // TODO: Usar el wrapper, ver como hice el resto.
    let db_lock = db.lock().unwrap();
    let attempt_number = student_simulations::get_next_attempt_number(&db_lock, ctx.student_id).unwrap_or(1);
    drop(db_lock);

    // Genera y guarda en storage/images/ las 3 imagenes del intento (mapa,
    // cobertura, diferencias)
    let (map_saved, coverage_saved, difference_saved) = save_simulation_images(
        &matrix,
        &interpolation,
        &path,
        echo_parameters,
        settings.simulation_constants(),
        &map_bytes,
        &settings,
        ctx.student_id,
        attempt_number,
        tx,
        &log_debug,
    );

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
        map_saved.as_deref(),
        coverage_saved.as_deref(),
        difference_saved.as_deref(),
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
        simulation_image_path: map_saved,
        coverage_image_path: coverage_saved,
        difference_image_path: difference_saved,
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
    
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx, &ctx.student.name);
 
    let echo_parameters = match ctx.data.echo_parameters {
        Some(params) => params,
        None => return generic::server_error("Faltan parámetros de ecosonda".to_string()),
    };
 
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.file_path, tx, &ctx.student.name) {
        Ok(m) => m,
        Err(err) => return generic::server_error(err),
    };
 
    let path = match lock_get_or_create_path(&cache, ctx.student_id, &matrix, &ctx.data.path_parameters,tx, &ctx.student.name) {
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
 

