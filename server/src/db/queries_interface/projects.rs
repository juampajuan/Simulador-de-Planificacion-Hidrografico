use crate::db::queries::proyects;
use crate::db::engine::DBEngine;
use std::sync::{Arc, Mutex};
use crate::db::queries::proyects::AdminProjectView;

/// Toma el lock de la DB y trae un proyecto por su id, si existe.
pub fn get_project_by_id_locked(
    db: &Arc<Mutex<DBEngine>>,
    id: i64,
) -> Result<Option<AdminProjectView>, sqlite::Error> { 
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    crate::db::queries::proyects::get_project_by_id(
        &db_connection,
        id,
    )
}

/// Toma el lock de la DB y trae todos los proyectos de un profesor.
pub fn get_all_by_professor_id_locked(
    db: &Arc<Mutex<DBEngine>>,
    professor_id: i64,
) -> Result<Vec<proyects::AdminProjectView>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    proyects::get_all_by_professor_id(
        &db_connection,
        professor_id,
    )
}

/// Toma el lock de la DB y borra un proyecto (validando que sea del profesor dado).
pub fn delete_project_by_id_locked(
    db: &Arc<Mutex<DBEngine>>,
    project_id: i64,
    professor_id: i64,
) -> Result<bool, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    proyects::delete_project_by_id(
        &db_connection,
        project_id,
        professor_id,
    )
}

/// Toma el lock de la DB y actualiza la metadata de un proyecto (validando dueño).
pub fn update_project_locked(
    db: &Arc<Mutex<DBEngine>>,
    id: i64,
    professor_id: i64,
    metadata: &proyects::ProjectMetadata,
) -> Result<bool, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    proyects::update_project(
        &db_connection,
        id,
        professor_id,
        metadata,
    )
}

/// Toma el lock de la DB y crea un proyecto nuevo, devolviendo su id.
pub fn create_project_locked(
    db: &Arc<Mutex<DBEngine>>,
    filename: &str,
    professor_id: i64,
    metadata: &proyects::ProjectMetadata,
) -> Result<usize, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    proyects::create_project(
        &db_connection,
        filename,
        professor_id,
        metadata,
    )
}

/// Toma el lock de la DB y devuelve el id del proyecto asignado a un alumno.
pub fn get_project_id_by_student_locked(
    db: &Arc<Mutex<DBEngine>>,
    student_id: i64,
) -> Result<Option<i64>, sqlite::Error> {
    let db_connection = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(sqlite::Error {
                code: None,
                message: Some("Cannot lock db".to_string()),
            })
        }
    };

    proyects::get_project_id_by_student(
        &db_connection,
        student_id,
    )
}

pub fn update_project_geotiff_bounds_locked(
    db: &Arc<Mutex<DBEngine>>,
    project_id: i64,
    min_depth: f64,
    max_depth: f64,
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

    crate::db::queries::proyects::update_project_geotiff_bounds(
        &db_connection,
        project_id,
        min_depth,
        max_depth,
    )
}