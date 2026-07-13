use crate::structs::request::HandlerResult;
use image::{ImageFormat, RgbaImage};
use serde::de::DeserializeOwned;
use std::io::Cursor;
use tiny_http::{Request, Response};

/// Lee el cuerpo de la petición y lo parsea a la estructura correspondiente de forma segura.
/// Como esta implementado como Generic, acepta cualquier estructura que necesitemos.
pub fn parse_json_body<T: DeserializeOwned>(request: &mut Request) -> Result<T, String> {
    let mut content = String::new();

    request
        .as_reader()
        .read_to_string(&mut content)
        .map_err(|_| "Error reading body".to_string())?;

    serde_json::from_str(&content).map_err(|e| format!("Invalid JSON: {}", e))
}

/// Genera la response generica 404, cuando no encuentra el recurso.
pub fn not_found() -> HandlerResult {
    let msg = "404 no existe".to_string();
    let response = Response::from_string(msg.clone()).with_status_code(404);

    (response.boxed(), 404, Some(msg))
}

/// Genera el mensaje de error adjudicado al servidor
/// Con codigo y texto que cambia segun el error especifico.
pub fn server_error(msg: String) -> HandlerResult {
    let response = Response::from_string(msg.clone()).with_status_code(500);

    (response.boxed(), 500, Some(msg))
}

/// Genera respuestas genericas, en base a un string.
/// Usando un codigo y mensaje de texto pasados por parametro.
pub fn string_response(msg: String, code: i32) -> HandlerResult {
    let mut response = Response::from_string(msg.clone()).with_status_code(code);

    if let Ok(header) = tiny_http::Header::from_bytes(b"Content-Type", b"application/json") {
        response = response.with_header(header);
    }

    let error = if code >= 400 { Some(msg) } else { None };
    (response.boxed(), code as u16, error)
}

/// Construye una respuesta HTTP con una imagen PNG y las cabeceras CORS de forma segura.
pub fn create_png_response(rgb_image: RgbaImage) -> Response<Cursor<Vec<u8>>> {
    let mut bytes = Vec::new();

    if rgb_image
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .is_err()
    {
        return Response::from_string("Error encoding PNG").with_status_code(500);
    }

    let mut response = Response::from_data(bytes).with_status_code(200);

    if let Ok(h) = tiny_http::Header::from_bytes(b"Content-Type", b"image/png") {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Methods", b"POST, OPTIONS")
    {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Headers", b"Content-Type") {
        response = response.with_header(h);
    }

    response
}
