use tiny_http::{Response, Request};
use crate::structs::request::{HandlerResult};
use simulations::test;
 
pub fn get_users(request: &Request) -> HandlerResult {

    // Podes leer todo, pero no respondas aca. La respuesta, la creas y la retornas
    match request.remote_addr() {
        Some(addr) => {
            println!("Cliente: {}", addr); 
        }
        None => {
            println!("No se pudo obtener la IP");
        }
    }

    test();

    // sumilation::load_tiff(/* nombre grupo, path del tif ... */);

    // Aca, llamamos a los metodos, para simular, procesar, etc. Y armamos la response.
    let response = Response::from_string("Lista de users")
        .with_status_code(200);

    (response.boxed(), 200)
}

// pub fn sumilar() {
//     // .....
// }