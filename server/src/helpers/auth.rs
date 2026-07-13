use crate::db::{
    engine::DBEngine, queries::auth::TokenOwner, queries_interface::auth,
    queries_interface::professor,
};
use crate::utils::helpers::get_cookie;
use rand::Rng;
use std::sync::{Arc, Mutex};
use tiny_http::{Header, Request};

/// Valida la sesión de un profesor a partir de la cookie `auth_token`.
/// Devuelve `Ok(Some(id))` si el token es válido y de un profesor, `Ok(None)` si no hay
/// cookie o el token no corresponde a un profesor, y `Err` ante un fallo de base de datos.
pub fn check_profesor_auth(
    request: &tiny_http::Request,
    db: &Arc<Mutex<DBEngine>>,
) -> Result<Option<i64>, String> {
    let Some(token) = get_cookie(request, "auth_token") else {
        return Ok(None);
    };

    match auth::get_user_by_token_locked(db, &token) {
        Ok(Some(TokenOwner::Professor(id))) => Ok(Some(id)),
        Ok(_) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Igual que `check_profesor_auth` pero para alumnos.
/// Devuelve el id SOLO si el token pertenece a un alumno.
pub fn check_student_auth(
    request: &tiny_http::Request,
    db: &Arc<Mutex<DBEngine>>,
) -> Result<Option<i64>, String> {
    let Some(token) = get_cookie(request, "auth_token") else {
        return Ok(None);
    };

    match auth::get_user_by_token_locked(db, &token) {
        Ok(Some(TokenOwner::Student(id))) => Ok(Some(id)),
        Ok(_) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Valida si la request fue realizada por alguien con permisos de administrador.
/// Autenticandolo en consecuencia.
pub fn is_admin_request(request: &Request, db: &Arc<Mutex<DBEngine>>) -> Result<bool, String> {
    if is_local_request(request) {
        return Ok(true);
    }

    let professor_id = match check_profesor_auth(request, db) {
        Ok(id) => id,
        Err(_err) => return Err("No se pudo obtener credenciales para validar.".to_string()),
    };

    let admin_id = match professor::get_professor_id_by_username_locked(db, "admin") {
        Ok(id) => id,
        Err(_err) => return Err("Error obteniendo informacion del admin.".to_string()),
    };

    Ok(professor_id == admin_id)
}

/// Determina si una request fue formada en el sistema local.
pub fn is_local_request(request: &Request) -> bool {
    match request.remote_addr() {
        Some(addr) => addr.ip().is_loopback(),
        None => false,
    }
}

/// Genera la cookie usada para la sesion
/// mediante el token previamente generado.
pub fn create_auth_cookie(token: &str) -> Result<Header, ()> {
    let cookie = format!(
        "auth_token={}; Path=/; Max-Age=604800; HttpOnly; SameSite=Lax",
        token
    );

    Header::from_bytes("Set-Cookie", cookie).map_err(|_| ())
}

/// Genera un token random.
pub fn generate_token() -> String {
    let bytes: [u8; 32] = rand::rng().random();
    hex::encode(bytes)
}
