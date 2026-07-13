use std::sync::{Arc, Mutex, mpsc};
use std::thread::JoinHandle;

mod requests;
use requests::handler::create_server;
mod threads;
use threads::creators::{create_cli_thread, create_logger_thread, create_request_thread};
mod structs;
use structs::filecache::FileCache;
mod db;
use db::{engine::DBEngine, queries::professor};
mod helpers;
use helpers::{config_loader::load_settings, utils::create_dirs};
mod logging;
use crate::logging::logger::info_logger;
use crate::logging::{logger::error_logger, structs::ThreadMessage};

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

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    // Genero el thread que se encarga de loggear.
    let (tx, rx) = mpsc::channel::<ThreadMessage>();
    threads.push(create_logger_thread(rx, Arc::clone(&settings)));
    let err_logger = error_logger(&tx, "On main");

    // Creamos el directorio donde se van a subir los archivo .tif
    if create_dirs(&settings.storage_path).is_none() {
        err_logger("Error creando directorios");
        return;
    }

    // Levantamos la DB y aplicamos el schema
    // Ademas, si corresponde, actualizamos la pass del Admin
    let db = match DBEngine::new(&settings.db_name) {
        Ok(db) => match professor::sync_admin_password(&db, &settings.admin_pass) {
            Ok(()) => db,
            Err(err) => {
                err_logger(format!("Error inicializando la DB: {err}").as_str());
                return;
            }
        },
        Err(err) => {
            err_logger(format!("Error inicializando la DB: {err}").as_str());
            return;
        }
    };
    let db_mutex = Arc::new(Mutex::new(db));

    // Generamos el struct para hacer de cache con los geotiffs cargados.
    let file_cache = FileCache::new(settings.cache_amount);
    let cache = Arc::new(Mutex::new(file_cache));

    // Levantamos Listener HTTP
    let server = match create_server(settings.port) {
        Ok(server) => server,
        Err(error) => {
            err_logger(format!("Error iniciando servidor: {}", error).as_str());
            return;
        }
    };

    {
        let info_log = info_logger(&tx, "On main");
        info_log(&format!("Server iniciado en puerto: {}", settings.port));
    }

    // Generamos el thread para el CLI
    threads.push(create_cli_thread(settings.port));

    // Escuchamos conexiones nuevas y ante cada una
    // Clonamos las estrucutas a ompartir
    // Levamantamos un hilo de ejecucion.
    for request in server.incoming_requests() {
        let settings_clone = Arc::clone(&settings);
        let cache_clone = Arc::clone(&cache);
        let db_clone = Arc::clone(&db_mutex);
        let tx_clone = tx.clone();
        threads.push(create_request_thread(
            request,
            cache_clone,
            settings_clone,
            db_clone,
            tx_clone,
        ));
    }

    // Esperamos a que hagan JOIN todos los hilos en ejecuion.
    for thread in threads {
        if let Err(err) = thread.join() {
            err_logger(format!("Error joineando el thread: {:?}", err).as_str());
        }
    }
}
