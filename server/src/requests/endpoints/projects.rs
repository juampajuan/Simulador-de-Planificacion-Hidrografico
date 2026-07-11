use crate::db::engine::DBEngine;
use crate::db::queries::proyects::ProjectMetadata;
use crate::db::queries_interface::projects;
use crate::db::queries_interface::student;
use crate::helpers::files;
use crate::logging::logger::{debug_logger, send_message_to_logger};
use crate::logging::structs::{LogType, ThreadMessage};
use crate::requests::endpoints::generic::{server_error, string_response};
use crate::requests::http_helper::parse_json_body;
use crate::structs::request::HandlerResult;
use crate::structs::settings::Settings;
use crate::structs::strudent_project_response::{GeoCorners, StudentProjectResponse};
use crate::utils::helpers_endpoints::{check_profesor_auth, check_student_auth};
use multipart::server::Multipart;
use serde_json;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs::File, io::Read};
use tiny_http::Request;

/// Metodo auxiliar para procesar el body multipart.
/// Se usa para la request de `crear_project`
pub fn get_boundary(request: &Request) -> Result<String, &str> {
    let content_type = match request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Content-Type"))
        .map(|h| h.value.as_str())
    {
        Some(v) => v,
        None => return Err("Missing Content-Type"),
    };

    let boundary = match content_type.split(';').find_map(|part| {
        let part = part.trim();

        part.strip_prefix("boundary=")
            .map(|boundary| boundary.to_string())
    }) {
        Some(b) => b,
        None => return Err("Missing boundary"),
    };

    Ok(boundary)
}

/// Genera un nuevo proyecto.
/// Primero comprueba que el profesor este autenticado
/// Luego recibe el archivo .tif y lo almacena en la carpeta
/// Y por ultimo genera la entrada, con la informacion extra en la DB.
pub fn create(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    settings: Arc<Settings>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "Iniciando creacion de un nuevo proyecto.".to_string(),
        LogType::Debug,
    );

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Sin autorizar".to_string(), 401),
        Err(err) => return server_error(err),
    };

    send_message_to_logger(
        tx,
        format!(
            "Iniciando creacion de un nuevo proyecto por parte del profesor con ID: {}",
            professor_id
        ),
        LogType::Info,
    );

    let boundary = match get_boundary(request) {
        Ok(b) => b,
        Err(e) => return server_error(e.into()),
    };

    let mut multipart = Multipart::with_body(request.as_reader(), boundary);

    let mut metadata_json = None::<String>;
    let mut filename_saved = None::<String>;
    let mut invalid_extension = false;
    let mut write_error = None::<String>;

    if let Err(_e) = multipart.foreach_entry(|mut field| {
        match field.headers.name.as_ref() {
            "metadata" => {
                let mut json = String::new();
                let _ = field.data.read_to_string(&mut json);

                metadata_json = Some(json);
            }

            "file" => {
                let original_filename = field
                    .headers
                    .filename
                    .clone()
                    .unwrap_or_else(|| "upload.bin".to_string());

                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or_else(|_| rand::random::<u64>());

                let path = Path::new(&original_filename);

                // Validamos que la extension sea .tif o .tiff antes de guardar nada.
                let ext_ok = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("tif") || ext.eq_ignore_ascii_case("tiff"))
                    .unwrap_or(false);

                if !ext_ok {
                    invalid_extension = true;
                    return;
                }

                let filename = match (
                    path.file_stem().and_then(|s| s.to_str()),
                    path.extension().and_then(|s| s.to_str()),
                ) {
                    (Some(stem), Some(ext)) => format!("{}_{}.{}", stem, timestamp, ext),
                    (Some(stem), None) => format!("{}_{}", stem, timestamp),
                    _ => format!("upload_{}.bin", timestamp),
                };

                let mut out = match File::create(format!(
                    "{}/geotiffs/{}",
                    settings.storage_path, filename
                )) {
                    Ok(f) => f,
                    Err(e) => {
                        write_error = Some(format!("No se pudo guardar el archivo: {}", e));
                        return;
                    }
                };

                let _ = std::io::copy(&mut field.data, &mut out);

                filename_saved = Some(filename);
            }

            _ => {}
        }
    }) {
        return server_error("No se pudo subir el archivo".to_string());
    }

    if let Some(err) = write_error {
        return server_error(err);
    }

    if invalid_extension {
        send_message_to_logger(
            tx,
            format!(
                "Profesor {} intentó subir un archivo con extensión inválida.",
                professor_id
            ),
            LogType::Warn,
        );
        return string_response(
            "El archivo debe ser un GeoTIFF (.tif o .tiff).".to_string(),
            400,
        );
    }

    let json = match metadata_json {
        Some(j) => j,
        None => return server_error("No hay metadata.".to_string()),
    };

    let meta: ProjectMetadata = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => return server_error("Metadata incompleta.".to_string()),
    };

    let filename = match filename_saved {
        Some(f) => f,
        None => return server_error("No se recibió ningún archivo.".to_string()),
    };

    let project = match projects::create_project_locked(&db, &filename, professor_id, &meta) {
        Ok(_) => {
            send_message_to_logger(
                tx,
                format!(
                    "Concluye correctamente la revision de datos para creacion de proyecto '{}' creado por el profesor {}.",
                    filename, professor_id
                ),
                LogType::Info,
            );
            string_response("ok".into(), 200)
        }
        Err(_) => server_error("NO pudo".into()),
    };

    send_message_to_logger(
        tx,
        format!("Proyecto para profesor con ID: {} creado.", professor_id),
        LogType::Debug,
    );

    project
}

