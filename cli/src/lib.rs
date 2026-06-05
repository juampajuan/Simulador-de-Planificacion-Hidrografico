pub mod requests;
pub mod interaction; 
use crate::interaction::logic;

pub fn cli_server_menu(port: i32){
    let host = format!("http://localhost:{}", port);
    loop {
        logic::menu(&host);
    }
}