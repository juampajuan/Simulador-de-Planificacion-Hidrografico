pub mod interaction;
pub mod requests;
use crate::interaction::logic;

/// Metodo que instancia el CLI, para ejecutar como thread
/// El mismo server, va a leventar un Hilo extra, para poder usar el CLI sin autenticar.
pub fn cli_server_menu(port: i32) {
    let host = format!("http://localhost:{}", port);

    let client = match requests::generate_client() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error creando cliente: {err}");
            return;
        }
    };

    loop {
        logic::menu(&host, &client);
    }
}
