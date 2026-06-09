use crate::db::engine::DBEngine;

pub fn init(
    db: &DBEngine
) -> Result<(), sqlite::Error> {

    db.connection.execute("
        CREATE TABLE IF NOT EXISTS professors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            filename TEXT NOT NULL,
            -- Me tienen que informar sobre que variables van aca.

            professor_id INTEGER NOT NULL,
            FOREIGN KEY (professor_id)
                REFERENCES professors(id)
                ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            code TEXT UNIQUE NOT NULL,

            professor_id INTEGER NOT NULL,
            project_id INTEGER NOT NULL, 
            FOREIGN KEY (project_id)
                REFERENCES projects(id)
                ON DELETE CASCADE,

            FOREIGN KEY (professor_id)
                REFERENCES professors(id)
                ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS auth_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            professor_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,

            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            expires_at DATETIME,

            FOREIGN KEY (professor_id)
                REFERENCES professors(id)
                ON DELETE CASCADE
        ); 

        INSERT INTO professors (username, password_hash)
        VALUES ('admin', '$2b$12$df1235sa8sf8kffddnasnb9qpnpoiznaswq2')
        ON CONFLICT (username) DO NOTHING;
    ")?;

    Ok(())
}