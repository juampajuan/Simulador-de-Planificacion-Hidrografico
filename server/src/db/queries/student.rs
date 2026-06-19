use sqlite::State;
use crate::db::engine::DBEngine;
use serde::{Serialize, Deserialize};

// Esto se podria mover a los structs. Y va a haber para cada tipo
#[derive(Serialize, Clone)] // Agregamos Clone por comodidad
pub struct Student {
    pub code: String,
    pub name: String,
    pub id: i64,
    pub project_id: i64,
    pub attempts: i64,
}

#[derive(Deserialize)]
pub struct NewStudent {
    pub name: String,
    pub project_id: i64,
}

pub fn create_student(
    db: &DBEngine,
    code: &str,
    name: &str,
    project_id: i64,
    professor_id: i64,
) -> Result<(), sqlite::Error> {

    let mut statement = db.run_query(
        "
        INSERT INTO students(code, name, project_id, professor_id)
        VALUES(?, ?, ?, ?)
        "
    )?;

    statement.bind((1, code))?;
    statement.bind((2, name))?;
    statement.bind((3, project_id))?;
    statement.bind((4, professor_id))?;
    statement.next()?;

    Ok(())
}

pub fn delete_student(
    db: &DBEngine,
    id: i64,
    professor_id: i64
) -> Result<bool, sqlite::Error> {

    let mut statement = db.run_query(
        "
        DELETE FROM students
        WHERE id = ? and professor_id = ?;
        "
    )?;

    statement.bind((1, id))?;
    statement.bind((2, professor_id))?;
    statement.next()?;

    Ok(db.connection.change_count() > 0)
}
    
pub fn verify_code(
    db: &DBEngine,
    code: &str,
) -> Result<Option<(i64, String)>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT id, name
        FROM students
        WHERE code = ?
        "
    )?;

    statement.bind((1, code))?;

    if let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>("id")?;
        let name = statement.read::<String, _>("name")?;
        return Ok(Some((id, name)));
    }

    Ok(None)
}

pub fn get_students_for_professor(
    db: &DBEngine,
    professor_id: i64,
) -> Result<Vec<Student>, sqlite::Error> {
    let query = "
        SELECT id, name, code, project_id, attempts
        FROM students
        WHERE professor_id = ?
    ";

    let mut statement = db.run_query(query)?;

    statement.bind((1, professor_id))?;

    let mut students = Vec::new();

    while let State::Row = statement.next()? {
        students.push(Student {
            id: statement.read::<i64, _>("id")?,
            name: statement.read::<String, _>("name")?,
            code: statement.read::<String, _>("code")?,
            project_id: statement.read::<i64, _>("project_id")?,
            attempts: statement.read::<i64, _>("attempts")?,
        });
    }

    Ok(students)
}

pub fn update_student(
    db: &DBEngine,
    id: i64,
    name: &str,
    project_id: i64,
    professor_id: i64,
) -> Result<bool, sqlite::Error> {

    let mut statement = db.run_query(
        "
        UPDATE students
        SET name = ?, project_id = ?
        WHERE id = ? AND professor_id = ?
        "
    )?;

    statement.bind((1, name))?;
    statement.bind((2, project_id))?;
    statement.bind((3, id))?;
    statement.bind((4, professor_id))?;
    statement.next()?;

    Ok(db.connection.change_count() > 0)
}

pub fn get_student_by_id(
    db: &DBEngine,
    student_id: i64,
) -> Result<Option<Student>, sqlite::Error> {
    let query = "SELECT id, name, code, project_id, attempts FROM students WHERE id = ?";
    let mut statement = db.run_query(query)?;
    statement.bind((1, student_id))?;

    if let State::Row = statement.next()? {
        let student = Student {
            id: statement.read::<i64, _>("id")?,
            name: statement.read::<String, _>("name")?,
            code: statement.read::<String, _>("code")?,
            project_id: statement.read::<i64, _>("project_id")?,
            attempts: statement.read::<i64, _>("attempts")?,
        };
        Ok(Some(student))
    } else {
        Ok(None)
    }
}

pub fn increment_attempts(
    db: &DBEngine,
    student_id: i64,
) -> Result<bool, sqlite::Error> {
    let query = "UPDATE students SET attempts = attempts + 1 WHERE id = ?";
    let mut statement = db.run_query(query)?;
    statement.bind((1, student_id))?;
    
    statement.next()?;
    Ok(db.connection.change_count() > 0)
}