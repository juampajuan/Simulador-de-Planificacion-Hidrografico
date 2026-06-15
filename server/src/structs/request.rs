use tiny_http::ResponseBox;

const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

/// Lo genera cada endpoints, para luego loggear en consola o si se decide en un archivo.
pub struct RequestLog {
    pub method: String,
    pub path: String,
    pub status_code: u16,
}

// TODO: Implementar/Agregar Error/String, como un option. Asi se puede logear lo que falla
impl RequestLog {
    pub fn print(&self) {
        let color: &str = match self.status_code {
            200..=299 => GREEN,
            300..=499 => BLUE,
            500..=599 => RED,
            _ => RESET,
        };

        println!(
            "{} {} -> {}{}{}",
            self.method,
            self.path,
            color,
            self.status_code,
            RESET
        );
    }
}

pub type HandlerResult = (ResponseBox, u16);