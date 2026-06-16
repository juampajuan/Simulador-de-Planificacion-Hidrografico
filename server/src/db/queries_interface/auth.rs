use crate::db::queries::auth;
use crate::db::engine::DBEngine;
use std::sync::{Arc, Mutex};

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