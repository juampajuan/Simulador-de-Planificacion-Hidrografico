use crate::db::queries::student;
use crate::db::engine::DBEngine;
use std::sync::{Arc, Mutex};

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
            })
        }
    };

    student::verify_code(&db_connection, code)
}

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
            })
        }
    };

    student::update_student(
        &db_connection,
        id,
        name,
        project_id,
        professor_id,
    )
}

pub fn delete_student_locked(
    db: &Arc<Mutex<DBEngine>>,
    id: i64,
    professor_id: i64
) -> Result<bool, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    student::delete_student(
        &db_connection,
        id,
        professor_id
    )
}

pub fn get_students_for_professor_locked(
    db: &Arc<Mutex<DBEngine>>,
    professor_id: i64,
) -> Result<Vec<student::Student>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    student::get_students_for_professor(
        &db_connection,
        professor_id,
    )
}

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
            })
        }
    };

    student::create_student(
        &db_connection,
        code,
        name,
        project_id,
        professor_id,
    )
}

pub fn get_student_by_id_locked(
    db: &Arc<Mutex<DBEngine>>,
    student_id: i64,
) -> Result<Option<student::Student>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => return Err(sqlite::Error { code: None, message: Some("Cannot lock db".to_string()) }),
    };

    student::get_student_by_id(&db_connection, student_id)
}

pub fn increment_student_attempts_locked(
    db: &Arc<Mutex<DBEngine>>,
    student_id: i64,
) -> Result<bool, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => return Err(sqlite::Error { code: None, message: Some("Cannot lock db".to_string()) }),
    };

    student::increment_attempts(&db_connection, student_id)
}