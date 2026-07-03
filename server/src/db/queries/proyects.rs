use crate::db::engine::DBEngine;
use serde::{Serialize, Deserialize};
 
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: Option<String>,
    pub attempts_limit: i64,
    pub exam_mode: bool,
    pub weather: String,
    pub seabed_hardness: String,
    pub budget: f64,
    pub geotiff_min_depth: f64,
    pub geotiff_max_depth: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminProjectView {
    pub id: usize,
    pub filename: String,
    pub professor_id: i64,
    #[serde(flatten)] 
    pub metadata: ProjectMetadata,
}

/// Inserta un proyecto nuevo con su archivo geotiff, su dueño (professor_id) y toda la metadata.
/// Devuelve el id generado por la base.
pub fn create_project(
    db: &DBEngine,
    filename: &str,
    professor_id: i64,
    metadata: &ProjectMetadata,
) -> Result<usize, sqlite::Error> {
    let mut statement = db.run_query(
        "
        INSERT INTO projects(
            name,
            description,
            filename,
            attempts_limit,
            exam_mode,
            weather,
            seabed_hardness,
            budget,
            geotiff_min_depth,
            geotiff_max_depth,
            professor_id
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        "
    )?;

    statement.bind((1, metadata.name.as_str()))?;
    match &metadata.description {
        Some(desc) => statement.bind((2, desc.as_str()))?,
        None => statement.bind((2, ""))?, 
    };
    statement.bind((3, filename))?;
    statement.bind((4, metadata.attempts_limit))?;
    statement.bind((5, if metadata.exam_mode { 1 } else { 0 }))?;
    statement.bind((6, metadata.weather.as_str()))?;
    statement.bind((7, metadata.seabed_hardness.as_str()))?;
    statement.bind((8, metadata.budget))?;
    statement.bind((9, metadata.geotiff_min_depth))?;
    statement.bind((10, metadata.geotiff_max_depth))?;
    statement.bind((11, professor_id))?;

    if let sqlite::State::Row = statement.next()? {
        Ok(statement.read::<i64, _>("id")? as usize)
    } else {
        unreachable!()
    }
}

/// Trae todos los proyectos que pertenecen a un profesor, armando un `AdminProjectView` por cada fila.
pub fn get_all_by_professor_id(
    db: &DBEngine,
    professor_id: i64,
) -> Result<Vec<AdminProjectView>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT id, name, description, filename, professor_id,
               attempts_limit, exam_mode, weather, seabed_hardness, budget, geotiff_min_depth, geotiff_max_depth
        FROM projects
        WHERE professor_id = ?
        "
    )?;

    statement.bind((1, professor_id))?;

    let mut projects = Vec::new();

    while let sqlite::State::Row = statement.next()? {
        projects.push(AdminProjectView {
            id: statement.read::<i64, _>("id")? as usize,
            filename: statement.read::<String, _>("filename")?, 
            professor_id: statement.read::<i64, _>("professor_id")?, 
            metadata: ProjectMetadata {
                name: statement.read::<String, _>("name")?,
                description: statement.read::<Option<String>, _>("description")?,
                exam_mode: statement.read::<i64, _>("exam_mode")? != 0,
                attempts_limit: statement.read::<i64, _>("attempts_limit")?,
                weather: statement.read::<String, _>("weather")?,
                seabed_hardness: statement.read::<String, _>("seabed_hardness")?,
                budget: statement.read::<f64, _>("budget")?,
                geotiff_min_depth: statement.read::<f64, _>("geotiff_min_depth")?,
                geotiff_max_depth: statement.read::<f64, _>("geotiff_max_depth")?,
            }
        });
    }

    Ok(projects)
}

