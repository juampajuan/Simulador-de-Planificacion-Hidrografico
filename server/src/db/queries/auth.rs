use crate::db::engine::DBEngine;

pub enum TokenOwner {
    Professor(i64),
    Student(i64),
}

/// Inserta un nuevo token de sesión en la tabla `auth_tokens`.
/// Según el `owner`, completa la columna `professor_id` o `student_id` (la otra queda NULL).
/// La expiración se calcula en la propia base como `now + expires_in_days` días.
pub fn create_token(
    db: &DBEngine,
    owner: TokenOwner,
    token: &str,
    expires_in_days: i64,
) -> Result<(), sqlite::Error> {
    let query = format!(
        "
        INSERT INTO auth_tokens (
            professor_id,
            student_id,
            token,
            expires_at
        )
        VALUES (
            ?,
            ?,
            ?,
            datetime('now', '+{} days')
        )
        ",
        expires_in_days
    );

    let mut statement = db.run_query(&query)?;

    match owner {
        TokenOwner::Professor(id) => {
            statement.bind((1, id))?;
            statement.bind((2, sqlite::Value::Null))?;
        }
        TokenOwner::Student(id) => {
            statement.bind((1, sqlite::Value::Null))?;
            statement.bind((2, id))?;
        }
    }

    statement.bind((3, token))?;
    statement.next()?;

    Ok(())
}

/// Busca el dueño de un token que todavía esté vigente (no expirado).
/// Devuelve `Some(TokenOwner)` si encuentra una fila válida con exactamente uno de los
/// dos ids cargado, o `None` si el token no existe, ya expiró o la fila es inconsistente.
pub fn get_user_by_token(
    db: &DBEngine,
    token: &str,
) -> Result<Option<TokenOwner>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT professor_id, student_id
        FROM auth_tokens
        WHERE token = ?
          AND expires_at > CURRENT_TIMESTAMP
        LIMIT 1
        "
    )?;

    statement.bind((1, token))?;

    match statement.next()? {
        sqlite::State::Row => {
            let professor_id: Option<i64> =
                statement.read::<Option<i64>, _>("professor_id")?;

            let student_id: Option<i64> =
                statement.read::<Option<i64>, _>("student_id")?;

            match (professor_id, student_id) {
                (Some(id), None) => Ok(Some(TokenOwner::Professor(id))),
                (None, Some(id)) => Ok(Some(TokenOwner::Student(id))),
                _ => Ok(None), 
            }
        }

        sqlite::State::Done => Ok(None),
    }
}

/// Borra todos los tokens de la tabla, cerrando de golpe todas las sesiones activas
/// (lo usa el comando `closeall` del CLI).
pub fn delete_all_tokens(
    db: &DBEngine,
) -> Result<(), sqlite::Error> {

    let mut statement = db.run_query(
        "
        DELETE FROM auth_tokens
        "
    )?;

    statement.next()?;

    Ok(())
}

pub fn delete_token(
    db: &DBEngine,
    token: &str,
) -> Result<(), sqlite::Error> {

    let mut statement = db.run_query(
        "
        DELETE FROM auth_tokens
        WHERE token = ?
        "
    )?;

    statement.bind((1, token))?;
    statement.next()?;

    Ok(())
}