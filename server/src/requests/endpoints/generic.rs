use tiny_http::{Response};
use crate::structs::request::{HandlerResult};

pub fn not_found() -> HandlerResult {
    let mut response = Response::from_string("404 no existe")
        .with_status_code(404);

    // Agregamos CORS
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Origin", b"*") {
        response = response.with_header(h);
    }

    (response.boxed(), 404)
}

pub fn server_error(msg: String) -> HandlerResult {
    let mut response = Response::from_string(msg)
        .with_status_code(500);

    // Agregamos CORS
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Origin", b"*") {
        response = response.with_header(h);
    }

    (response.boxed(), 500)
}

pub fn normal_response(msg: String, code: i32) -> HandlerResult {
    let mut response = Response::from_string(msg)
        .with_status_code(code);

    if let Ok(header) = tiny_http::Header::from_bytes(b"Content-Type", b"application/json") {
        response = response.with_header(header);
    }

    // Agregamos CORS
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Origin", b"*") {
        response = response.with_header(h);
    }

    (response.boxed(), code as u16)
}