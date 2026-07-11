use std::sync::mpsc::Sender;

use tiny_http::ResponseBox;
use crate::logging::logger::send_message_to_logger;
use crate::logging::structs::LogType;
use crate::{db::queries::student::Student, logging::structs::ThreadMessage};
use crate::db::queries::proyects::AdminProjectView;
use common::{StudentMeasuringParameters, PathParameters};
 
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
 
/// Lo genera cada endpoints, para luego loggear en consola.
/// Sirve para obtener informacion en tiempo real de que esta haciendo el servidor.
pub struct RequestLog {
    pub method: String,
    pub path: String,
    pub status_code: u16,
    pub error: Option<String>,
}
 
impl RequestLog {
    /// Imprime el log en consola coloreado según el código de estado (verde 2xx, azul 3xx/4xx, rojo 5xx).
    pub fn print(&self, simplified: bool) {

        if !simplified {
            return;
        }

        let color: &str = match self.status_code {
            200..=299 => GREEN,
            300..=499 => BLUE,
            500..=599 => RED,
            _ => RESET,
        };
 
        let error_str = match &self.error {
            Some(err) => format!(" - {}", err),
            None => String::new(),
        };
 
        println!(
            "{} {} -> {}{}{}{}",
            self.method,
            self.path,
            color,
            self.status_code,
            error_str,
            RESET
        );
    }

    pub fn send_to_logger(&self, tx: &Sender<ThreadMessage>) {
        let log_type = match self.status_code {
            200..=299 => LogType::Info,
            300..=499 => LogType::Warn,
            500..=599 => LogType::Error,
            _ => LogType::Info,
        };
 
        let error_str = match &self.error {
            Some(err) => format!(" - {}", err),
            None => String::new(),
        };
 
        let msg = format!(
            "{} {} -> {}{}",
            self.method,
            self.path,
            self.status_code,
            error_str,
        );

        send_message_to_logger(tx, msg, log_type);
    }
}

/// Lo que devuelve cada endpoint: la respuesta a enviar, el código de estado HTTP
/// y un mensaje de error opcional para el logging.
pub type HandlerResult = (ResponseBox, u16, Option<String>);

/// Body que manda el alumno al pedir una simulación: los parámetros de ecosonda
/// (opcionales, según el endpoint) y los parámetros del recorrido.
#[derive(serde::Deserialize)]
pub struct FullSimulationRequest {
    #[serde(default)]
    pub echo_parameters: Option<StudentMeasuringParameters>,
    pub path_parameters: PathParameters,
}

pub struct RequestContext {
    pub file_path: String,
    pub data: FullSimulationRequest,
    pub student_id: i64,
    pub student: Student, 
    pub project: AdminProjectView,
    pub project_id: i64,
}