use std::thread;
use tiny_http::Request;
use std::sync::{Arc, Mutex};
use crate::structs::filecache::{FileCache};
use crate::requests::handler::handle_request;

pub fn create_request_thread(request: Request, cache: Arc<Mutex<FileCache>>) {
    thread::spawn(move || {
        let log = handle_request(request, cache);
        log.print();
    });
}