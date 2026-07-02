use common::{EchosounderParameters, PathParameters, TransportParameters};

use crate::db::queries::student_simulations::StudentSimulation;
use crate::db::queries::{student_simulations};
use crate::db::engine::DBEngine;
use std::sync::{Arc, Mutex};

pub fn create_student_simulation_locked(
    db: &Arc<Mutex<DBEngine>>,
    student_id: i64,
    project_id: i64,
    result_min_depth: f64,
    result_max_depth: f64,
    path: &PathParameters,
    transport: &TransportParameters,
    echo: &EchosounderParameters,
) -> Result<(), sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    student_simulations::create_student_simulation(
        &db_connection,
        student_id,
        project_id,
        result_min_depth,
        result_max_depth,
        path,
        transport,
        echo,
    )
}

pub fn select_student_simulation_locked(
    db: &Arc<Mutex<DBEngine>>,
    simulation_id: i64,
) -> Result<(), sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    student_simulations::select_student_simulation(
        &db_connection,
        simulation_id,
    )
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
            })
        }
    };

    student_simulations::get_student_simulations(
        &db_connection,
        student_id,
    )
}