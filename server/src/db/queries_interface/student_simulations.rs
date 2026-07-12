use crate::db::engine::DBEngine;
use crate::db::queries::student_simulations;
use common::{StudentSimulation, StudentSimulationData};
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

pub fn get_next_attempt_number_locked(
    db: &Arc<Mutex<DBEngine>>,
    student_id: i64,
) -> Result<i64, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student_simulations::get_next_attempt_number(&db_connection, student_id)
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
