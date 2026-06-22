pub mod requests;
mod interaction; 
use crate::interaction::print;
use crate::interaction::logic;

fn main() {

    // Obtiene args al ejecutar
    let (host, pass) = match logic::get_args() {
        Some(args) => args,
        None => {
            eprintln!("Ejecutar como: \x1b[36m./programa\x1b[0m \x1b[95m<host>\x1b[0m \x1b[95m<password>\x1b[0m");
            return;
        }
    };

    // Genera el cliente para realizar las requests
    let client = match requests::generate_client() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error creando cliente: {err}");
            return;
        }
    };

    // Se autentica como admin, usando la pass introducida como arg
    match requests::login(&host, &pass, &client) {
        Ok((response, code)) if code == 200 => (response, code),
        Ok((response, code)) => {
            eprintln!("Login falló (HTTP {}): {}", code, response);
            return;
        }
        Err(err) => {
            eprintln!("Error haciendo login: {err}");
            return;
        }
    };

    // Se presenta el menu y se queda a la espera de un input
    print::print_banner();
    print::print_help();
    loop {
        logic::menu(&host, &client);
    }
}