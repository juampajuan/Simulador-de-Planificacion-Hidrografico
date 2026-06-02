use tiny_http::{Response, Request};
use serde::de::DeserializeOwned;
use std::io::{Cursor};
use image::{RgbImage, ImageFormat};

/// Lee el cuerpo de la petición y lo parsea a la estructura correspondiente de forma segura.
pub fn parse_json_body<T: DeserializeOwned>(request: &mut Request) -> Result<T, String> {
    let mut content = String::new();
    
    request.as_reader().read_to_string(&mut content)
        .map_err(|_| "Error reading body".to_string())?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Invalid JSON: {}", e))
}

/// Construye una respuesta HTTP con una imagen PNG y las cabeceras CORS de forma segura.
pub fn create_png_response(rgb_image: RgbImage) -> Response<Cursor<Vec<u8>>> {
    let mut bytes = Vec::new();
    
    if let Err(_) = rgb_image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png) {
        return Response::from_string("Error encoding PNG").with_status_code(500);
    }

    let mut response = Response::from_data(bytes).with_status_code(200);

    if let Ok(h) = tiny_http::Header::from_bytes(b"Content-Type", b"image/png") {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Origin", b"*") {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Methods", b"POST, OPTIONS") {
        response = response.with_header(h);
    }
    if let Ok(h) = tiny_http::Header::from_bytes(b"Access-Control-Allow-Headers", b"Content-Type") {
        response = response.with_header(h);
    }

    response
}