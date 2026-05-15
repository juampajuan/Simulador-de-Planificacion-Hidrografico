mod requests;
use requests::handler::{create_server};
mod threads;
use threads::creators::{create_request_thread};
mod structs;

fn main() {
    // TODO: Abrir un settings y cargarlas
    // TODO: Generar struct para almacenar geotiffs a lo cache
    // Generar recurso compartido.

    let server = match create_server(/* puerto */) {
        Ok(server) => server,
        Err(error) => {
            eprintln!("Error iniciando servidor: {}", error);
            return;
        }
    };

    for request in server.incoming_requests() {
        create_request_thread(request);
    }
}