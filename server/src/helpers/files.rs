use std::{
    collections::HashSet,
    error::Error,
    fs,
    path::{PathBuf,Component,Path},
    sync::{Arc, Mutex},
};

use crate::{db::{engine::DBEngine, queries_interface::student_simulations::get_all_simulation_images_locked}, structs::settings::Settings};

use tiny_http::{Header, Response};
use crate::structs::request::HandlerResult;
use std::fs::File;
use crate::requests::endpoints::generic::{not_found, server_error};

///Selecciona el tipo de contenido/archivo basandose en el path.
pub fn content_type(ext: &str) -> &'static str {
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

/// Es el metodo encargado de buscar, leer y entregar el archivo.
/// Usado por los 2 pseudowrappers de arriba.
pub fn serve_file(path: PathBuf) -> HandlerResult {
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
pub fn get_relative_path(url: &str) -> Option<&Path> {
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

pub fn clean_unused_images(
    db: &Arc<Mutex<DBEngine>>,
    settings: &Settings,
) -> Result<(), Box<dyn Error>> {
    let images: HashSet<String> = get_all_simulation_images_locked(db)?;

    let images_dir = PathBuf::from(&settings.storage_path).join("images");

    for entry in fs::read_dir(&images_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if path.extension().and_then(|e| e.to_str()) != Some("png") {
            continue;
        }

        let Some(filename) = path.file_name().and_then(|f| f.to_str()) else {
            continue;
        };

        if !images.contains(filename) {
            // TODO: Logear como debug.
            println!("Deleting unused image: {}", filename);
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}