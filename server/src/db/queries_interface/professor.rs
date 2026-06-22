use crate::db::queries::professor;
use crate::db::engine::DBEngine;
use std::sync::{Arc, Mutex};

/// Toma el lock de la DB y verifica usuario y contraseña de un profesor.
pub fn verify_professor_credentials_locked(
    db: &Arc<Mutex<DBEngine>>,
    username: &str,
    password: &str,
) -> Result<Option<i64>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    professor::verify_professor_credentials(
        &db_connection,
        username,
        password,
    )
}

/// Toma el lock de la DB y cambia la contraseña de un profesor buscándolo por username.
pub fn change_password_by_username_locked(
    db: &Arc<Mutex<DBEngine>>,
    username: &str,
    new_password_hash: &str,
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

    professor::change_password_by_username(
        &db_connection,
        username,
        new_password_hash,
    )
}

/// Toma el lock de la DB y crea un profesor nuevo, devolviendo su id.
pub fn create_professor_locked(
    db: &Arc<Mutex<DBEngine>>,
    username: &str,
    password_hash: &str,
) -> Result<usize, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    professor::create_professor(
        &db_connection,
        username,
        password_hash,
    )
}

/// Toma el lock de la DB y busca el id de un profesor por su username.
pub fn get_professor_id_by_username_locked(
    db: &Arc<Mutex<DBEngine>>,
    username: &str,
) -> Result<Option<i64>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            });
        }
    };

    professor::get_professor_id_by_username(
        &db_connection,
        username,
    )
}