use std::thread::{self, JoinHandle};
use tiny_http::Request;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use crate::structs::filecache::{FileCache};
use crate::requests::handler::handle_request;
use crate::structs::settings::Settings;
use crate::db::engine::DBEngine;
use crate::logging::{logger, structs::ThreadMessage};

// Creación de los hilos del servidor: uno por cada request entrante y uno para el CLI.

/// Lanza un hilo nuevo para atender una request: la procesa con `handle_request`
/// y al terminar imprime su log en consola. Devuelve el handle del hilo.
pub fn create_request_thread(request: Request, cache: Arc<Mutex<FileCache>>, settings: Arc<Settings>, db: Arc<Mutex<DBEngine>>, tx: Sender<ThreadMessage>) -> JoinHandle<()> {
    thread::spawn(move || {
        let log = handle_request(request, cache, db, settings, &tx);
        log.print();
        log.send_to_logger(&tx);
    })
}

/// Lanza un hilo aparte que corre el menú interactivo del CLI de administración.
pub fn create_cli_thread(port: i32) -> JoinHandle<()> {
    thread::spawn(move || {
        cli::cli_server_menu(port);
    })
}

pub fn create_logger_thread(rx: Receiver<ThreadMessage>, settings: Arc<Settings>) -> JoinHandle<()> {
    thread::spawn(move || {
        logger::logger_handler(rx, settings);
    })
}