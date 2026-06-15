use tiny_http::Request;
use crate::utils::helpers::generate_code;
use crate::db::engine::DBEngine;
use crate::db::queries::student::{NewStudent, create_student, get_students_for_professor,update_student, delete_student};
use crate::db::queries::proyects::{delete_project_by_id, get_all_by_professor_id, get_project_by_id, create_project, ProjectMetadata};
use crate::requests::endpoints::auth::{check_profesor_auth};
use crate::requests::http_helper::parse_json_body;
use crate::structs::request::HandlerResult;
use crate::requests::endpoints::generic::{server_error, string_response};
use crate::structs::settings::Settings;
use std::sync::{Arc};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::db::queries::{professor, auth, student};
use serde_json;
use std::{
    fs::File,
    io::{Read, Write},
}; 

pub fn create_new_student(request: &mut Request, db: DBEngine) -> HandlerResult {

    let Some(id) = check_profesor_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let data: NewStudent = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return string_response(format!("Bad Request: {}", err), 400),
    };

    // TODO: Chquear q pasa si no existe.
    let Ok(projects) = get_project_by_id(&db, data.project_id) else {
        return server_error("Error al obtener los proyectos".to_string());
    };

    let Some(project) = projects else {
        return string_response("Proyecto no encontrado".to_string(), 404);
    };

    if project.professor_id != id {
        return string_response(format!("No te pertenece el proyecto"), 400)
    }

    match create_student(&db, &generate_code(), &data.name, data.project_id, id) {
        Ok(_) => string_response("Estudiante creado".into(), 200),
        Err(_) => server_error("Interal error".into())
    }

}

pub fn get_all_students(request: &mut Request, db: DBEngine) -> HandlerResult {

    let Some(id) = check_profesor_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let Ok(students) = get_students_for_professor(&db, id) else {
        return server_error("Error al obtener los alumnos".to_string());
    };

    let response = match serde_json::to_string(&students) {
        Ok(json) => json,
        Err(_) => return server_error("Error serializing limits data".to_string()),
    };

    string_response(response, 200)
}

pub fn delete_a_student(request: &mut Request, db: DBEngine) -> HandlerResult {

    let Some(id_str) = request.url().rsplit('/').next() else {
        return string_response("Ruta inválida".to_string(), 400);
    };

    let Some(_) = check_profesor_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let Ok(id) = id_str.parse::<i64>() else {
        return string_response("ID inválido".to_string(), 400);
    };

    match delete_student(&db, id) {
        Ok(true) => string_response("Estudiante eliminado".to_string(), 200),
        Ok(false) => string_response("Estudiante no encontrado.".to_string(), 404),
        Err(_) => server_error("Error al eliminar".to_string())
    }
}

pub fn update_an_student(request: &mut Request, db: DBEngine) -> HandlerResult {
    
    let Some(professor_id) = check_profesor_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let id_str = match request.url().rsplit('/').next() {
        Some(id) => id,
        None => return string_response("Ruta inválida".to_string(), 400),
    };

    let id = match id_str.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return string_response("ID inválido".to_string(), 400),
    };

    let data: NewStudent = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return string_response(format!("Bad Request: {}", err), 400),
    };

    match update_student(&db, id, &data.name, data.project_id, professor_id) {
        Ok(true) => string_response("Estudiante actualizado".to_string(), 200),
        Ok(false) => string_response("Estudiante no encontrado.".to_string(), 404),
        Err(_) => server_error("Error al actualizar".to_string())
    }
}