use sqlite::State;
use crate::db::engine::DBEngine;
use serde::{Serialize, Deserialize};

// Esto se podria mover a los structs. Y va a haber para cada tipo
#[derive(Serialize)]
pub struct Student {
    pub code: String,
    pub name: String,
    pub id: i64,
    pub project_id: i64
}

#[derive(Deserialize)]
pub struct NewStudent {
    pub name: String,
    pub project_id: i64,
}

#[derive(serde::Deserialize)]
pub struct UpdateStudent {
    pub id: i64,
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
    id: i64
) -> Result<Option<()>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        DELETE FROM students
        WHERE id = ?;
        "
    )?;

    statement.bind((1, id))?;
    statement.next()?;

    Ok(None)
}
    
pub fn verify_code(
    db: &DBEngine,
    code: &str,
) -> Result<Option<i64>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT id
        FROM students
        WHERE code = ?
        "
    )?;

    statement.bind((1, code))?;

    if let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>("id")?;
        return Ok(Some(id));
    }

    Ok(None)
}

pub fn get_students_for_professor(
    db: &DBEngine,
    professor_id: i64,
) -> Result<Vec<Student>, sqlite::Error> {
    let query = "
        SELECT id, name, code, project_id
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
) -> Result<(), sqlite::Error> {

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

    Ok(()) 
    //Deberia devolver alguna confirmacion de que pudo modificarlo
}

// Demo de uso
// use db::queries::student::{Student, create_student, get_student_by_code};

//     create_student(
//         &db,
//         "A123",
//         "Julen"
//     )?;

//     let student =
//         get_student_by_code(
//             &db,
//             "A123"
//         )?;

//     match student {

//         Some(student) => {
//             println!("{}", student.name);
//         }

//         None => {
//             println!("No encontrado");
//         }
//     }