/// Retorna todos los proyectos almacenados en la db
/// Para el profesor autenticado.
pub fn get_projects(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        ("Intentando obtener un proyecto").to_string(),
        LogType::Debug,
    );

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Sin autorizar".to_string(), 401),
        Err(err) => return server_error(err),
    };

    let projects = match projects::get_all_by_professor_id_locked(&db, professor_id) {
        Ok(projects) => projects,
        Err(e) => {
            return server_error(format!(
                "Error al obtener los proyectos del profesor {}: {}",
                professor_id, e
            ));
        }
    };

    let response = match serde_json::to_string(&projects) {
        Ok(json) => json,
        Err(_) => return server_error("Error serializing limits data".to_string()),
    };

    string_response(response, 200)
}

/// Retorna el proyecto de un alumno especifico.
/// Esto lo hace en base a la cookie que recibe en la request.
/// Con la cookie obtiene el id del mismo y con eso el proyecto
pub fn get_student_project(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    settings: Arc<Settings>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "Intentando obtener un proyecto asociado a un estudiante.".to_string(),
        LogType::Debug,
    );

    let student_id = match check_student_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Sin autorizar".to_string(), 401),
        Err(err) => return server_error(err),
    };

    let student = match student::get_student_by_id_locked(&db, student_id) {
        Ok(Some(s)) => s,
        Ok(None) => return string_response("Estudiante no encontrado".to_string(), 404),
        Err(_) => return server_error("Error al obtener los datos del alumno".to_string()),
    };

    send_message_to_logger(
        tx,
        format!(
            "Intentando obtener el proyecto del grupo/estudiante {}.",
            student.name
        ),
        LogType::Debug,
    );

    // Closure para el DEBUG del logger, que se pasa a los metodos de simulacion para loggear desde alli.
    let log_debug = debug_logger(tx, &student.name);

    let project_id_opt = match projects::get_project_id_by_student_locked(&db, student_id) {
        Ok(id_opt) => id_opt,
        Err(e) => {
            return server_error(format!(
                "Error al obtener el proyecto del grupo/estudiante {}: {}",
                student_id, e
            ));
        }
    };

    let Some(project_id_real) = project_id_opt else {
        return string_response("Alumno no tiene asignado un proyecto".to_string(), 404);
    };

    let Ok(projects_opt) = projects::get_project_by_id_locked(&db, project_id_real) else {
        return server_error("Error al obtener los detalles del proyecto".to_string());
    };

    let Some(project) = projects_opt else {
        return string_response("Proyecto no encontrado".to_string(), 404);
    };

    // Le enchufo al Json las coordenadas del tiff en lat,lon
    let geotiff_path = format!("{}/geotiffs/{}", settings.storage_path, project.filename);

    let (sup_izq, sup_der, inf_izq, inf_der, centro) =
        match simulations::get_geotiff_corners(&geotiff_path, &log_debug) {
            Ok(c) => c,
            Err(e) => {
                return server_error(format!("No se pudieron calcular las coordenadas: {}", e));
            }
        };

    let final_response = StudentProjectResponse {
        project,
        attempts_spent: student.attempts, // El número real (ej: 1)
        coordinates: GeoCorners {
            sup_izq,
            sup_der,
            inf_izq,
            inf_der,
            centro,
        },
        maptiler_api_key: settings.maptiler_api_key.clone(),
    };

    let response = match serde_json::to_string(&final_response) {
        Ok(json) => json,
        Err(_) => return server_error("Error serializing student project data".to_string()),
    };

    string_response(response, 200)
}

