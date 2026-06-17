pub mod requests;
mod interaction; 
use crate::interaction::print;
use crate::interaction::logic;

fn main() {
    let (host, pass) = match logic::get_args() {
        Some(args) => args,
        None => {
            eprintln!("Ejecutar como: \x1b[36m./programa\x1b[0m \x1b[95m<host>\x1b[0m \x1b[95m<password>\x1b[0m");
            return;
        }
    };

    let client = match requests::generate_client() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error creando cliente: {err}");
            return;
        }
    };

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

    print::print_banner();
    print::print_help();
    loop {
        logic::menu(&host, &client);
    }
}