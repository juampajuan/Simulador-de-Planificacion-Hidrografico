use tiny_http::Request;
use crate::utils::helpers::generate_code;
use crate::db::engine::DBEngine;
use crate::db::queries::student::{NewStudent};
use crate::db::queries_interface::{student, projects};
use crate::requests::http_helper::parse_json_body;
use crate::requests::endpoints::generic::{server_error, string_response};
use crate::structs::request::HandlerResult;
use crate::utils::helpers_endpoints::check_profesor_auth;
use std::sync::{Arc, Mutex};
use serde_json;


/// Endpoint para la creacion de un nuevo alumno, por parte de un docente.
/// Autentica al profesor y comprueba datos y ejecuta el metodo para agregar la entrada a al DB.
pub fn create_new_student(request: &mut Request, db: Arc<Mutex<DBEngine>>) -> HandlerResult {

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Sin autorizar".to_string(), 401),
        Err(err) => return server_error(err),
    };

    let data: NewStudent = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return string_response(format!("Bad Request: {}", err), 400),
    };

    let Ok(projects) = projects::get_project_by_id_locked(&db, data.project_id) else {
        return server_error("Error al obtener los proyectos".to_string());
    };

    let Some(project) = projects else {
        return string_response("Proyecto no encontrado".to_string(), 404);
    };

    if project.professor_id != professor_id {
        return string_response("No te pertenece el proyecto".to_string(), 400)
    }

    match student::create_student_locked(&db, &generate_code(), &data.name, data.project_id, professor_id) {
        Ok(_) => string_response("Estudiante creado".into(), 200),
        Err(_) => server_error("Interal error".into())
    }

}

/// Obtiene todos los alumnos, presentes en la DB, para el profesor autenticado.
pub fn get_all_students(request: &mut Request, db: Arc<Mutex<DBEngine>>) -> HandlerResult {

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Sin autorizar".to_string(), 401),
        Err(err) => return server_error(err),
    };

    let Ok(students) = student::get_students_for_professor_locked(&db, professor_id) else {
        return server_error("Error al obtener los alumnos".to_string());
    };

    let response = match serde_json::to_string(&students) {
        Ok(json) => json,
        Err(_) => return server_error("Error serializing limits data".to_string()),
    };

    string_response(response, 200)
}

/// Borra a un alumno dado, presente en la DB, para el profesor autenticado.
/// Para eliminarlo, ademas, provee su id como profesor, para evitar que por algun error en el front
/// Un profesor autenticado, pueda borrar alumnos que no le pertenecen.
pub fn delete_a_student(request: &mut Request, db: Arc<Mutex<DBEngine>>) -> HandlerResult {

    let Some(id_str) = request.url().rsplit('/').next() else {
        return string_response("Bad Request".to_string(), 400);
    };

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Sin autorizar".to_string(), 401),
        Err(err) => return server_error(err),
    };

    let Ok(id) = id_str.parse::<i64>() else {
        return string_response("ID inválido".to_string(), 400);
    };

    match student::delete_student_locked(&db, id, professor_id) {
        Ok(true) => string_response("Estudiante eliminado".to_string(), 200),
        Ok(false) => string_response("Estudiante no encontrado.".to_string(), 404),
        Err(_) => server_error("Error al eliminar".to_string())
    }
}

/// Actualiza los datos de un alumno.
/// Comprueba que el profesor este autenticado.
pub fn update_an_student(request: &mut Request, db: Arc<Mutex<DBEngine>>) -> HandlerResult {
    
    let id_str = match request.url().rsplit('/').next() {
        Some(id) => id,
        None => return string_response("Bad Request".to_string(), 400),
    };

    let professor_id = match check_profesor_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Sin autorizar".to_string(), 401),
        Err(err) => return server_error(err),
    };

    let id = match id_str.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return string_response("ID inválido".to_string(), 400),
    };

    let data: NewStudent = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return string_response(format!("Bad Request: {}", err), 400),
    };

    match student::update_student_locked(&db, id, &data.name, data.project_id, professor_id) {
        Ok(true) => string_response("Estudiante actualizado".to_string(), 200),
        Ok(false) => string_response("Estudiante no encontrado.".to_string(), 404),
        Err(_) => server_error("Error al actualizar".to_string())
    }
}