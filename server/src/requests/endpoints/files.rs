use crate::db::engine::DBEngine;
use crate::helpers::auth::is_admin_request;
use crate::helpers::files;
use crate::helpers::files::{get_relative_path, serve_file};
use crate::logging::logger::send_message_to_logger;
use crate::logging::structs::{LogType, ThreadMessage};
use crate::requests::http_utils;
use crate::structs::{request::HandlerResult, settings::Settings};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use tiny_http::Request;

/// Dada una url retorna imagen o geotiff con el nombre.
/// SOLO busca archivos en la carpeta establecida como /storage en las settings.
/// Este metodo es usado para entregar
/// 1. Las imagenes de cada intento de simulacion.
/// 2. Archivos geotiff para las entregas.
pub fn get_file_from_storage(request: &Request, settings: Arc<Settings>) -> HandlerResult {
    let relative = match get_relative_path(request.url()) {
        Some(path) => path,
        None => return http_utils::not_found(),
    };

    serve_file(PathBuf::from(&settings.storage_path).join(relative))
}

/// Dada una url retorna el archivo relacionado a esta.
/// Este metodo es usado para server la pagina web propiamente dicha.
/// Cuando en `handle_request` el match no accedo a algun endpoint, la request llega a este metodo
/// Que intentara buscar un archivo dentro del path donde se aloja el front, del archivo buscado.
pub fn get_page_file(request: &Request) -> HandlerResult {
    let url = request.url();

    let relative = match get_relative_path(url) {
        Some(path) => path,
        None => return http_utils::not_found(),
    };

    let path = if url == "/" || relative.extension().is_none() {
        PathBuf::from("client/dist/index.html")
    } else {
        PathBuf::from("client/dist").join(relative)
    };

    serve_file(path)
}

/// Limpia los archivos (imagenes), creados en cada simulacion.
// Los mismos se mantienen para las entregas y una vez borrado el proyecto o alumno, se pueden eliminar.
pub fn clean_temp_files(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    settings: &Arc<Settings>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "Intentando limpiar imágenes temporales de simulaciones".to_string(),
        LogType::Debug,
    );

    match is_admin_request(request, &db) {
        Ok(true) => {}
        Ok(false) => {
            send_message_to_logger(
                tx,
                "Intento de limpieza de imágenes sin permisos de administrador.".to_string(),
                LogType::Warn,
            );
            return http_utils::string_response(
                "Solo permitido para administradores.".to_string(),
                403,
            );
        }
        Err(_err) => return http_utils::server_error("Error autenticando".into()),
    }

    match files::clean_unused_images(&db, settings, tx) {
        Ok(()) => {
            send_message_to_logger(
                tx,
                "Se limpiaron las imágenes temporales correctamente.".to_string(),
                LogType::Info,
            );
            http_utils::string_response("Imagenes borradas correctamente".into(), 200)
        }
        Err(_) => http_utils::server_error("No se pudo borrar las imagenes".to_string()),
    }
}
