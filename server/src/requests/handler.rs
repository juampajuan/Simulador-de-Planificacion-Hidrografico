use tiny_http::{Server, Request, Method};
use std::sync::{Arc, Mutex};
use crate::structs::request::{RequestLog, HandlerResult};
use crate::structs::filecache::{FileCache};
use crate::requests::endpoints::{auth, generic, limits, projects, simulation, students, webpage};
use crate::db::engine::DBEngine;
use tiny_http::Response;
use crate::structs::settings::Settings;

const API_V1: &str = "/api/v1";

pub fn handle_request(mut request: Request, cache: Arc<Mutex<FileCache>>, db: Option<DBEngine>, settings: Arc<Settings>) -> RequestLog {

    let result = if let Some(api_path) = request.url().strip_prefix(API_V1) {

        match (request.method(), api_path) {

            (Method::Options, _) => {
                (Response::empty(200).boxed(), 200)
            }

            _ => {
                let db_connection = match db {
                    Some(valid_db) => valid_db,
                    None => return response_sender(request, generic::server_error("No se pudo iniciar la db".to_string())),
                };

                match (request.method(), api_path) {
                    (Method::Post, "/create_path") =>
                        simulation::create_path(&mut request, cache),

                    (Method::Post, "/run_simulation") =>
                        simulation::run_simulation(&mut request, cache),

                    (Method::Get, "/limits") =>
                        limits::get_limits(settings),

                    (Method::Get, "/projects") =>
                        projects::get_projects(&mut request, db_connection), 

                    (Method::Post, "/projects") =>
                        projects::create(&mut request, db_connection, settings), 

                    (Method::Delete, url) if url.starts_with("/projects/") =>
                        projects::delete_project(&mut request, db_connection, settings),
                    
                    (Method::Put, url) if url.starts_with("/projects/") =>
                        projects::update_a_project(&mut request, db_connection),

                    (Method::Get, "/student_project") =>
                        projects::get_student_project(&mut request, db_connection),  

                    (Method::Get, "/students") =>
                        students::get_all_students(&mut request, db_connection),

                    (Method::Post, "/students") =>
                        students::create_new_student(&mut request, db_connection),

                    (Method::Put, "/students") => 
                        students::update_an_student(&mut request, db_connection),  

                    (Method::Post, "/auth/create_professor_user") =>
                        auth::create_professor(&mut request, db_connection),

                    (Method::Post, "/auth/change_professor_pass") =>
                        auth::change_pass(&mut request, db_connection),

                    (Method::Post, "/auth/login") =>
                        auth::login(&mut request, db_connection),

                    (Method::Post, "/auth/close_session") =>
                        auth::close_session(&mut request, db_connection),

                    (Method::Post, "/auth/close_all") =>
                        auth::close_all(&mut request, db_connection),

                    _ => generic::not_found(),
                }
            }
        }

    } else {
        webpage::get_page_file(&request)
    };

    response_sender(request, result)
}

/// Envia la respuesta al cliente y la loguea por consola.
fn response_sender(request: Request, result: HandlerResult) -> RequestLog {
    let method = request.method().to_string();
    let path = request.url().to_string();

    let (mut response, status) = result;

    // 🚀 Centralización absoluta: Todo lo que responda la API lleva estos headers obligatoriamente
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Origin", b"http://localhost.:8080") {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Credentials", b"true") {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Methods", b"POST, GET, OPTIONS, DELETE") {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Headers", b"Content-Type, Cookie") {
        response = response.with_header(h);
    }

    let status_code = match request.respond(response) {
        Ok(_) => status,
        Err(_) => 499,
    };

    RequestLog {
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