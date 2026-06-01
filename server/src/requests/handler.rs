use tiny_http::{Server, Request, Method};
use std::sync::{Arc, Mutex};
use crate::structs::request::{RequestLog, HandlerResult};
use crate::structs::filecache::{FileCache};
use crate::requests::endpoints::{webpage, simulation, errors};
use tiny_http::Response;

const API_V1: &str = "/api/v1";

/// Recibe todas las requests y llama al metodo que corresponde
// Cada metodo, devuelve una response
// Que luego es enviada por el sender y logueada
pub fn handle_request(mut request: Request, cache: Arc<Mutex<FileCache>>) -> RequestLog {

    let result = if let Some(api_path) = request.url().strip_prefix(API_V1) {

        match (request.method(), api_path) {

            (Method::Options, "/create_path" | "/run_simulation" ) => {

                let mut response = Response::empty(200);

                if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Origin", b"*") {
                    response = response.with_header(h);
                }
                if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Methods", b"POST, GET, OPTIONS") {
                    response = response.with_header(h);
                }
                if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Headers", b"Content-Type") {
                    response = response.with_header(h);
                }

                (response.boxed(), 200)
            }

            (Method::Post, "/create_path") =>
                simulation::create_path(&mut request, cache),

            (Method::Post, "/run_simulation") =>
                simulation::run_simulation(&mut request, cache),

            _ => errors::not_found(),
        }

    } else {
        // Entrega los archivos de la web.
        webpage::get_page_file(&request)
    };

    response_sender(request, result)
} 

/// Envia la respuesta al cliente y la loguea por consola.
// Si falla, imprime 499 como status code.
fn response_sender(request: Request, result: HandlerResult) -> RequestLog {

    let method = request.method().to_string();
    let path = request.url().to_string();

    let (response, status) = result;
    let status_code = match request.respond(response) {
        Ok(_) => status,
        Err(_) => 499,
    };

    RequestLog{
        method,
        path,
        status_code,
    }
}

pub fn create_server(port: i32) -> Result<Server, Box<dyn std::error::Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(addr)?;
    Ok(server)
}