use std::sync::{Arc};
use crate::logging::structs::LogType;
use crate::structs::settings::Settings;
use std::sync::mpsc::{Receiver, Sender};
use super::structs::ThreadMessage;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use chrono::Local;

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
            eprintln!("\x1b[91m[LOGGER]: No se pudo abrir el archivo.\x1b[0m");
            eprintln!("\x1b[93m[LOGGER]: El servidor funcionará con regularidad, pero no se almacenarán los LOG en el archivo.\x1b[0m");
            return;
        },
    };

    logging_writter_loop(rx, &mut file, settings.logging_type);
}


/// Ejecuta el loop que recibe los LOGS de distintos hilos y los escribe en archivo si corresponde
pub fn logging_writter_loop(rx: Receiver<ThreadMessage>, file: &mut File, logging_type: i32) {
    loop{

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
            let timestamp = now.format("%H:%M:%S %d:%M").to_string();
            if let Err(e) = writeln!(file, "[{}] [{}]: {}",timestamp, msg.1.to_string(), msg.0) {
                eprintln!("\x1b[91m[LOGGER]: No se pudo escribir en el archivo ({:?}).\x1b[0m", e);
                eprintln!("\x1b[93m[LOGGER]: El servidor funcionará con regularidad, pero no se almacenarán los LOG en el archivo.\x1b[0m");
            };
        };
 
    }
}

pub fn send_message_to_logger(
    tx: &Sender<ThreadMessage>,
    msg: String,
    log_type: LogType
) {

    if tx
        .send(ThreadMessage {
            msg: msg,
            log_type: log_type,
        })
        .is_err()
    {
        eprintln!("\x1b[91m[LOGGER]: No se pudo enviar el LOG al hilo correspondiente.\x1b[0m");
        eprintln!("\x1b[93m[LOGGER]: El servidor funcionará con regularidad, pero no se almacenarán los LOG en el archivo.\x1b[0m");
    }

}

/// Arma el closure de debug logging atado a un `tx`
/// El '_ indica que el closure tiene una vida útil atada a la vida del `tx` que se le pasa como parámetro.
/// El move indica que el closure toma la propiedad de `tx` y lo mueve dentro del closure.
pub fn debug_logger<'a>(tx: &'a Sender<ThreadMessage>, prefix: &'a str) -> impl Fn(&str) + 'a {
    move |msg: &str| send_message_to_logger(tx, format!("{}: {}", prefix, msg), LogType::Debug)
}