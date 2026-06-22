use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

mod utils;
use utils::config_loader::{load_settings};
mod requests;
use requests::handler::{create_server};
mod threads;
use threads::creators::{create_request_thread, create_cli_thread};
mod structs;
use structs::filecache::{FileCache};
mod db;
use db::engine::DBEngine;
use db::queries::professor;

use crate::utils::helpers::create_dirs;

/// Punto de entrada del servidor. Carga la configuración, prepara los recursos compartidos
/// (base de datos y cache detrás de `Arc<Mutex>`), levanta el servidor HTTP y atiende cada
/// request entrante en su propio hilo, además de un hilo para el CLI.
fn main() {

    // Intentamos cargar la config y transformamos en un recurso compartido.
    let settings = match load_settings() {
        Ok(config) => Arc::new(config),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    if create_dirs(&settings.upload_path).is_none() {
        eprintln!("Error creando directorios");
        return;
    }

    let db = match DBEngine::new(&settings.db_name) {
        Ok(db) => match professor::sync_admin_password(&db, &settings.admin_pass) {
            Ok(()) => db,
            Err(err) => {
                eprintln!("Error inicializando la DB: {err}");
                return;
            }
        },
        Err(err) => {
            eprintln!("Error inicializando la DB: {err}");
            return;
        }
    };
    let db_mutex = Arc::new(Mutex::new(db));

    // Generamos el struct para hacer de cache con los geotiffs cargados.
    let file_cache = FileCache::new(settings.cache_amount);
    let cache = Arc::new(Mutex::new(file_cache));

    let server = match create_server(settings.port) {
        Ok(server) => server,
        Err(error) => {
            eprintln!("Error iniciando servidor: {}", error);
            return;
        }
    };

    println!("Server iniciado en puerto: {}", settings.port);

    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    threads.push(create_cli_thread(settings.port));

    for request in server.incoming_requests() {
        let settings_clone = Arc::clone(&settings);
        let cache_clone = Arc::clone(&cache);
        let db_clone = Arc::clone(&db_mutex);
        threads.push(create_request_thread(request, cache_clone, settings_clone, db_clone));
    }

    for thread in threads {
        if let Err(err) = thread.join() {
            eprintln!("\x1b[31mError:\x1b[0m {:?}", err);
        }
    }
}