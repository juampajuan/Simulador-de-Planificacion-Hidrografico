use crate::db::engine::DBEngine;
use crate::db::queries_interface::student_simulations::get_next_attempt_number_locked;
use crate::db::queries_interface::{projects, student_simulations};
use crate::helpers::simulation::{
    SimulationImagesInput, extract_request_context, lock_get_or_create_matrix,
    lock_get_or_create_path, save_simulation_images,
};
use crate::logging::logger::{debug_logger, send_message_to_logger};
use crate::logging::structs::{LogType, ThreadMessage};
use crate::requests::http_utils;
use crate::requests::http_utils::create_png_response;
use crate::structs::filecache::FileCache;
use crate::structs::request::HandlerResult;
use crate::structs::settings::Settings;
use common::{SimulationResponse, StudentSimulationData};
use std::io::Cursor;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use tiny_http::Request;

/// Endpoint para la creacion del recorrido.
/// Utiliza contenidos de la cache y la db, si los hay.
pub fn create_path(
    request: &mut Request,
    cache: Arc<Mutex<FileCache>>,
    db: Arc<Mutex<DBEngine>>,
    settings: Arc<Settings>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let prefix = format!(
        "Estudiante (Id: {}), proyecto (Id: {})",
        &ctx.student.id, &ctx.project_id
    );
    let log_debug = debug_logger(tx, &prefix);

    // reutilizamos la depthmatrix o la creamos
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.file_path, tx, &prefix) {
        Ok(m) => m,
        Err(err) => return http_utils::server_error(err),
    };

    // reutilizamos el path o lo creamos
    let path = match lock_get_or_create_path(
        &cache,
        ctx.student_id,
        &matrix,
        &ctx.data.path_parameters,
        tx,
        &prefix,
    ) {
        Ok(p) => p,
        Err(err) => return http_utils::server_error(err),
    };

    let image = simulations::create_path_image(&matrix, &path, &log_debug);
    let response = create_png_response(image);

    send_message_to_logger(
        tx,
        format!("{} : Generó un nuevo recorrido.", prefix),
        LogType::Info,
    );

    (response.boxed(), 200, None)
}

