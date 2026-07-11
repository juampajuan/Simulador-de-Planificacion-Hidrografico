use crate::db::engine::DBEngine;
use crate::logging::structs::ThreadMessage;
use crate::requests::endpoints::{
    auth, exams, files, generic, limits, projects, simulation, students,
};
use crate::structs::filecache::FileCache;
use crate::structs::request::{HandlerResult, RequestLog};
use crate::structs::settings::Settings;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use tiny_http::Response;
use tiny_http::{Method, Request, Server};

const API_V1: &str = "/api/v1";

// Router principal del servidor. Toma cada request entrante, la dirige al endpoint
// correspondiente según método HTTP y ruta, y se encarga de enviar la respuesta al cliente.

/// Punto de entrada de cada request. Las rutas bajo `/api/v1` se rutean por (método, path)
/// al endpoint que corresponda; las requests OPTIONS se responden con 200 (preflight de CORS)
/// y cualquier ruta desconocida cae en un 404. Las URLs que no son de la API se sirven como
/// archivos de la página web. Finalmente delega el envío y el logging en `response_sender`.
pub fn handle_request(
    mut request: Request,
    cache: Arc<Mutex<FileCache>>,
    db: Arc<Mutex<DBEngine>>,
    settings: Arc<Settings>,
    tx: &Sender<ThreadMessage>,
) -> RequestLog {
    let result = match request.url().strip_prefix(API_V1) {
        Some(api_path) => {
            let path_clean = api_path.split('?').next().unwrap_or(api_path);

            match (request.method(), path_clean) {
                (Method::Options, _) => (Response::empty(200).boxed(), 200, None),

                (Method::Get, "/exams/my_simulations") => {
                    exams::get_my_simulations(&mut request, db, tx)
                }

                (Method::Post, "/exams/select_delivery") => {
                    exams::select_exam_simulation(&mut request, db, tx)
                }

                (Method::Post, "/create_path") => {
                    simulation::create_path(&mut request, cache, db, settings, tx)
                }

                (Method::Post, "/run_simulation") => {
                    simulation::run_simulation(&mut request, cache, db, settings, tx)
                }

                (Method::Post, "/coverage_image") => {
                    simulation::create_coverage_image(&mut request, cache, db, settings, tx)
                }

                (Method::Get, "/limits") => limits::get_limits(settings),

                (Method::Get, "/projects") => projects::get_projects(&mut request, db, tx),

                (Method::Post, "/projects") => projects::create(&mut request, db, settings, tx),

                (Method::Delete, url) if url.starts_with("/projects/") => {
                    projects::delete_project(&mut request, db, settings, tx)
                }

                (Method::Put, url) if url.starts_with("/projects/") => {
                    projects::update_a_project(&mut request, db, tx)
                }

                (Method::Get, "/student_project") => {
                    projects::get_student_project(&mut request, db, settings, tx)
                }

                (Method::Get, "/students") => students::get_all_students(&mut request, db, tx),

                (Method::Post, "/students") => students::create_new_student(&mut request, db, tx),

                (Method::Delete, url) if url.starts_with("/students/") => {
                    students::delete_a_student(&mut request, db, settings, tx)
                }

                (Method::Put, url) if url.starts_with("/students/") => {
                    students::update_an_student(&mut request, db, tx)
                }

                (Method::Post, "/auth/create_professor_user") => {
                    auth::create_professor(&mut request, db, tx)
                }

                (Method::Post, "/auth/change_professor_pass") => {
                    auth::change_pass(&mut request, db, tx)
                }

                (Method::Post, "/auth/login") => auth::login(&mut request, db, tx),

                (Method::Post, "/auth/close_session") => auth::close_session(&mut request, db, tx),

                (Method::Post, "/auth/close_all") => auth::close_all(&mut request, db, tx),

                (Method::Post, "/clean_files") => {
                    files::clean_temp_files(&mut request, db, &settings, tx)
                }

                _ => generic::not_found(),
            }
        }

        None if request.url().starts_with("/images/") => {
            files::get_file_from_storage(&request, settings)
        }
        None if request.url().starts_with("/simulations/") => {
            files::get_file_from_storage(&request, settings)
        }
        _ => files::get_page_file(&request),
    };

    response_sender(request, result)
}

/// Recibe la respuesta generada por los metodos dentro del `handle_request` y la envia.
/// Ademas, instancia un `RequestLog` y lo retorna para ser impreso por la terminal.
fn response_sender(request: Request, result: HandlerResult) -> RequestLog {
    let method = request.method().to_string();
    let path = request.url().to_string();

    let (mut response, status, error) = result;

    if let Ok(h) =
        tiny_http::Header::from_bytes(b"Access-Control-Allow-Origin", b"http://localhost:8080")
    {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Credentials", b"true") {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(
        b"Access-Control-Allow-Methods",
        b"POST, GET, OPTIONS, DELETE, PUT",
    ) {
        response = response.with_header(h);
    }
    if let Ok(h) =
        tiny_http::Header::from_bytes(b"Access-Control-Allow-Headers", b"Content-Type, Cookie")
    {
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
        error,
    }
}

/// Levanta el servidor HTTP escuchando en `0.0.0.0:<port>` (todas las interfaces).
pub fn create_server(port: i32) -> Result<Server, Box<dyn std::error::Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(addr)?;
    Ok(server)
}
