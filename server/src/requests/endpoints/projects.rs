use tiny_http::Request;
use crate::db::engine::DBEngine;
use crate::db::queries::proyects::{delete_project_by_id, get_all_by_professor_id, get_project_by_id};
use crate::requests::endpoints::auth::{check_profesor_auth, check_student_auth, get_cookie};
use crate::structs::request::HandlerResult;
use crate::requests::endpoints::generic::{server_error, string_response};
use crate::structs::settings::Settings;
use std::sync::{Arc};
use crate::db::queries::{professor, auth, student};


// TODO: Falta el de crear, editar projectos

pub fn get_projects(request: &mut Request, db: DBEngine, settings: Arc<Settings>) -> HandlerResult {
  
    let Some(id) = check_profesor_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let Ok(projects) = get_all_by_professor_id(&db, id) else {
        return server_error("Error al obtener los proyectos".to_string());
    };

    let response = match serde_json::to_string(&projects) {
        Ok(json) => json,
        Err(_) => return server_error("Error serializing limits data".to_string()),
    };

    string_response(response, 200)
}

pub fn get_student_project(request: &mut Request, db: DBEngine, settings: Arc<Settings>) -> HandlerResult {
  
    let Some(id) = check_student_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let Ok(projects) = get_project_by_id(&db, id) else {
        return server_error("Error al obtener los proyectos".to_string());
    };

    let response = match serde_json::to_string(&projects) {
        Ok(json) => json,
        Err(_) => return server_error("Error serializing limits data".to_string()),
    };

    string_response(response, 200)
}

pub fn delete_project(request: &mut Request, db: DBEngine, settings: Arc<Settings>) -> HandlerResult {

    let Some(id_str) = request.url().rsplit('/').next() else {
        return string_response("Ruta inválida".to_string(), 400);
    };

    let Ok(id) = id_str.parse::<i64>() else {
        return string_response("ID inválido".to_string(), 400);
    };
  
    let Some(professor_id) = check_profesor_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    match delete_project_by_id(&db, id, professor_id) {
        Ok(true) => string_response("Proyecto eliminado.".to_string(), 200),
        Ok(false) => string_response(
                "Proyecto no encontrado".to_string(),
                404,
        ),
        Err(_) => server_error(
                "Error al eliminar el proyecto".to_string(),
            )
    }

}