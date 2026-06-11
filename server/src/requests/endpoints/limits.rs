use tiny_http::{Request};
use crate::structs::request::HandlerResult;
use crate::requests::endpoints::generic;
use super::generic::{not_found, server_error, normal_response};
use crate::structs::settings::Settings;
use std::sync::{Arc, Mutex};


pub fn get_limits(settings: Arc<Settings>) -> HandlerResult {
    let response = match serde_json::to_string(&*settings) {
        Ok(json) => json,
        Err(_) => return generic::server_error("Error serializing limits data".to_string()),
    };

    normal_response(response, 200)
}
