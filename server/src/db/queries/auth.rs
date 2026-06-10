use sqlite::State;
use crate::db::engine::DBEngine;
use crate::db::encrypt::{hash_password, verify_password};

pub fn create_token(
    db: &DBEngine,
    professor_id: i64,
    token: &str,
    expires_in_days: i64,
) -> Result<(), sqlite::Error> {

    let query = format!(
        "
        INSERT INTO auth_tokens (
            professor_id,
            token,
            expires_at
        )
        VALUES (
            ?,
            ?,
            datetime('now', '+{} days')
        )
        ",
        expires_in_days
    );

    let mut statement = db.run_query(&query)?;
    statement.bind((1, professor_id))?;
    statement.bind((2, token))?;
    statement.next()?;

    Ok(())
}

pub fn get_professor_id_by_token(
    db: &DBEngine,
    token: &str,
) -> Result<Option<i64>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT professor_id
        FROM auth_tokens
        WHERE token = ?
          AND expires_at > CURRENT_TIMESTAMP
        LIMIT 1
        "
    )?;

    statement.bind((1, token))?;

    match statement.next()? {
        sqlite::State::Row => {
            let professor_id = statement.read::<i64, _>("professor_id")?;
            Ok(Some(professor_id))
        }
        sqlite::State::Done => Ok(None),
    }
}

pub fn delete_all_tokens(
    db: &DBEngine,
) -> Result<(), sqlite::Error> {

    let mut statement = db.run_query(
        "
        DELETE FROM auth_tokens
        "
    )?;

    statement.next()?;

    Ok(())
}