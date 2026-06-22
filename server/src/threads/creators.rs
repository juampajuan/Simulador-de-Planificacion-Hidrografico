use std::thread::{self, JoinHandle};
use tiny_http::Request;
use std::sync::{Arc, Mutex};
use crate::structs::filecache::{FileCache};
use crate::requests::handler::handle_request;
use crate::structs::settings::Settings;
use crate::db::engine::DBEngine;

// Creación de los hilos del servidor: uno por cada request entrante y uno para el CLI.

/// Lanza un hilo nuevo para atender una request: la procesa con `handle_request`
/// y al terminar imprime su log en consola. Devuelve el handle del hilo.
pub fn create_request_thread(request: Request, cache: Arc<Mutex<FileCache>>, settings: Arc<Settings>, db: Arc<Mutex<DBEngine>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let log = handle_request(request, cache, db, settings);
        log.print();
    })
}

/// Lanza un hilo aparte que corre el menú interactivo del CLI de administración.
pub fn create_cli_thread(port: i32) -> JoinHandle<()> {
    thread::spawn(move || {
        cli::cli_server_menu(port);
    })
}