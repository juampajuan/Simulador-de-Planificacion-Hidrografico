use crate::db::queries::auth;
use crate::db::engine::DBEngine;
use std::sync::{Arc, Mutex};

//Capa intermedia de autenticación: toma el lock del Mutex de la DB y delega
//en las queries crudas de `queries::auth`. Si el lock está envenenado, devuelve error.

/// Toma el lock de la DB y crea un token de sesión para el dueño dado.
pub fn create_token_locked(
    db: &Arc<Mutex<DBEngine>>,
    owner: auth::TokenOwner,
    token: &str,
    expires_in_days: i64,
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

    auth::create_token(
        &db_connection,
        owner,
        token,
        expires_in_days,
    )
}

/// Toma el lock de la DB y borra un token puntual (logout).
pub fn delete_token_locked(
    db: &Arc<Mutex<DBEngine>>,
    token: &str,
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

    auth::delete_token(&db_connection, token)
}

/// Toma el lock de la DB y borra todos los tokens (cierra todas las sesiones).
pub fn delete_all_tokens_locked(
    db: &Arc<Mutex<DBEngine>>,
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

    auth::delete_all_tokens(&db_connection)
}

/// Toma el lock de la DB y devuelve el dueño de un token vigente, si existe.
pub fn get_user_by_token_locked(
    db: &Arc<Mutex<DBEngine>>,
    token: &str,
) -> Result<Option<auth::TokenOwner>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    auth::get_user_by_token(
        &db_connection,
        token,
    )
}