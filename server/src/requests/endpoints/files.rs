use tiny_http::{Header, Response, Request};
use crate::structs::{request::HandlerResult, settings::Settings};
use std::{fs::File, sync::Arc};
use std::path::{PathBuf, Component, Path};
use super::generic::{not_found, server_error};

///Selecciona el tipo de contenido/archivo basandose en el path.
fn content_type(ext: &str) -> &'static str {
    match ext {
        "html" => "text/html",
        "js" => "text/javascript",
        "css" => "text/css",
        "wasm" => "application/wasm",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "tif" | "tiff" => "image/tiff",
        _ => "application/octet-stream",
    }
}

/// Dada una url retorna imagen o geotiff con el nombre.
/// SOLO busca archivos en la carpeta establecida como /storage en las settings.
/// Este metodo es usado para entregar 
/// 1. Las imagenes de cada intento de simulacion.
/// 2. Archivos geotiff para las entregas.
pub fn get_file_from_storage(
    request: &Request,
    settings: Arc<Settings>,
) -> HandlerResult {
    let relative = match get_relative_path(request.url()) {
        Some(path) => path,
        None => return not_found(),
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
        None => return not_found(),
    };

    let path = if url == "/" || relative.extension().is_none() {
        PathBuf::from("client/dist/index.html")
    } else {
        PathBuf::from("client/dist").join(relative)
    };

    serve_file(path)
}


/// Es el metodo encargado de buscar, leer y entregar el archivo.
/// Usado por los 2 pseudowrappers de arriba.
fn serve_file(path: PathBuf) -> HandlerResult {
    match File::open(&path) {
        Ok(file) => {
            let mime = content_type(
                path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or(""),
            );

            let header = match Header::from_bytes(
                &b"Content-Type"[..],
                mime.as_bytes(),
            ) {
                Ok(header) => header,
                Err(_) => return server_error("Internal Error".to_string()),
            };

            let response = Response::from_file(file)
                .with_header(header)
                .boxed();

            (response, 200, None)
        }

        Err(_) => not_found(),
    }
}

/// Obtiene el path relativo
/// Evita que se pueda acceder a archivos fuera del dict root.
    /// Es decir: No podra hacer root/../../archivo_secreto.txt
fn get_relative_path(url: &str) -> Option<&Path> {
    let relative = Path::new(url.trim_start_matches('/'));

    if relative
        .components()
        .any(|c| matches!(c, Component::ParentDir))
    {
        None
    } else {
        Some(relative)
    }
}