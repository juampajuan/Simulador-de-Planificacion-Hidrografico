use tiny_http::Request;
use crate::db::engine::DBEngine;
use crate::db::queries::proyects::{delete_project_by_id, get_all_by_professor_id, get_project_by_id, create_project, update_project, ProjectMetadata};
use crate::requests::endpoints::auth::{check_profesor_auth, check_student_auth};
use crate::structs::request::HandlerResult;
use crate::requests::endpoints::generic::{server_error, string_response};
use crate::requests::http_helper::parse_json_body;
use crate::structs::settings::Settings;
use std::sync::{Arc};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::db::queries::{professor, auth, student};
use serde_json;
use std::{
    fs::File,
    io::{Read},
}; 
use multipart::server::Multipart;
 

pub fn get_boundary(request: &Request) -> Result<String, &str> {

    let content_type = match request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Content-Type"))
        .map(|h| h.value.as_str())
    {
        Some(v) => v,
        None => return Err("Missing Content-Type"),
    };

    let boundary = match content_type
        .split(';')
        .find_map(|part| {
            let part = part.trim();

            if let Some(boundary) = part.strip_prefix("boundary=") {
                Some(boundary.to_string())
            } else {
                None
            }
        })
    {
        Some(b) => b,
        None => return Err("Missing boundary")
    };

    Ok(boundary)
}


pub fn create(
    request: &mut Request,
    db: DBEngine,
    settings: Arc<Settings>,
) -> HandlerResult {

    let Some(id) = check_profesor_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let boundary = match get_boundary(request) {
        Ok(b) => b,
        Err(e) => return server_error(e.into())
    };

    let mut multipart = Multipart::with_body(
        request.as_reader(),
        boundary,
    );

    let mut metadata_json = None::<String>;
    let mut filename_saved = None::<String>;

    // TODO: Re hacer el codigo mas feo que hice en mi vida
    // Transformarlo en un metodo, no iterativo, solo hay 2 entries que leer. 
    // Y hacer que comprueba si el tipo de archivo es .geotiff
    if let Err(e) = multipart.foreach_entry(|mut field| {

        match field.headers.name.as_ref() {

                "metadata" => {
                    let mut json = String::new();
                    let _ = field.data.read_to_string(&mut json);

                    metadata_json = Some(json);
                }

                "file" => {
                    let original_filename = field.headers
                        .filename
                        .clone()
                        .unwrap_or_else(|| "upload.bin".to_string());

                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let path = Path::new(&original_filename);

                    let filename = match (
                        path.file_stem().and_then(|s| s.to_str()),
                        path.extension().and_then(|s| s.to_str()),
                    ) {
                        (Some(stem), Some(ext)) => format!("{}_{}.{}", stem, timestamp, ext),
                        (Some(stem), None) => format!("{}_{}", stem, timestamp),
                        _ => format!("upload_{}.bin", timestamp),
                    };


                    let mut out = File::create(
                        format!("{}/geotiffs/{}", settings.upload_path, filename)
                    ).unwrap();

                    let _ = std::io::copy(
                        &mut field.data,
                        &mut out,
                    );

                    filename_saved = Some(filename);

                    
                }

                _ => {}
        }

    }) {
        return server_error("No se pudo subir el archivo".to_string());
    }

    let json = match metadata_json {
        Some(j) => j,
        None => return server_error("No hay metadata.".to_string()),
    };

    let meta: ProjectMetadata = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => return server_error("Metadata incompleta.".to_string()),
    };

    match create_project(&db, &filename_saved.unwrap(), id,&meta) {
        Ok(_) => string_response("ok".into(), 200),
        Err(_) => server_error("NO pudo".into())
    }
    
}

pub fn get_projects(request: &mut Request, db: DBEngine) -> HandlerResult {
  
    let Some(id) = check_profesor_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let Ok(projects) = get_all_by_professor_id(&db, id) else {
        return server_error("Error al obtener los proyectos".to_string());
    };

    let response = match serde_json::to_string(&projects) {
        Ok(json) => json,
        Err(_) => return server_error("Error serializing limits data".to_string()),
    };

    string_response(response, 200)
}

pub fn get_student_project(request: &mut Request, db: DBEngine) -> HandlerResult {
  
    let Some(id) = check_student_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let Ok(projects) = get_project_by_id(&db, id) else {
        return server_error("Error al obtener los proyectos".to_string());
    };

    let Some(project) = projects else {
        return string_response("Proyecto no encontrado".to_string(), 404);
    };

    let response = match serde_json::to_string(&project) {
        Ok(json) => json,
        Err(_) => return server_error("Error serializing limits data".to_string()),
    };

    string_response(response, 200)
}

pub fn delete_project(request: &mut Request, db: DBEngine, settings: Arc<Settings>) -> HandlerResult {

    let Some(professor_id) = check_profesor_auth(request, &db) else {
            return string_response("Sin autorizar".to_string(), 401);
        };

    let Some(id_str) = request.url().rsplit('/').next() else {
        return string_response("Ruta inválida".to_string(), 400);
    };

    let Ok(id) = id_str.parse::<i64>() else {
        return string_response("ID inválido".to_string(), 400);
    };
    
    let Ok(projects) = get_project_by_id(&db, id) else {
        return server_error("Error al obtener los proyectos".to_string());
    };
    
    let Some(project) = projects else {
        return string_response("Proyecto no encontrado".to_string(), 404);
    };

    let filename = project.filename;

    match delete_project_by_id(&db, id, professor_id) {
        Ok(true) => {
            let path = format!("{}/geotiffs/{}", settings.upload_path, filename);
            let _ = std::fs::remove_file(&path);
            string_response("Proyecto eliminado.".to_string(), 200)
        }
        Ok(false) => string_response(
                "Proyecto no encontrado".to_string(),
                404,
        ),
        Err(_) => server_error(
                "Error al eliminar el proyecto".to_string(),
            )
    }

}

pub fn update_a_project(
    request: &mut Request,
    db: DBEngine,
) -> HandlerResult {

    let Some(professor_id) = check_profesor_auth(request, &db) else {
        return string_response("Sin autorizar".to_string(), 401);
    };

    let id_str = match request.url().rsplit('/').next() {
        Some(id) => id,
        None => return string_response("Ruta inválida".to_string(), 400),
    };

    let id = match id_str.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return string_response("ID inválido".to_string(), 400),
    };

    let meta: ProjectMetadata = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    match update_project(&db, id, professor_id, &meta) {
        Ok(true) => string_response("Proyecto actualizado.".to_string(), 200),
        Ok(false) => string_response("Proyecto no encontrado.".to_string(), 404),
        Err(_) => server_error("Error al actualizar el proyecto.".to_string()),
    }
}