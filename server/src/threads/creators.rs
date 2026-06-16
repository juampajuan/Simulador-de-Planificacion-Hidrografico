use std::thread::{self, JoinHandle};
use tiny_http::Request;
use std::sync::{Arc, Mutex};
use crate::structs::filecache::{FileCache};
use crate::requests::handler::handle_request;
use crate::structs::settings::Settings;
use crate::db::engine::DBEngine;

pub fn create_request_thread(request: Request, cache: Arc<Mutex<FileCache>>, settings: Arc<Settings>, db: Arc<Mutex<DBEngine>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let log = handle_request(request, cache, db, settings);
        log.print();
    })
}

pub fn create_cli_thread(port: i32) -> JoinHandle<()> {
    thread::spawn(move || {
        cli::cli_server_menu(port);
    })
}