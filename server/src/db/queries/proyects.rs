use sqlite::State;
use crate::db::engine::DBEngine;
use serde::Serialize;

#[derive(Serialize)]
pub struct Project {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub filename: String, 
}

pub fn create_project(
    db: &DBEngine,
    name: &str,
    description: Option<&str>,
    filename: &str,
    professor_id: i64,
) -> Result<usize, sqlite::Error> {

    let mut statement = db.run_query(
        "
        INSERT INTO projects(name, description, filename, professor_id)
        VALUES(?, ?, ?, ?)
        RETURNING id
        "
    )?;

    statement.bind((1, name))?;
    statement.bind((2, description))?;
    statement.bind((3, filename))?;
    statement.bind((4, professor_id))?;

    if let sqlite::State::Row = statement.next()? {
        Ok(statement.read::<i64, _>("id")? as usize)
    } else {
        unreachable!()
    }
}

pub fn get_all_by_professor_id(
    db: &DBEngine,
    professor_id: i64,
) -> Result<Vec<Project>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT id, name, description, filename, professor_id
        FROM projects
        WHERE professor_id = ?
        "
    )?;

    statement.bind((1, professor_id))?;

    let mut projects = Vec::new();

    while let sqlite::State::Row = statement.next()? {
        projects.push(Project {
            id: statement.read::<i64, _>("id")? as usize,
            name: statement.read::<String, _>("name")?,
            description: statement.read::<Option<String>, _>("description")?,
            filename: statement.read::<String, _>("filename")?, 
        });
    }

    Ok(projects)
}


pub fn get_project_by_id(
    db: &DBEngine,
    id: i64,
) -> Result<Project, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT id, name, description, filename, professor_id
        FROM projects
        WHERE id = ?
        "
    )?;

    statement.bind((1, id))?;

    if let sqlite::State::Row = statement.next()? {
        Ok(Project {
            id: statement.read::<i64, _>("id")? as usize,
            name: statement.read::<String, _>("name")?,
            description: statement.read::<Option<String>, _>("description")?,
            filename: statement.read::<String, _>("filename")?, 
        })
    } else {
        unreachable!()
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