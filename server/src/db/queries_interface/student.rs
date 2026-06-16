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