/// Busca un proyecto puntual por su id.
/// Devuelve `Some(AdminProjectView)` si existe o `None` si no.
pub fn get_project_by_id(
    db: &DBEngine,
    id: i64,
) -> Result<Option<AdminProjectView>, sqlite::Error> {
    let mut statement = db.run_query(
        "
        SELECT id, name, description, filename, professor_id,
               attempts_limit, exam_mode, weather, seabed_hardness, budget, geotiff_min_depth, geotiff_max_depth
        FROM projects
        WHERE id = ?
        "
    )?;

    statement.bind((1, id))?;

    if let sqlite::State::Row = statement.next()? {
        Ok(Some(AdminProjectView {
            id: statement.read::<i64, _>("id")? as usize,
            filename: statement.read::<String, _>("filename")?,
            professor_id: statement.read::<i64, _>("professor_id")?,
            metadata: ProjectMetadata {
                name: statement.read::<String, _>("name")?,
                description: statement.read::<Option<String>, _>("description")?,
                exam_mode: statement.read::<i64, _>("exam_mode")? != 0,
                attempts_limit: statement.read::<i64, _>("attempts_limit")?,
                weather: statement.read::<String, _>("weather")?,
                seabed_hardness: statement.read::<String, _>("seabed_hardness")?,
                budget: statement.read::<f64, _>("budget")?,
                geotiff_min_depth: statement.read::<f64, _>("geotiff_min_depth")?,
                geotiff_max_depth: statement.read::<f64, _>("geotiff_max_depth")?,
            }
        }))
    } else {
        Ok(None)
    }
}

/// Devuelve el id del proyecto asignado a un alumno (consultando la tabla students).
/// `None` si el alumno no existe o no tiene proyecto asignado.
pub fn get_project_id_by_student(
    db: &DBEngine,
    student_id: i64,
) -> Result<Option<i64>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT project_id
        FROM students
        WHERE id = ?
        "
    )?;

    statement.bind((1, student_id))?;

    if let sqlite::State::Row = statement.next()? {
        Ok(Some(statement.read::<i64, _>("project_id")?))
    } else {
        Ok(None)
    }
}

/// Borra un proyecto validando que pertenezca al profesor que lo pide.
/// Devuelve `true` si borró alguna fila (existía y era suyo) o `false` si no.
pub fn delete_project_by_id(
    db: &DBEngine,
    project_id: i64,
    professor_id: i64,
) -> Result<bool, sqlite::Error> {

    let mut statement = db.run_query(
        "
        DELETE FROM projects
        WHERE id = ?
          AND professor_id = ?
        "
    )?;

    statement.bind((1, project_id))?;
    statement.bind((2, professor_id))?;

    statement.next()?;

    Ok(db.connection.change_count() > 0)
}

/// Actualiza la metadata de un proyecto, validando que sea del profesor que lo pide.
/// Devuelve `true` si modificó alguna fila o `false` si no existía o no era suyo.
pub fn update_project(
    db: &DBEngine,
    id: i64,
    professor_id: i64,
    metadata: &ProjectMetadata,
) -> Result<bool, sqlite::Error> {

    let mut statement = db.run_query(
        "
        UPDATE projects
        SET name = ?,
            description = ?,
            attempts_limit = ?,
            exam_mode = ?,
            weather = ?,
            seabed_hardness = ?,
            budget = ?,
            geotiff_min_depth = ?,
            geotiff_max_depth = ?
        WHERE id = ?
          AND professor_id = ?
        "
    )?;

    statement.bind((1, metadata.name.as_str()))?;
    match &metadata.description {
        Some(desc) => statement.bind((2, desc.as_str()))?,
        None => statement.bind((2, ""))?, 
    };
    statement.bind((3, metadata.attempts_limit))?;
    statement.bind((4, if metadata.exam_mode { 1 } else { 0 }))?;
    statement.bind((5, metadata.weather.as_str()))?;
    statement.bind((6, metadata.seabed_hardness.as_str()))?;
    statement.bind((7, metadata.budget))?;
    statement.bind((8, metadata.geotiff_min_depth))?;
    statement.bind((9, metadata.geotiff_max_depth))?;
    statement.bind((10, id))?;
    statement.bind((11, professor_id))?;

    statement.next()?;

    Ok(db.connection.change_count() > 0)
}