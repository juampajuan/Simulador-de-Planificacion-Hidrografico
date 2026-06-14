use crate::db::engine::DBEngine;

pub fn init(
    db: &DBEngine
) -> Result<(), sqlite::Error> {

    db.connection.execute("
        CREATE TABLE IF NOT EXISTS professors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            is_admin INTEGER NOT NULL DEFAULT 0,

            price_boat REAL NOT NULL DEFAULT 0.0,
            price_launch REAL NOT NULL DEFAULT 0.0, 
            price_ship REAL NOT NULL DEFAULT 0.0,
            price_echosounder_monohaz REAL NOT NULL DEFAULT 0.0,
            price_echosounder_multihaz REAL NOT NULL DEFAULT 0.0,
            price_sensor_sound_profiler REAL NOT NULL DEFAULT 0.0,
            price_sensor_mareograph REAL NOT NULL DEFAULT 0.0,
            price_sensor_inertial REAL NOT NULL DEFAULT 0.0
        );

        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            filename TEXT NOT NULL,
            
            attempts_limit INTEGER NOT NULL,
            weather TEXT NOT NULL,
            seabed_hardness TEXT NOT NULL,
            budget REAL NOT NULL,
            geotiff_min_depth REAL NOT NULL,
            geotiff_max_depth REAL NOT NULL,

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

            professor_id INTEGER,
            student_id INTEGER,

            token TEXT UNIQUE NOT NULL,

            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            expires_at DATETIME,

            FOREIGN KEY (professor_id)
                REFERENCES professors(id)
                ON DELETE CASCADE,

            FOREIGN KEY (student_id)
                REFERENCES students(id)
                ON DELETE CASCADE,

            CHECK (
                (professor_id IS NOT NULL AND student_id IS NULL)
                OR
                (professor_id IS NULL AND student_id IS NOT NULL)
            )
        );

        INSERT INTO professors (username, password_hash, is_admin)
        VALUES ('admin', '$2b$12$df1235sa8sf8kffddnasnb9qpnpoiznaswq2', 1)
        ON CONFLICT (username) DO NOTHING;
        INSERT OR IGNORE INTO students (name, code, professor_id, project_id)
        VALUES ('Grupo Alumnos 1', 'ABC1J5', 1, 1);

        -- TODO: Los indices.
    ")?;

    Ok(())
}