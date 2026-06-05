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
// use db::engine::DBEngine;

fn main() {

    // Intentamos cargar la config y transformamos en un recurso compartido.
    let settings = match load_settings() {
        Ok(config) => Arc::new(config),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    // Generamos el struct para hacer de cache con los geotiffs cargados.
    let geotiff_cache = FileCache::new(settings.cache_amount);
    let cache = Arc::new(Mutex::new(geotiff_cache));

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
        threads.push(create_request_thread(request, cache_clone, settings_clone));
    }

    for thread in threads {
        if let Err(err) = thread.join() {
            eprintln!("\x1b[31mError:\x1b[0m {:?}", err);
        }
    }
}