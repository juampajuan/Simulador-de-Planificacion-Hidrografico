use sqlite::State;

use crate::db::engine::DBEngine;

// Esto se podria mover a los structs. Y va a haber para cada tipo
pub struct Student {
    pub code: String,
    pub name: String,
}

pub fn create_student(
    db: &DBEngine,
    code: &str,
    name: &str
) -> Result<(), sqlite::Error> {

    let mut statement = db.run_query(
        "
        INSERT INTO students(code, name)
        VALUES(?, ?)
        "
    )?;

    statement.bind((1, code))?;
    statement.bind((2, name))?;

    statement.next()?;

    Ok(())
}

pub fn get_student_by_code(
    db: &DBEngine,
    code: &str
) -> Result<Option<Student>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT code, name
        FROM students
        WHERE code = ?
        "
    )?;

    statement.bind((1, code))?;

    if let Ok(State::Row) = statement.next() {

        let student = Student {
            code: statement.read::<String, _>("code")?,
            name: statement.read::<String, _>("name")?,
        };

        return Ok(Some(student));
    }

    Ok(None)
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