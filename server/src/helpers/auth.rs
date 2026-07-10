
use tiny_http::{Header, Request};
use std::sync::{Arc, Mutex};
use rand::Rng;
use crate::db::engine::DBEngine;
use crate::db::queries_interface::professor;
use crate::utils::helpers_endpoints::check_profesor_auth;

/// Valida si la request fue realizada por alguien con permisos de administrador.
/// Autenticandolo en consecuencia.
pub fn is_admin_request(
    request: &Request,
    db: &Arc<Mutex<DBEngine>>,
) -> Result<bool, String> {
    if is_local_request(request) {
        return Ok(true);
    }

    let professor_id = match check_profesor_auth(request, db) {
        Ok(id) => id,
        Err(_err) => return Err("No se pudo obtener credenciales para validar.".to_string())
    };

    let admin_id = match professor::get_professor_id_by_username_locked(
        db,
        "admin",
    ) {
        Ok(id) => id,
        Err(_err) => return Err("Error obteniendo informacion del admin.".to_string())
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
pub fn create_auth_cookie(
    token: &str,
) -> Result<Header, ()> {

    let cookie = format!(
        "auth_token={}; Path=/; Max-Age=604800; HttpOnly; SameSite=Lax",
        token
    );

    Header::from_bytes("Set-Cookie", cookie)
        .map_err(|_| ())
}

/// Genera un token random.
pub fn generate_token() -> String {
    let bytes: [u8; 32] = rand :: rng().random();
    hex::encode(bytes)
}