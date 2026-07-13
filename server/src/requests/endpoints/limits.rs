use crate::requests::http_utils;
use crate::structs::request::HandlerResult;
use crate::structs::settings::Settings;
use std::sync::Arc;

/// Retorna los limites configurados para cada proyecto.
/// Estos cubren desde limites en frecuencias hasta el azimut del alumno.
pub fn get_limits(settings: Arc<Settings>) -> HandlerResult {
    let response = match serde_json::to_string(&*settings) {
        Ok(json) => json,
        Err(_) => return http_utils::server_error("Error serializing limits data".to_string()),
    };

    http_utils::string_response(response, 200)
}
