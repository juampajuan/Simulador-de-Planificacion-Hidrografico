use tiny_http::{Response};
use crate::structs::request::{HandlerResult};

pub fn not_found() -> HandlerResult {
    let response = Response::from_string("404 no existe")
        .with_status_code(404);

    (response.boxed(), 404)
}

pub fn server_error(msg: String) -> HandlerResult {
    let response = Response::from_string(msg)
        .with_status_code(500);

    (response.boxed(), 500)
}