use std::thread;
use tiny_http::Request;
use crate::requests::handler::handle_request;

pub fn create_request_thread(request: Request) {
    thread::spawn(move || {
        let log = handle_request(request);
        log.print();
    });
}