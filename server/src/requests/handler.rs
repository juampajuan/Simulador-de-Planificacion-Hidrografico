use tiny_http::{Server, Request, Method};
use std::sync::{Arc, Mutex};
use crate::structs::request::{RequestLog, HandlerResult};
use crate::structs::filecache::{FileCache, DepthMatrix};
use crate::requests::endpoints::{webpage, users};

const API_V1: &str = "/api/v1";

/// Recibe todas las requests y llama al metodo que corresponde
// Cada metodo, devuelve una response
// Que luego es enviada por el sender y logueada
pub fn handle_request(request: Request, cache: Arc<Mutex<FileCache>>) -> RequestLog {

    // Asi se usa.
    // Estaria probarlo un poco mas.
    // let mut cache = cache.lock().unwrap();
    // cache.add(DepthMatrix { id: 1 });
    // println!("{:?}", cache.get(1));
    // println!("{:?}", cache.get(2));
 
    // Si se agrega otro versionado de apis, es tan facil, como agregar el `elseif` correspondiente.
    let result = if let Some(api_path) = request.url().strip_prefix(API_V1) {

        // AYUDA: En este match se agrega cada endpoint nueveo.
        // El de users, es un ejemplo a eliminar. 
        match (request.method(), api_path) {
            (Method::Get, "/users") => users::get_users(&request),
            _ => webpage::not_found(),
        }

    } else {
        webpage::get_page_file(&request)
    };

    // ---- http//sdadad.com/ --> la pagina
    // ---- http//sdadad.com/api/v1/users --> GET all users

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