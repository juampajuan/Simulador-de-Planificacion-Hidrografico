pub mod requests;
pub mod interaction; 
use crate::interaction::logic;

pub fn cli_server_menu(port: i32){
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