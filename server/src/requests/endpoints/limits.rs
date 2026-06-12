use crate::structs::request::HandlerResult;
use crate::requests::endpoints::generic::{server_error, string_response};
use crate::structs::settings::Settings;
use std::sync::{Arc};


pub fn get_limits(settings: Arc<Settings>) -> HandlerResult {
    let response = match serde_json::to_string(&*settings) {
        Ok(json) => json,
        Err(_) => return server_error("Error serializing limits data".to_string()),
    };

    string_response(response, 200)
}
