use crate::db::engine::DBEngine;
use crate::db::queries_interface::{projects, student};
use crate::helpers::auth::check_profesor_auth;
use crate::helpers::files;
use crate::helpers::utils::generate_code;
use crate::logging::logger::send_message_to_logger;
use crate::logging::structs::{LogType, ThreadMessage};
use crate::requests::http_utils;
use crate::requests::http_utils::parse_json_body;
use crate::structs::request::HandlerResult;
use crate::structs::settings::Settings;
use common::NewStudent;
use serde_json;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use tiny_http::Request;

/// Endpoint para la creacion de un nuevo alumno, por parte de un docente.
/// Autentica al profesor y comprueba datos y ejecuta el metodo para agregar la entrada a al DB.
pub fn create_new_student(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        ("Iniciando la creación de un nuevo grupo/estudiante").to_string(),
        LogType::Debug,
    );

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return http_utils::string_response("Sin autorizar".to_string(), 401),
        Err(err) => return http_utils::server_error(err),
    };

    send_message_to_logger(
        tx,
        format!(
            "creando estudiante/grupo para el profesor (Id: {})",
            professor_id
        ),
        LogType::Debug,
    );

    let data: NewStudent = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return http_utils::string_response(format!("Bad Request: {}", err), 400),
    };

    let Ok(projects) = projects::get_project_by_id_locked(&db, data.project_id) else {
        return http_utils::server_error("Error al obtener los proyectos".to_string());
    };

    let Some(project) = projects else {
        return http_utils::string_response("Proyecto no encontrado".to_string(), 404);
    };

    if project.professor_id != professor_id {
        send_message_to_logger(
            tx,
            format!(
                "Docente (Id: {}) intentó crear un estudiante/grupo en un proyecto ({}) que no le pertenece.",
                professor_id, data.project_id
            ),
            LogType::Warn,
        );
        return http_utils::string_response("No te pertenece el proyecto".to_string(), 400);
    }

    match student::create_student_locked(
        &db,
        &generate_code(),
        &data.name,
        data.project_id,
        professor_id,
    ) {
        Ok(_) => {
            send_message_to_logger(
                tx,
                format!(
                    "Estudiante/grupo '{}' creado con el proyecto (Id: {}).",
                    data.name, data.project_id
                ),
                LogType::Info,
            );
            http_utils::string_response("Estudiante creado".into(), 200)
        }
        Err(e) => http_utils::server_error(format!(
            "Error interno al crear el estudiante/grupo '{}': {}",
            data.name, e
        )),
    }
}

/// Obtiene todos los alumnos, presentes en la DB, para el profesor autenticado.
pub fn get_all_students(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        ("Iniciando la obtención de todos los estudiantes/grupos").to_string(),
        LogType::Debug,
    );

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return http_utils::string_response("Sin autorizar".to_string(), 401),
        Err(err) => return http_utils::server_error(err),
    };

    let Ok(students) = student::get_students_for_professor_locked(&db, professor_id) else {
        return http_utils::server_error(format!(
            "Error al obtener los estudiantes/grupos del profesor (Id: {})",
            { professor_id }
        ));
    };

    let response = match serde_json::to_string(&students) {
        Ok(json) => json,
        Err(e) => {
            return http_utils::server_error(format!(
                "Error al obtener los estudiantes/grupos del profesor (Id: {}): {}",
                professor_id, e
            ));
        }
    };

    send_message_to_logger(
        tx,
        format!(
            "Obteniendo todos los estudiantes/grupos para el profesor (Id: {})",
            professor_id
        ),
        LogType::Debug,
    );

    http_utils::string_response(response, 200)
}

/// Borra a un alumno dado, presente en la DB, para el profesor autenticado.
/// Para eliminarlo, ademas, provee su id como profesor, para evitar que por algun error en el front
/// Un profesor autenticado, pueda borrar alumnos que no le pertenecen.
pub fn delete_a_student(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    settings: Arc<Settings>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        ("Iniciando la eliminación de un estudiante/grupo").to_string(),
        LogType::Debug,
    );

    let Some(id_str) = request.url().rsplit('/').next() else {
        return http_utils::string_response("Bad Request".to_string(), 400);
    };

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return http_utils::string_response("Sin autorizar".to_string(), 401),
        Err(err) => return http_utils::server_error(err),
    };

    let Ok(id) = id_str.parse::<i64>() else {
        return http_utils::string_response("ID inválido".to_string(), 400);
    };

    send_message_to_logger(
        tx,
        format!(
            "Docente (Id: {}) intenta eliminar el estudiante/grupo (Id: {}).",
            professor_id, id
        ),
        LogType::Debug,
    );

    let result = student::delete_student_locked(&db, id, professor_id);

    if let Err(e) = files::clean_unused_images(&db, &settings, tx) {
        send_message_to_logger(
            tx,
            format!(
                "No se pudieron limpiar las imagenes sin uso tras borrar al alumno {}: {}",
                id, e
            ),
            LogType::Error,
        );
    }

    match result {
        Ok(true) => {
            send_message_to_logger(
                tx,
                format!(
                    "Estudiante/grupo {} eliminado por el profesor {}.",
                    id, professor_id
                ),
                LogType::Info,
            );
            http_utils::string_response("Estudiante eliminado".to_string(), 200)
        }
        Ok(false) => http_utils::string_response("Estudiante no encontrado.".to_string(), 404),
        Err(e) => http_utils::server_error(format!("Error al eliminar al alumno {}: {}", id, e)),
    }
}

/// Actualiza los datos de un alumno.
/// Comprueba que el profesor este autenticado.
pub fn update_an_student(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        ("Iniciando la actualización de un estudiante").to_string(),
        LogType::Debug,
    );

    let id_str = match request.url().rsplit('/').next() {
        Some(id) => id,
        None => return http_utils::string_response("Bad Request".to_string(), 400),
    };

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return http_utils::string_response("Sin autorizar".to_string(), 401),
        Err(err) => return http_utils::server_error(err),
    };

    let id = match id_str.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return http_utils::string_response("ID inválido".to_string(), 400),
    };

    send_message_to_logger(
        tx,
        format!(
            "Docente (Id: {}) intenta actualizar el estudiante/grupo (Id: {}).",
            professor_id, id
        ),
        LogType::Debug,
    );

    let data: NewStudent = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return http_utils::string_response(format!("Bad Request: {}", err), 400),
    };

    match student::update_student_locked(&db, id, &data.name, data.project_id, professor_id) {
        Ok(true) => {
            send_message_to_logger(
                tx,
                format!(
                    "Alumno {} actualizado por el profesor {}.",
                    id, professor_id
                ),
                LogType::Info,
            );
            http_utils::string_response("Estudiante actualizado".to_string(), 200)
        }
        Ok(false) => http_utils::string_response("Estudiante no encontrado.".to_string(), 404),
        Err(e) => http_utils::server_error(format!("Error al actualizar al alumno {}: {}", id, e)),
    }
}
