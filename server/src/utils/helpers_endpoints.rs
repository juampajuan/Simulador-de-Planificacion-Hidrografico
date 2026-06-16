use std::sync::{Arc, Mutex};
use tiny_http::{Request};
use crate::{db::{engine::DBEngine, queries::auth::TokenOwner,queries_interface::{auth, projects}},
            requests::endpoints::generic::{server_error, string_response}, 
            structs::{request::HandlerResult, settings::Settings    
        }, utils::helpers::get_cookie};

pub fn check_profesor_auth(request: &tiny_http::Request, db: &Arc<Mutex<DBEngine>>) -> Result<Option<i64>,String> {

    let Some(token) = get_cookie(request, "auth_token") else {
        return Ok(None);
    };

    match auth::get_user_by_token_locked(&db, &token) {
        Ok(Some(TokenOwner::Professor(id))) => Ok(Some(id)),
        Ok(_) => Ok(None),
        Err(e) => Err(e.to_string())
    }
}

pub fn check_student_auth(request: &tiny_http::Request, db: &Arc<Mutex<DBEngine>>) -> Result<Option<i64>,String> {

    let Some(token) = get_cookie(request, "auth_token") else {
        return Ok(None);
    };

    match auth::get_user_by_token_locked(&db, &token) {
        Ok(Some(TokenOwner::Student(id))) => Ok(Some(id)),
        Ok(_) => Ok(None),
        Err(e) => Err(e.to_string())
    }
}

pub fn get_file_path_for_student(
    request: &mut Request,
    db: &Arc<Mutex<DBEngine>>,
    settings: &Arc<Settings>,
) -> Result<String, HandlerResult> {
    let student_id = match check_student_auth(request, db) {
        Ok(Some(id)) => id,
        Ok(None) => return Err(string_response("Sin autorizar".to_string(), 401)),
        Err(err) => return Err(server_error(err)),
    };
 
    let project_id = match projects::get_project_id_by_student_locked(db, student_id) {
        Ok(Some(id)) => id,
        Ok(None) => return Err(string_response("Proyecto no encontrado".to_string(), 404)),
        Err(_) => return Err(server_error("Error al obtener el proyecto del estudiante".to_string())),
    };
 
    let filename = match projects::get_project_by_id_locked(db, project_id) {
        Ok(Some(project)) => project.filename,
        Ok(None) => return Err(string_response("Proyecto no encontrado".to_string(), 404)),
        Err(_) => return Err(server_error("Error al obtener el proyecto".to_string())),
    };
 
    Ok(format!("{}/geotiffs/{}", settings.upload_path, filename))
}

