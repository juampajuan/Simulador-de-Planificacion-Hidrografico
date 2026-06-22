use crate::db::engine::DBEngine;

/// Crea el esquema de la base si no existe: las tablas professors, projects, students y
/// auth_tokens (con sus foreign keys en cascada), inserta el usuario 'admin' por defecto
/// y crea los índices de las columnas más consultadas. Es idempotente: se puede correr en
/// cada arranque sin romper datos existentes.
pub fn init(
    db: &DBEngine
) -> Result<(), sqlite::Error> {

    db.connection.execute("
        CREATE TABLE IF NOT EXISTS professors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,

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
            attempts INTEGER NOT NULL DEFAULT 0,
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

        INSERT INTO professors (id, username, password_hash)
        VALUES (1, 'admin', '$2b$12$df1235sa8sf8kffddnasnb9qpnpoiznaswq2')
        ON CONFLICT (id) DO NOTHING;

        CREATE INDEX IF NOT EXISTS idx_projects_professor_id ON projects(professor_id);
        CREATE INDEX IF NOT EXISTS idx_students_project_id ON students(project_id);
        CREATE INDEX IF NOT EXISTS idx_students_professor_id ON students(professor_id);
        CREATE INDEX IF NOT EXISTS idx_students_code ON students(code);
        CREATE INDEX IF NOT EXISTS idx_professors_username ON professors(username);
    ")?;

    Ok(())
}