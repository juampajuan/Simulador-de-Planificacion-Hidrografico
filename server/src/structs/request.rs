/// Lo genera cada endpoints, para luego loggear en consola o si se decide en un archivo.
pub struct RequestLog {
    pub method: String,
    pub path: String,
    pub status_code: u16,
}

// TODO: Implementar/Agregar Error/String, como un option. Asi se puede loguear lo que falla.
impl RequestLog {

    pub fn print(&self) {

        let color = match self.status_code {
            200..=299 => "\x1b[32m", // verde
            300..=499 => "\x1b[34m", // azul 
            500..=599 => "\x1b[31m", // rojo
            _ => "\x1b[0m",
        };

        let reset = "\x1b[0m";

        println!(
            "{} {} -> {}{}{}",
            
            self.method,
            self.path,
            color,
            self.status_code,
            reset
        );
    }
}

use tiny_http::{Response};
pub type HandlerResult = (Response<std::io::Cursor<Vec<u8>>>, u16);