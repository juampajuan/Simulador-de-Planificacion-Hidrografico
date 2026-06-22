use std::sync::{Arc, Mutex};

use crate::{db::{engine::DBEngine, queries::auth::TokenOwner, queries_interface::auth}, utils::helpers::get_cookie};

// Helpers de autenticación compartidos entre endpoints: validan la cookie de sesión
// y resuelven el id del usuario según su rol.

/// Valida la sesión de un profesor a partir de la cookie `auth_token`.
/// Devuelve `Ok(Some(id))` si el token es válido y de un profesor, `Ok(None)` si no hay
/// cookie o el token no corresponde a un profesor, y `Err` ante un fallo de base de datos.
pub fn check_profesor_auth(request: &tiny_http::Request, db: &Arc<Mutex<DBEngine>>) -> Result<Option<i64>,String> {

    let Some(token) = get_cookie(request, "auth_token") else {
        return Ok(None);
    };

    match auth::get_user_by_token_locked(db, &token) {
        Ok(Some(TokenOwner::Professor(id))) => Ok(Some(id)),
        Ok(_) => Ok(None),
        Err(e) => Err(e.to_string())
    }
}

/// Igual que `check_profesor_auth` pero para alumnos: devuelve el id solo si el token
/// pertenece a un alumno.
pub fn check_student_auth(request: &tiny_http::Request, db: &Arc<Mutex<DBEngine>>) -> Result<Option<i64>,String> {

    let Some(token) = get_cookie(request, "auth_token") else {
        return Ok(None);
    };

    match auth::get_user_by_token_locked(db, &token) {
        Ok(Some(TokenOwner::Student(id))) => Ok(Some(id)),
        Ok(_) => Ok(None),
        Err(e) => Err(e.to_string())
    }
}
