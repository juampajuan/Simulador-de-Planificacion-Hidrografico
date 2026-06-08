use tiny_http::{Header, Response, Request};
use crate::structs::request::{HandlerResult};
use crate::requests::http_helper::{parse_json_body};
use crate::db::encrypt::{hash_password, verify_password};
use crate::db::queries::professor;
use std::fs::File;
use std::path::PathBuf;
use super::generic::{not_found, server_error, normal_response};
use crate::db::engine::DBEngine;
use serde_json::Value;

#[derive(serde::Deserialize)]
pub struct AuthData {
    #[serde(default)]
    pub user: String,
    pub pass: String,
}

pub fn create_professor(request: &mut Request, db: DBEngine) -> HandlerResult {

    if !is_local_request(&request) {
        return normal_response("Solo permitido en localhost (Por ahora).".to_string(), 403)
    }

    let data: AuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    if !check_password(&data.pass){
        return normal_response("La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula.".to_string(), 400)
    }

    let password_hash = match hash_password(&data.pass) {
        Ok(hash) => hash,
        Err(_) =>  return server_error("No se pudo hashear la contraseña.".to_string())
    };

    let professor_id = match professor::create_professor(&db, &data.user, &password_hash) {
        Ok(id) => id,
        Err(_) =>  return server_error("Ya existe un profesor con ese username.".to_string())
    };

    normal_response("Usuario creado correctamente".to_string(), 200)
}


pub fn change_pass(request: &mut Request, db: DBEngine) -> HandlerResult {

    if !is_local_request(&request) {
        return normal_response("Solo permitido en localhost (Por ahora).".to_string(), 403)
    }

    let data: AuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    if !check_password(&data.pass){
        return normal_response("La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula.".to_string(), 400)
    }

    let password_hash = match hash_password(&data.pass) {
        Ok(hash) => hash,
        Err(_) =>  return server_error("No se pudo hashear la contraseña.".to_string())
    };

    let professor_id = match professor::get_professor_id_by_username(&db, &data.user) {
        Ok(Some(id)) => id,
        Ok(None) => return normal_response("No existe el usuario".to_string(), 400),
        Err(_) => return server_error("Internal error.".to_string())
    };

    let professor_id = match professor::change_password(&db, professor_id, &password_hash) {
        Ok(id) => id,
        Err(_) =>  return server_error("No se pudo cambiar la contraseña.".to_string())
    };

    normal_response("Contraseña cambiada correctamente".to_string(), 200)
}

pub fn login(request: &mut Request, db: DBEngine) -> HandlerResult {

    let data: AuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };
 

    normal_response("Pending".to_string(), 200)
}

fn check_password(pass: &str) -> bool {
    let has_upper = pass.chars().any(|c| c.is_uppercase());
    let has_number = pass.chars().any(|c| c.is_numeric());
    let ok_length = pass.len() >= 8;
    has_upper && has_number && ok_length
}

pub fn is_local_request(request: &Request) -> bool {
    match request.remote_addr() {
        Some(addr) => addr.ip().is_loopback(),
        None => false,
    }
}