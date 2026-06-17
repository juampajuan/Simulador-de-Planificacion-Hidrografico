use tiny_http::{Header, Response, Request};
use crate::structs::request::{HandlerResult};
use std::fs::File;
use std::path::PathBuf;
use super::generic::{not_found, server_error};

fn content_type(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html"
    } else if path.ends_with(".js") {
        "text/javascript"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else if path.ends_with(".png") {
        "image/png"
    } else {
        "application/octet-stream"
    }
}
 
pub fn get_page_file(request: &Request) -> HandlerResult {

    let url = request.url();
 
    let path = if url == "/" || PathBuf::from(&url).extension().is_none() {
        PathBuf::from("client/dist/index.html")
    } else {
        PathBuf::from(format!("client/dist{}", url))
    };

    match File::open(&path) {
        Ok(file) => {
            
            let path_str = match path.to_str() {
                Some(path) => path,
                None => return server_error("Internal Error".to_string())
            };

            let mime = content_type(path_str);

            let header = match Header::from_bytes(
                &b"Content-Type"[..],
                mime.as_bytes(),
            ) {
                Ok(header) => header,
                Err(_) => return server_error("Internal Error".to_string())
            };

            let response = Response::from_file(file)
                .with_header(header)
                .boxed();

            return (response, 200, None);
        }

        Err(_) => return not_found(),
    };
    
}
