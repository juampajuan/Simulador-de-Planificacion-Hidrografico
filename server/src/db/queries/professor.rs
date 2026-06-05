use sqlite::State;
use crate::db::engine::DBEngine;

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