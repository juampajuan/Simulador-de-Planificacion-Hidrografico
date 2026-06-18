use tiny_http::{Response};
use crate::structs::request::{HandlerResult};
 
pub fn not_found() -> HandlerResult {
    let msg = "404 no existe".to_string();
    let response = Response::from_string(msg.clone())
        .with_status_code(404);
 
    (response.boxed(), 404, Some(msg))
}
 
pub fn server_error(msg: String) -> HandlerResult {
    let response = Response::from_string(msg.clone())
        .with_status_code(500);
 
    (response.boxed(), 500, Some(msg))
}
 
pub fn string_response(msg: String, code: i32) -> HandlerResult {
    let mut response = Response::from_string(msg.clone())
        .with_status_code(code);
 
    if let Ok(header) = tiny_http::Header::from_bytes(b"Content-Type", b"application/json") {
        response = response.with_header(header);
    }
 
    let error = if code >= 400 { Some(msg) } else { None };
    (response.boxed(), code as u16, error)
}