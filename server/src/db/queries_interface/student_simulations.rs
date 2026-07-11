use crate::db::engine::DBEngine;
use crate::db::queries::student_simulations;
use crate::db::queries::student_simulations::{StudentSimulation, StudentSimulationData};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn create_student_simulation_locked(
    db: &Arc<Mutex<DBEngine>>,
    data: &StudentSimulationData,
) -> Result<(), sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student_simulations::create_student_simulation(&db_connection, data)
}

pub fn select_student_simulation_locked(
    db: &Arc<Mutex<DBEngine>>,
    student_id: i64,
    simulation_id: Option<i64>,
) -> Result<(), sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    match simulation_id {
        Some(sim_id) => {
            // Usa la función atómica que limpia todo y tilda el nuevo
            student_simulations::select_student_simulation(&db_connection, sim_id)
        }
        None => {
            // Usa la función nueva que limpia todo para el alumno
            student_simulations::clear_student_simulations(&db_connection, student_id)
        }
    }
}

pub fn get_student_simulations_locked(
    db: &Arc<Mutex<DBEngine>>,
    student_id: i64,
) -> Result<Vec<StudentSimulation>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student_simulations::get_student_simulations(&db_connection, student_id)
}

/// Obtiene el número correlativo del próximo intento para un alumno específico.
/// TODO: Esto esta mal, aca va el wrapper. Osea aca haces el lock.
pub fn get_next_attempt_number(db: &DBEngine, student_id: i64) -> Result<i64, sqlite::Error> {
    let mut statement = db.run_query(
        "
        SELECT COUNT(*) FROM student_simulations WHERE student_id = ?
    ",
    )?;

    statement.bind((1, student_id))?;

    if let sqlite::State::Row = statement.next()? {
        let count: i64 = statement.read(0)?;
        Ok(count + 1)
    } else {
        Ok(1)
    }
}

pub fn get_all_simulation_images_locked(
    db: &Arc<Mutex<DBEngine>>,
) -> Result<HashSet<String>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student_simulations::get_all_simulation_images(&db_connection)
}