/// Endpoint para la simulacion/interpolacion.
/// Utiliza recursos de la db y cache si los hay, sino debera tambien crear el path.
/// Revisa y compara la cantidad de intentos realizados.
pub fn run_simulation(
    request: &mut Request,
    cache: Arc<Mutex<FileCache>>,
    db: Arc<Mutex<DBEngine>>,
    settings: Arc<Settings>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let prefix = format!(
        "Estudiante (Id: {}), proyecto (Id: {})",
        &ctx.student.id, &ctx.project_id
    );
    let log_debug = debug_logger(tx, &prefix);

    let limit = ctx.project.metadata.attempts_limit;
    if limit != -1 && ctx.student.attempts >= limit {
        return http_utils::string_response(
            "Has alcanzado el límite máximo de intentos permitidos para este proyecto.".to_string(),
            403,
        );
    }

    let echo_parameters = match ctx.data.echo_parameters {
        Some(params) => params,
        None => return http_utils::server_error("Faltan parámetros de ecosonda".to_string()),
    };

    // reutilizamos la depth matrix o la creamos
    let matrix = match lock_get_or_create_matrix(&cache, &ctx.file_path, tx, &prefix) {
        Ok(m) => m,
        Err(err) => return http_utils::server_error(err),
    };

    // reutilizamos el path o lo creamos
    let path = match lock_get_or_create_path(
        &cache,
        ctx.student_id,
        &matrix,
        &ctx.data.path_parameters,
        tx,
        &prefix,
    ) {
        Ok(p) => p,
        Err(err) => return http_utils::server_error(err),
    };

    if path.is_empty() {
        return http_utils::server_error("Error: El Recorrido (Path) está vacío.".to_string());
    }

    let interpolation = match simulations::run_simulation(
        &matrix,
        &path,
        echo_parameters,
        settings.simulation_constants(),
        &log_debug,
    ) {
        Ok(interp) => interp,
        Err(e) => return http_utils::server_error(e),
    };

    // Generamos las imágenes de la simulación
    let (
        map_image,
        real_min_depth,
        real_max_depth,
        interpolation_min_depth,
        interpolation_max_depth,
    ) = simulations::create_simulation_image(&matrix, &interpolation, &log_debug);

    // Guardamos de forma limpia y segura los límites en la base de datos a través del wrapper
    if let Err(e) = projects::update_project_geotiff_bounds_locked(
        &db,
        ctx.project_id,
        real_min_depth,
        real_max_depth,
    ) {
        send_message_to_logger(
            tx,
            format!(
                "Error al actualizar límites del GeoTIFF del proyecto {}: {}",
                ctx.project_id, e
            ),
            LogType::Error,
        );
        return http_utils::server_error(
            "Error interno al actualizar parámetros del proyecto".to_string(),
        );
    }

    let mut map_bytes = Vec::new();
    let _ = map_image.write_to(&mut Cursor::new(&mut map_bytes), image::ImageFormat::Png);

    let attempt_number = match get_next_attempt_number_locked(&db, ctx.student_id) {
        Ok(number) => number,
        Err(e) => {
            send_message_to_logger(
                tx,
                format!(
                    "Error al obtener el intento en la DB para el alumno {}: {}",
                    ctx.student_id, e
                ),
                LogType::Error,
            );

            return http_utils::server_error("No se pudo obtener el número de intento".to_string());
        }
    };

    let images_input = SimulationImagesInput {
        constants: settings.simulation_constants(),
        matrix,
        interpolation,
        path,
        params: echo_parameters,
        map_bytes,
        settings,
        student_id: ctx.student_id,
        attempt_number,
    };
    let (map_saved, coverage_saved, difference_saved) =
        save_simulation_images(&images_input, tx, &log_debug);

    let sim_data = StudentSimulationData {
        student_id: ctx.student_id,
        project_id: ctx.project_id,
        attempt_number,
        result_min_depth: interpolation_min_depth,
        result_max_depth: interpolation_max_depth,
        path_parameters: ctx.data.path_parameters.clone(),
        transport_parameters: echo_parameters.transport_parameters,
        echosounder_parameters: echo_parameters.echo_sounder_parameters,
        simulation_image_path: map_saved.clone(),
        coverage_image_path: coverage_saved.clone(),
        difference_image_path: difference_saved.clone(),
    };

    if let Err(e) = student_simulations::create_student_simulation_locked(&db, &sim_data) {
        send_message_to_logger(
            tx,
            format!(
                "Error al registrar el intento en la DB para el alumno {}: {}",
                ctx.student_id, e
            ),
            LogType::Error,
        );

        return http_utils::server_error("Error al guardar el intento de simulación".to_string());
    }

    let response_data = SimulationResponse {
        real_min_depth,
        real_max_depth,
        interpolation_min_depth,
        interpolation_max_depth,
        simulation_image_path: map_saved,
        coverage_image_path: coverage_saved,
        difference_image_path: difference_saved,
    };

    let json_payload = match serde_json::to_string(&response_data) {
        Ok(json) => json,
        Err(_) => {
            return http_utils::server_error(
                "Error al serializar la respuesta de la simulación".to_string(),
            );
        }
    };

    send_message_to_logger(
        tx,
        format!(
            "Estudiante (Id: {}), proyecto (Id: {}): Simulación completada con éxito.",
            &ctx.student.id, &ctx.project_id
        ),
        LogType::Info,
    );
    if let Err(e) = crate::db::queries_interface::student::increment_student_attempts_locked(
        &db,
        ctx.student_id,
    ) {
        send_message_to_logger(
            tx,
            format!(
                "Error al registrar el intento en la DB para el alumno {}: {}",
                ctx.student_id, e
            ),
            LogType::Error,
        );
        return http_utils::server_error("Error interno al registrar el intento".to_string());
    }
    http_utils::string_response(json_payload, 200)
}

/// Genera la imagen de cubrimiento del recorrido con los parametros actuales.
/// Sirve para ver de manera preliminar que areas cubre el recorrido.
pub fn create_coverage_image(
    request: &mut Request,
    cache: Arc<Mutex<FileCache>>,
    db: Arc<Mutex<DBEngine>>,
    settings: Arc<Settings>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    let ctx = match extract_request_context(request, &db, &settings) {
        Ok(context) => context,
        Err(response) => return response,
    };

    //Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let prefix = format!(
        "Estudiante (Id: {}), proyecto (Id: {})",
        &ctx.student.id, &ctx.project_id
    );
    let log_debug = debug_logger(tx, &prefix);

    let echo_parameters = match ctx.data.echo_parameters {
        Some(params) => params,
        None => return http_utils::server_error("Faltan parámetros de ecosonda".to_string()),
    };

    let matrix = match lock_get_or_create_matrix(&cache, &ctx.file_path, tx, &prefix) {
        Ok(m) => m,
        Err(err) => return http_utils::server_error(err),
    };

    let path = match lock_get_or_create_path(
        &cache,
        ctx.student_id,
        &matrix,
        &ctx.data.path_parameters,
        tx,
        &prefix,
    ) {
        Ok(p) => p,
        Err(err) => return http_utils::server_error(err),
    };

    if path.is_empty() {
        return http_utils::server_error("Error: El Recorrido (Path) está vacío.".to_string());
    }

    let image = simulations::create_path_with_coverage(
        &matrix,
        &path,
        echo_parameters,
        settings.simulation_constants(),
        &log_debug,
    );
    let response = create_png_response(image);

    send_message_to_logger(
        tx,
        format!("{} : Imagen de cobertura generada.", prefix),
        LogType::Info,
    );

    (response.boxed(), 200, None)
}
