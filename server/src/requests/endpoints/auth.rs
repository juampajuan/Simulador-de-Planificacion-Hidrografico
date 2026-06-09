use tiny_http::{Header, Response, Request};
use crate::structs::request::{HandlerResult};
use crate::requests::http_helper::{parse_json_body};
use crate::db::encrypt::{hash_password, verify_password};
use crate::db::queries::{professor, auth};
use std::fs::File;
use std::path::PathBuf;
use super::generic::{not_found, server_error, string_response};
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
        return string_response("Solo permitido en localhost (Por ahora).".to_string(), 403)
    }

    let data: AuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    if !check_password(&data.pass){
        return string_response("La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula.".to_string(), 400)
    }

    let password_hash = match hash_password(&data.pass) {
        Ok(hash) => hash,
        Err(_) =>  return server_error("No se pudo hashear la contraseña.".to_string())
    };

    let professor_id = match professor::create_professor(&db, &data.user, &password_hash) {
        Ok(id) => id,
        Err(_) =>  return server_error("Ya existe un profesor con ese username.".to_string())
    };

    string_response("Usuario creado correctamente".to_string(), 200)
}


pub fn change_pass(request: &mut Request, db: DBEngine) -> HandlerResult {

    if !is_local_request(&request) {
        return string_response("Solo permitido en localhost (Por ahora).".to_string(), 403)
    }

    let data: AuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    if !check_password(&data.pass){
        return string_response("La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula.".to_string(), 400)
    }

    let password_hash = match hash_password(&data.pass) {
        Ok(hash) => hash,
        Err(_) =>  return server_error("No se pudo hashear la contraseña.".to_string())
    };

    // TODO: Mergear esto. Asi lo uso en main.
    let professor_id = match professor::get_professor_id_by_username(&db, &data.user) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("No existe el usuario".to_string(), 400),
        Err(_) => return server_error("Internal error.".to_string())
    };

    let professor_id = match professor::change_password(&db, professor_id, &password_hash) {
        Ok(id) => id,
        Err(_) =>  return server_error("No se pudo cambiar la contraseña.".to_string())
    };

    string_response("Contraseña cambiada correctamente".to_string(), 200)
}

pub fn login(request: &mut Request, db: DBEngine) -> HandlerResult {

    let data: AuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    let professor_id = match professor::verify_professor_credentials(&db, &data.user, &data.pass) {
        Ok(Some(id)) => id,
        Ok(None) => return string_response("Datos incorrectos.".to_string(), 401),
        Err(_) => return server_error("Internal error.".to_string())
    };

    let token = "asdadadadadad";

    if let Err(_) = auth::create_token(&db, professor_id, token, 7) {
        return server_error("Internal error.".to_string());
    }

    let cookie = match create_auth_cookie(token) {
        Ok(cookie) => cookie,
        Err(_) => return server_error("Internal error.".to_string()),
    };

    let re = Response::from_string("OK").with_header(cookie);
    (re.boxed(), 200)
}

pub fn close_all(request: &mut Request, db: DBEngine) -> HandlerResult {

    if !is_local_request(&request) {
        return string_response("Solo permitido en localhost (Por ahora).".to_string(), 403)
    }

    if let Err(_) = auth::delete_all_tokens(&db) {
        return server_error("Internal error.".to_string());
    }

    string_response("Todos las sesiones fueron cerradas.".to_string(), 200)
}

fn check_password(pass: &str) -> bool {
    let has_upper = pass.chars().any(|c| c.is_uppercase());
    let has_number = pass.chars().any(|c| c.is_numeric());
    let ok_length = pass.len() >= 8;
    has_upper && has_number && ok_length
}

fn is_local_request(request: &Request) -> bool {
    match request.remote_addr() {
        Some(addr) => addr.ip().is_loopback(),
        None => false,
    }
}

fn create_auth_cookie(
    token: &str,
) -> Result<Header, ()> {

    let cookie = format!(
        "auth_token={}; Path=/; Max-Age=604800; HttpOnly; SameSite=Strict",
        token
    );

    Header::from_bytes("Set-Cookie", cookie)
        .map_err(|_| ())
}