use tiny_http::{Response, Request};
use std::sync::{Arc, Mutex};
use crate::structs::filecache::{FileCache, DepthMatrix};
use crate::structs::request::{HandlerResult};
use simulations;
 
pub fn create_path(request: &Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {

    // Podes leer todo, pero no respondas aca. La respuesta, la creas y la retornas
    // Leo los params de requests

    // Chequeo si esta en el cache
    // let mut cache = cache.lock().unwrap();
    // cache.add(DepthMatrix { id: 1 });
    // println!("{:?}", cache.get(1));
 
    // Si no esta, la creo
    let matrix = simulations::create_depth_matrix(/* paso el path o algo */);

    let path = simulations::create_path(matrix,/* azimut */, /* separation */);
    let PNGImagePath = simulations::create_path_image(matrix, path);
 
    let response = Response::from_string(
        "Un increible path en forma de imagen"
    )
    .with_status_code(200)
    .with_header(
        tiny_http::Header::from_bytes(
            "Access-Control-Allow-Origin",
            "*"
        ).unwrap()
    )
    .with_header(
        tiny_http::Header::from_bytes(
            "Access-Control-Allow-Methods",
            "POST, OPTIONS"
        ).unwrap()
    );

    (response.boxed(), 200)
}

pub fn run_simulation(request: &Request, cache: Arc<Mutex<FileCache>>) -> HandlerResult {

    // Podes leer todo, pero no respondas aca. La respuesta, la creas y la retornas
    // Leo los params de requests
 
    // Si no esta en cache, la creo
    let matrix = simulations::create_depth_matrix(/* paso el path o algo */);

    simulations::create_path(/* paramas */);
    simulations::run_simulation(/* paramas */);
    simulations::create_simulation_image();
 
    let response = Response::from_string("Un increible path en forma de imagen")
        .with_status_code(200);

    (response.boxed(), 200)
}

 