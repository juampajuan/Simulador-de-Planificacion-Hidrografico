use super::structs::ThreadMessage;
use crate::logging::structs::LogType;
use crate::structs::settings::Settings;
use chrono::Local;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const YELLOW: &str = "\x1b[33m";
const PURPLE: &str = "\x1b[35m";

/// Abre el archivo para guardar los logs
/// y ejecuta el metodo para que loguee
pub fn logger_handler(rx: Receiver<ThreadMessage>, settings: Arc<Settings>) {
    let mut file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open(&settings.log_file_name)
    {
        Ok(file) => file,
        Err(_) => {
            eprintln!("\x1b[91m[LOGGER]: No se pudo abrir el archivo para almacenarlos.\x1b[0m");
            eprintln!("\x1b[91m[LOGGER]: El servidor operará con normalidad, pero no se guardaran Logs.\x1b[0m");
            return;
        }
    };

    logging_writter_loop(
        rx,
        &mut file,
        settings.logging_type,
        settings.simplified_terminal_logs,
    );
}

/// Ejecuta el loop que recibe los LOGS de distintos hilos y los escribe en archivo si corresponde
pub fn logging_writter_loop(
    rx: Receiver<ThreadMessage>,
    file: &mut File,
    logging_type: i32,
    simplified: bool,
) {
    loop {
        let result = match rx.recv() {
            Ok(msg) => {
                if msg.log_type.level() >= logging_type {
                    Ok(Some((msg.msg, msg.log_type)))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        };

        if let Ok(Some(msg)) = result {
            let now = Local::now();
            let timestamp = now.format("%H:%M:%S %d:%m").to_string();

            log_message_on_terminal(&msg.0, &msg.1, simplified, &timestamp);

            if let Err(e) = writeln!(file, "[{}] [{}]: {}", timestamp, msg.1.to_string(), msg.0) {
                eprintln!("\x1b[91m[LOGGER]: No se pudo escribir el archivo para almacenarlos.\x1b[0m");
            };
        };
    }
}


fn log_message_on_terminal(message: &str, log_type: &LogType, simplified: bool, timestamp: &str) {

    if !simplified {
        
        let color = match *log_type {
            LogType::Debug => PURPLE,
            LogType::Info => CYAN,
            LogType::Warn => YELLOW,
            LogType::Error => RED,
        };

        println!(
            "[{}] [{}{}{}]: {}",
            timestamp,
            color,
            log_type.to_string(),
            RESET,
            message
        );

    }
}

fn log_message_on_terminal_timestamp_wrap(message: &str, log_type: LogType){
    let now = Local::now();
    let timestamp = now.format("%H:%M:%S %d:%m").to_string();
    log_message_on_terminal(message, &log_type, false, &timestamp);
}

pub fn send_message_to_logger(tx: &Sender<ThreadMessage>, msg: String, log_type: LogType) {
    if let Err(err) = tx.send(ThreadMessage { msg, log_type }) {
        eprintln!("\x1b[91m[LOGGER]: No se pudo enviar el Log al hilo correspondiente. Se procede a mostrar todos por terminal, ya que no se almacenarán.\x1b[0m");
        log_message_on_terminal_timestamp_wrap(&err.0.msg, err.0.log_type);
    }
}

/// Arma el closure de debug logging atado a un `tx`
/// El '_ indica que el closure tiene una vida útil atada a la vida del `tx` que se le pasa como parámetro.
/// El move indica que el closure toma la propiedad de `tx` y lo mueve dentro del closure.
pub fn debug_logger<'a>(tx: &'a Sender<ThreadMessage>, prefix: &'a str) -> impl Fn(&str) + 'a {
    move |msg: &str| send_message_to_logger(tx, format!("{}: {}", prefix, msg), LogType::Debug)
}

/// Arma el closure de debug logging atado a un `tx` para Errores
pub fn error_logger<'a>(tx: &'a Sender<ThreadMessage>, prefix: &'a str) -> impl Fn(&str) + 'a {
    move |msg: &str| send_message_to_logger(tx, format!("{}: {}", prefix, msg), LogType::Error)
}

/// Arma el closure de debug logging atado a un `tx` para Info
pub fn info_logger<'a>(tx: &'a Sender<ThreadMessage>, prefix: &'a str) -> impl Fn(&str) + 'a {
    move |msg: &str| send_message_to_logger(tx, format!("{}: {}", prefix, msg), LogType::Info)
}
