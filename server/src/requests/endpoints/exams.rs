use crate::db::engine::DBEngine;
use crate::db::queries_interface::student_simulations;
use crate::logging::logger::send_message_to_logger;
use crate::logging::structs::{LogType, ThreadMessage};
use crate::requests::endpoints::generic;
use crate::requests::http_helper::parse_json_body;
use crate::structs::request::HandlerResult;
use crate::utils::helpers_endpoints::{check_profesor_auth, check_student_auth};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use tiny_http::Request;

#[derive(serde::Deserialize)]
struct SelectSimulationPayload {
    simulation_id: Option<i64>,
}

/// Endpoint para que el alumno o docente obtenga el historial de intentos de simulación.
/// Si es alumno, devuelve las suyas. Si es docente, requiere ?student_id=<id> en la URL.
pub fn get_my_simulations(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        ("se esta intentando consultar el historial de simulaciones de un estudiante/grupo")
            .to_string(),
        LogType::Debug,
    );

    let mut target_student_id: Option<i64> = None;
    let mut queried_by_professor = false;

    // Intentamos validar si es un Alumno
    if let Ok(Some(student_id)) = check_student_auth(request, &db) {
        target_student_id = Some(student_id);
    }
    // Si no es alumno, nos fijamos si es un Docente
    else if let Ok(Some(_professor_id)) = check_profesor_auth(request, &db) {
        queried_by_professor = true;
        // Al ser docente, buscamos el id del alumno que quiere corregir desde la URL
        // Ejemplo de URL: /api/v1/exams/my_simulations?student_id=14
        if let Some(pos) = request.url().find("student_id=") {
            let id_str = &request.url()[pos + 11..];
            if let Ok(parsed_id) = id_str.parse::<i64>() {
                target_student_id = Some(parsed_id);
            }
        }

        if target_student_id.is_none() {
            return generic::string_response(
                "Bad Request: Se requiere el parámetro 'student_id' para el docente.".to_string(),
                400,
            );
        }
    }

    // Si no es ninguno de los dos, rebota con 401
    let student_id = match target_student_id {
        Some(id) => id,
        None => return generic::string_response("Sin autorizar".to_string(), 401),
    };

    if queried_by_professor {
        send_message_to_logger(
            tx,
            format!(
                "Un docente consultó el historial de simulaciones del estudiante/grupo {}.",
                student_id
            ),
            LogType::Debug,
        );
    }

    // Ejecutamos la query con el ID definitivo
    match student_simulations::get_student_simulations_locked(&db, student_id) {
        Ok(sims) => {
            let json_payload = match serde_json::to_string(&sims) {
                Ok(json) => json,
                Err(_) => {
                    return generic::server_error(
                        "Error al serializar las simulaciones".to_string(),
                    );
                }
            };
            generic::string_response(json_payload, 200)
        }
        Err(e) => generic::server_error(format!("Error en la base de datos: {}", e)),
    }
}

/// Endpoint para que el alumno marque una simulación específica como su entrega final (o la remueva).
pub fn select_exam_simulation(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    // Validamos autenticación del alumno
    let student_id = match check_student_auth(request, &db) {
        Ok(Some(id)) => id,
        Ok(None) => return generic::string_response("Sin autorizar".to_string(), 401),
        Err(err) => return generic::server_error(err),
    };

    // Parseamos el JSON body
    let payload: SelectSimulationPayload = match parse_json_body(request) {
        Ok(data) => data,
        Err(err) => return generic::string_response(format!("Bad Request: {}", err), 400),
    };

    // Pasamos los 3 argumentos requeridos: db, student_id, y el Option<i64>
    match student_simulations::select_student_simulation_locked(
        &db,
        student_id,
        payload.simulation_id,
    ) {
        Ok(_) => {
            send_message_to_logger(
                tx,
                format!(
                    "estudiante/grupo {} marcó la simulación {:?} como su entrega.",
                    student_id, payload.simulation_id
                ),
                LogType::Info,
            );
            generic::string_response("Estado de entrega actualizado con éxito".to_string(), 200)
        }
        Err(e) => generic::server_error(format!("Error al actualizar la base de datos: {}", e)),
    }
}
