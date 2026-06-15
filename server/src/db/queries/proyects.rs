use crate::db::engine::DBEngine;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Project {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub filename: String, 
    pub professor_id: i64
}
 
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectMetadata {
    name: String,
    description: String,
    attempts_limit: i64,
    weather: String,
    seabed_hardness: String,
    budget: f64,
    geotiff_min_depth: f64,
    geotiff_max_depth: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminProjectView {
    pub id: usize,
    pub filename: String,
    pub professor_id: i64,
    #[serde(flatten)] 
    pub metadata: ProjectMetadata,
}

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
            weather,
            seabed_hardness,
            budget,
            geotiff_min_depth,
            geotiff_max_depth,
            professor_id
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        "
    )?;

    statement.bind((1, metadata.name.as_str()))?;
    statement.bind((2, metadata.description.as_str()))?;
    statement.bind((3, filename))?;
    statement.bind((4, metadata.attempts_limit))?;
    statement.bind((5, metadata.weather.as_str()))?;
    statement.bind((6, metadata.seabed_hardness.as_str()))?;
    statement.bind((7, metadata.budget))?;
    statement.bind((8, metadata.geotiff_min_depth))?;
    statement.bind((9, metadata.geotiff_max_depth))?;
    statement.bind((10, professor_id))?;

    if let sqlite::State::Row = statement.next()? {
        Ok(statement.read::<i64, _>("id")? as usize)
    } else {
        unreachable!()
    }
}

pub fn get_all_by_professor_id(
    db: &DBEngine,
    professor_id: i64,
) -> Result<Vec<AdminProjectView>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT id, name, description, filename, professor_id,
               attempts_limit, weather, seabed_hardness, budget, geotiff_min_depth, geotiff_max_depth
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
                description: statement.read::<Option<String>, _>("description")?.unwrap_or_default(),
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


pub fn get_project_by_id(
    db: &DBEngine,
    id: i64,
) -> Result<Option<Project>, sqlite::Error> {
    let mut statement = db.run_query(
        "
        SELECT id, name, description, filename, professor_id
        FROM projects
        WHERE id = ?
        "
    )?;

    statement.bind((1, id))?;

    if let sqlite::State::Row = statement.next()? {
        Ok(Some(Project {
            id: statement.read::<i64, _>("id")? as usize,
            name: statement.read::<String, _>("name")?,
            description: statement.read::<Option<String>, _>("description")?,
            filename: statement.read::<String, _>("filename")?,
            professor_id: statement.read::<i64, _>("professor_id")?,
        }))
    } else {
        Ok(None)
    }
}

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

    if let Ok(sqlite::State::Row) = statement.next() {
        return Ok(Some(statement.read::<i64, _>("project_id")?));
    } else {
        Ok(None)
    }
}

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
    statement.bind((2, metadata.description.as_str()))?;
    statement.bind((3, metadata.attempts_limit))?;
    statement.bind((4, metadata.weather.as_str()))?;
    statement.bind((5, metadata.seabed_hardness.as_str()))?;
    statement.bind((6, metadata.budget))?;
    statement.bind((7, metadata.geotiff_min_depth))?;
    statement.bind((8, metadata.geotiff_max_depth))?;
    statement.bind((9, id))?;
    statement.bind((10, professor_id))?;

    statement.next()?;

    Ok(db.connection.change_count() > 0)
}