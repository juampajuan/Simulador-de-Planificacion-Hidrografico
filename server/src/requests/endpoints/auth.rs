use tiny_http::{Header, Response, Request};
use crate::structs::request::{HandlerResult};
use crate::requests::http_helper::{parse_json_body};
use std::fs::File;
use std::path::PathBuf;
use super::errors::{not_found, server_error};
use serde_json::Value;

#[derive(serde::Deserialize)]
pub struct AuthData {
    #[serde(default)]
    pub user: String,
    pub pass: String,
}

pub fn create_professor(request: &mut Request) -> HandlerResult {

    if !is_local_request(&request) {
       let mut response = Response::from_string("Solo permitido en localhost (Por ahora).")
        .with_status_code(403);
        return (response.boxed(), 403)
    }

    let data: AuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    if (!check_password(&data.pass)){
        let mut response = Response::from_string("La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula.")
        .with_status_code(400);
        return (response.boxed(), 400)
    }

    println!("{}", data.pass);

    let mut response = Response::from_string("TEST")
        .with_status_code(200);

    (response.boxed(), 200)
}

pub fn change_pass(request: &mut Request) -> HandlerResult {

    if !is_local_request(&request) {
       let mut response = Response::from_string("Solo permitido en localhost (Por ahora).")
        .with_status_code(403);
        return (response.boxed(), 403)
    }

    let data: AuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    if (!check_password(&data.pass)){
        let mut response = Response::from_string("La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula.")
        .with_status_code(400);
        return (response.boxed(), 400)
    }
 
    let mut response = Response::from_string("TEST")
        .with_status_code(200);

    (response.boxed(), 200)
}

pub fn login(request: &mut Request) -> HandlerResult {

    let data: AuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };
 

    let mut response = Response::from_string("TEST")
        .with_status_code(200);

    (response.boxed(), 200)
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