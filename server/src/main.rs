use std::sync::Arc;

mod utils;
use utils::config_loader::{load_settings};
mod requests;
use requests::handler::{create_server};
mod threads;
use threads::creators::{create_request_thread};
mod structs;


fn main() { 
    // TODO: Generar struct para almacenar geotiffs a lo cache

    // Intentamos cargar la config y transformamos en un recurso compartido.
    let settings = match load_settings() {
        Ok(config) => Arc::new(config),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let server = match create_server(settings.port) {
        Ok(server) => server,
        Err(error) => {
            eprintln!("Error iniciando servidor: {}", error);
            return;
        }
    };

    for request in server.incoming_requests() {
        let settings_clone = Arc::clone(&settings);
        create_request_thread(request);
    }
}