use crate::db::engine::DBEngine;
use crate::db::queries::student;
use common::Student;
use std::sync::{Arc, Mutex};

/// Toma el lock de la DB y valida el código de acceso de un alumno (login).
pub fn verify_code_locked(
    db: &Arc<Mutex<DBEngine>>,
    code: &str,
) -> Result<Option<(i64, String)>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student::verify_code(&db_connection, code)
}

/// Toma el lock de la DB y actualiza nombre y proyecto de un alumno (validando dueño).
pub fn update_student_locked(
    db: &Arc<Mutex<DBEngine>>,
    id: i64,
    name: &str,
    project_id: i64,
    professor_id: i64,
) -> Result<bool, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student::update_student(&db_connection, id, name, project_id, professor_id)
}

/// Toma el lock de la DB y borra un alumno (validando que sea del profesor dado).
pub fn delete_student_locked(
    db: &Arc<Mutex<DBEngine>>,
    id: i64,
    professor_id: i64,
) -> Result<bool, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student::delete_student(&db_connection, id, professor_id)
}

/// Toma el lock de la DB y trae todos los alumnos de un profesor.
pub fn get_students_for_professor_locked(
    db: &Arc<Mutex<DBEngine>>,
    professor_id: i64,
) -> Result<Vec<Student>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student::get_students_for_professor(&db_connection, professor_id)
}

/// Toma el lock de la DB y crea un alumno nuevo.
pub fn create_student_locked(
    db: &Arc<Mutex<DBEngine>>,
    code: &str,
    name: &str,
    project_id: i64,
    professor_id: i64,
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

    student::create_student(&db_connection, code, name, project_id, professor_id)
}

/// Toma el lock de la DB y trae un alumno por su id, si existe.
pub fn get_student_by_id_locked(
    db: &Arc<Mutex<DBEngine>>,
    student_id: i64,
) -> Result<Option<Student>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student::get_student_by_id(&db_connection, student_id)
}

/// Toma el lock de la DB y suma uno al contador de intentos de un alumno.
pub fn increment_student_attempts_locked(
    db: &Arc<Mutex<DBEngine>>,
    student_id: i64,
) -> Result<bool, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    student::increment_attempts(&db_connection, student_id)
}
