use tiny_http::{Server, Request, Method};
use std::sync::{Arc, Mutex};
use crate::structs::request::{RequestLog, HandlerResult};
use crate::structs::filecache::{FileCache};
use crate::requests::endpoints::{auth, generic, limits, projects, simulation, students, webpage};
use crate::db::engine::DBEngine;
use tiny_http::Response;
use crate::structs::settings::Settings;

const API_V1: &str = "/api/v1";

/// Recibe todas las requests y llama al metodo que corresponde
// Cada metodo, devuelve una response
// Que luego es enviada por el sender y logueada
pub fn handle_request(mut request: Request, cache: Arc<Mutex<FileCache>>, db: Option<DBEngine>, settings: Arc<Settings>) -> RequestLog {

    let db = match db {
        Some(db) => db,
        None => {
            let result = generic::server_error("No se pudo iniciar la db".to_string());
            return response_sender(request, result)
        }
    };

    let result = if let Some(api_path) = request.url().strip_prefix(API_V1) {

        match (request.method(), api_path) {

            (Method::Options, "/create_path" | "/run_simulation" | "/limits" ) => {

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

            (Method::Get, "/limits") =>
                limits::get_limits(settings),

            (Method::Get, "/projects") =>
                projects::get_projects(&mut request, db), 

            (Method::Post, "/projects") =>
                projects::create(&mut request, db, settings), 

            (Method::Delete, url) if url.starts_with("/projects/") =>
                projects::delete_project(&mut request, db, settings),
            
            (Method::Put, url) if url.starts_with("/projects/") =>
                projects::update_a_project(&mut request, db),

            (Method::Get, "/student_project") =>
                projects::get_student_project(&mut request, db),  

            (Method::Get, "/students") =>
                students::get_all_students(&mut request, db),

            (Method::Post, "/students") =>
                students::create_new_student(&mut request, db),

            (Method::Put, "/students") => 
                students::update_an_student(&mut request, db),  

            // TODO: Yo sapararia por dominio esto. En otros arhcivos?
                // Y agregas un nivel mas.
                // OSea path /simulation/ <todas las apis para simular>
                // /projects/ ...
                // /auth/ .. Todo lo relacionado a autenticarse

            // Auth requests methods.
            (Method::Post, "/auth/create_professor_user") =>
                auth::create_professor(&mut request, db),

            (Method::Post, "/auth/change_professor_pass") =>
                auth::change_pass(&mut request, db),

            (Method::Post, "/auth/login") =>
                auth::login(&mut request, db),

            (Method::Post, "/auth/close_session") =>
                auth::close_session(&mut request, db),

            (Method::Post, "/auth/close_all") =>
                auth::close_all(&mut request, db),

            _ => generic::not_found(),
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