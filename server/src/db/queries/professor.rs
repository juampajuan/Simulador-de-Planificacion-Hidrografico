use sqlite::State;
use crate::db::engine::DBEngine;
use crate::db::encrypt::{hash_password, verify_password};

pub fn create_professor(
    db: &DBEngine,
    username: &str,
    password_hash: &str,
) -> Result<usize, sqlite::Error> {

    let mut statement = db.run_query(
        "
        INSERT INTO professors(username, password_hash)
        VALUES(?, ?)
        RETURNING id
        "
    )?;

    statement.bind((1, username))?;
    statement.bind((2, password_hash))?;

    if let sqlite::State::Row = statement.next()? {
        let id = statement.read::<i64, _>("id")? as usize;
        Ok(id)
    } else {
        unreachable!()
    }
}

pub fn change_password(
    db: &DBEngine,
    professor_id: i64,
    new_password_hash: &str,
) -> Result<(), sqlite::Error> {

    let mut statement = db.run_query(
        "
        UPDATE professors
        SET password_hash = ?
        WHERE id = ?
        "
    )?;

    statement.bind((1, new_password_hash))?;
    statement.bind((2, professor_id))?;
    statement.next()?;

    Ok(())
}

pub fn change_password_by_username(
    db: &DBEngine,
    username: &str,
    new_password_hash: &str,
) -> Result<(), sqlite::Error> {
    let professor_id = get_professor_id_by_username(db, username)?
        .ok_or_else(|| sqlite::Error {
            code: None,
            message: Some("Professor not found".to_string()),
        })?;

    change_password(db, professor_id, new_password_hash)
}

pub fn get_professor_id_by_username(
    db: &DBEngine,
    username: &str,
) -> Result<Option<i64>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT id
        FROM professors
        WHERE username = ?
        "
    )?;

    statement.bind((1, username))?;

    if let sqlite::State::Row = statement.next()? {
        Ok(Some(
            statement.read::<i64, _>("id")?
        ))
    } else {
        Ok(None)
    }
}

pub fn verify_professor_credentials(
    db: &DBEngine,
    username: &str,
    password: &str,
) -> Result<Option<i64>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT id, password_hash
        FROM professors
        WHERE username = ?
        "
    )?;

    statement.bind((1, username))?;

    if let sqlite::State::Row = statement.next()? {
        let id = statement.read::<i64, _>("id")?;
        let password_hash = statement.read::<String, _>("password_hash")?;

        if verify_password(password, &password_hash) {
            Ok(Some(id))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn sync_admin_password(
    db: &DBEngine,
    admin_password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut statement = db.run_query(
        "
        SELECT password_hash
        FROM professors
        WHERE username = 'admin'
        "
    )?;

    let current_hash = if let sqlite::State::Row = statement.next()? {
        Some(statement.read::<String, _>("password_hash")?)
    } else {
        None
    };

    let needs_update = match current_hash {
        Some(hash) => !verify_password(admin_password, &hash),
        None => true,
    };

    if needs_update {
        let new_hash = hash_password(admin_password)?;
        change_password_by_username(db, "admin", &new_hash)?;
    }

    Ok(())
}