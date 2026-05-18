use tiny_http::{Response, Request};
use crate::structs::request::{HandlerResult};
 
pub fn get_page_file(request: &Request) -> HandlerResult {

    // Cuando el front este terminado. Aca hay que servir el HTML, asi se puede acceder a todo directo.
    let response = Response::from_string(format!("Debe servir el archivo asociado al path: \"{}\". \nEl index.html si es \"/\" \nO un 404 si no existe.", request.url()))
        .with_status_code(200);

    // url:8000/ -> index.html
    // url:8000/algo.js -> algo.js
    // url:8000/style.css -> style.css

    (response, 200)
}

pub fn not_found() -> HandlerResult {

    let response = Response::from_string("404 no existe")
        .with_status_code(404);

    (response, 404)
}


// Ejmplo que encontre, sobre como servir los archivos del front.
// Para usar de referencia, como extrae y busca y devuelve el archivo en cuestion.

// use std::fs::File;
// use std::path::PathBuf;

// use tiny_http::{Header, Response, Server, StatusCode};

// fn content_type(path: &str) -> &'static str {
//     if path.ends_with(".html") {
//         "text/html"
//     } else if path.ends_with(".js") {
//         "text/javascript"
//     } else if path.ends_with(".css") {
//         "text/css"
//     } else if path.ends_with(".wasm") {
//         "application/wasm"
//     } else if path.ends_with(".png") {
//         "image/png"
//     } else {
//         "application/octet-stream"
//     }
// }

// fn main() {
//     let server = Server::http("0.0.0.0:8000").unwrap();

//     println!("http://localhost:8000");

//     for request in server.incoming_requests() {
//         let url = request.url();

//         // "/" -> "index.html"
//         let path = if url == "/" {
//             PathBuf::from("dist/index.html")
//         } else {
//             PathBuf::from(format!("dist{}", url))
//         };

//         match File::open(&path) {
//             Ok(file) => {
//                 let mime = content_type(path.to_str().unwrap());

//                 let response = Response::from_file(file).with_header(
//                     Header::from_bytes(
//                         &b"Content-Type"[..],
//                         mime.as_bytes(),
//                     )
//                     .unwrap(),
//                 );

//                 request.respond(response).unwrap();
//             }

//             Err(_) => {
//                 let response = Response::from_string("404")
//                     .with_status_code(StatusCode(404));

//                 request.respond(response).unwrap();
//             }
//         }
//     }
// }