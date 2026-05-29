use crate::db::engine::DBEngine;

pub fn init(
    db: &DBEngine
) -> Result<(), sqlite::Error> {

    db.connection.execute(
        "
        CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            filename TEXT NOT NULL,
            code CHAR(4) NOT NULL
        );
        "
    )?;

    Ok(())
}