/// Elimina un proyecto de la base de datos
/// Comprobando primero que el profesor este autenticado.
pub fn delete_project(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    settings: Arc<Settings>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "Iniciando el borrado de un proyecto.".to_string(),
        LogType::Debug,
    );

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Sin autorizar".to_string(), 401),
        Err(err) => return server_error(err),
    };

    let Some(id_str) = request.url().rsplit('/').next() else {
        return string_response("Ruta inválida".to_string(), 400);
    };

    let Ok(id) = id_str.parse::<i64>() else {
        return string_response("ID inválido".to_string(), 400);
    };

    let Ok(projects) = projects::get_project_by_id_locked(&db, id) else {
        return server_error("Error al obtener el proyecto.".to_string());
    };

    let Some(project) = projects else {
        return string_response("Proyecto no encontrado".to_string(), 404);
    };

    let filename = project.filename;
    let result = projects::delete_project_by_id_locked(&db, id, professor_id);

    if let Err(e) = files::clean_unused_images(&db, &settings, tx) {
        send_message_to_logger(
            tx,
            format!(
                "No se pudieron limpiar las imagenes sin uso tras borrar el proyecto {}: {}",
                id, e
            ),
            LogType::Error,
        );
    }

    send_message_to_logger(
        tx,
        format!(
            "Iniciando el borrado del proyecto: {} por parte del profesor {}.",
            project.metadata.name, professor_id
        ),
        LogType::Info,
    );

    match result {
        Ok(true) => {
            let path = format!("{}/geotiffs/{}", settings.storage_path, filename);
            let _ = std::fs::remove_file(&path);
            send_message_to_logger(
                tx,
                format!(
                    "Proyecto {} ('{}') eliminado por el profesor {}.",
                    id, project.metadata.name, professor_id
                ),
                LogType::Info,
            );
            string_response("Proyecto eliminado.".to_string(), 200)
        }
        Ok(false) => string_response("Proyecto no encontrado".to_string(), 404),
        Err(e) => server_error(format!("Error al eliminar el proyecto {}: {}", id, e)),
    }
}

/// Actualiza la informacion de un proyecto en la db
/// Comprobando primero que el profesor este autenticado.
pub fn update_a_project(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "Iniciando la actualización de un proyecto.".to_string(),
        LogType::Debug,
    );
    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Sin autorizar".to_string(), 401),
        Err(err) => return server_error(err),
    };

    let id_str = match request.url().rsplit('/').next() {
        Some(id) => id,
        None => return string_response("Ruta inválida".to_string(), 400),
    };

    let id = match id_str.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return string_response("ID inválido".to_string(), 400),
    };

    let meta: ProjectMetadata = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    match projects::update_project_locked(&db, id, professor_id, &meta) {
        Ok(true) => {
            send_message_to_logger(
                tx,
                format!(
                    "Proyecto {} actualizado por el profesor {}.",
                    id, professor_id
                ),
                LogType::Info,
            );
            string_response("Proyecto actualizado.".to_string(), 200)
        }
        Ok(false) => string_response("Proyecto no encontrado.".to_string(), 404),
        Err(e) => server_error(format!("Error al actualizar el proyecto {}: {}", id, e)),
    }
}
