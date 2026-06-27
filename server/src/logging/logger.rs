use std::sync::{Arc};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::logging::structs::LogType;
use crate::structs::settings::Settings;
use std::sync::mpsc::{Receiver, Sender};
use super::structs::ThreadMessage;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;

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
            println!("TODO: Que hace aca, si no podemos hacer panic???");
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
                if msg.log_type.level() <= logging_type {
                    Ok(Some((msg.msg, msg.log_type)))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        };

        if let Ok(Some(msg)) = result {
            let timestamp = timestamp();
            if let Err(e) = writeln!(file, "[{}] [{}]: {}",timestamp, msg.1.toString(), msg.0) {
                println!("{:?}", e);
                // TODO: Nuevamente, que hacemos con este error??
            };
        };
 
    }
}

pub fn send_message_to_logger(
    tx: Sender<ThreadMessage>,
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
        println!("TODO: Asumo que simplemente lo imprimimos, no tiene sentido que falle en el backend")
    }

}


// TOOD: Manejar el unwrap
// Esto devuelva [day:19700 14:32:10]
// Ver si usamos un crate o algo, para que sea mejor y te de la fecha posta.
fn timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    let secs = now.as_secs();

    // convertir segundos a fecha "humana" básica
    let days = secs / 86400;
    let secs_of_day = secs % 86400;

    let hours = secs_of_day / 3600;
    let minutes = (secs_of_day % 3600) / 60;
    let seconds = secs_of_day % 60;

    format!("day:{} {:02}:{:02}:{:02}", days, hours, minutes, seconds)
}

// TODO: Queremos alguna devolucion?? yo diria que no.
// Loguea y falla de forma silenciosa.