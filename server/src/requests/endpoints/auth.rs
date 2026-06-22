use tiny_http::{Header, Response, Request};
use crate::utils::helpers::{check_password, get_cookie};
use crate::db::queries::auth::{TokenOwner};
use crate::structs::request::{HandlerResult};
use crate::requests::http_helper::{parse_json_body};
use crate::utils::helpers_endpoints::check_profesor_auth;
use std::sync::{Arc, Mutex};
use crate::db::encrypt::{hash_password};
use crate::db::queries_interface::{auth, professor, student};
use super::generic::{server_error, string_response};
use crate::db::engine::DBEngine;
use serde_json::Value;
use rand::Rng;
use std::str::FromStr;

#[derive(serde::Deserialize)]
pub struct ProfessorAuthData {
    #[serde(default)]
    pub user: String,
    pub pass: String, 
}

#[derive(serde::Deserialize)]
pub struct StudentAuthData {
    #[serde(default)]
    pub code: String, 
}

///Endpoint para la creacion de profesores. Revisa permisos, datos de la request y los atributos que tendra el nuevo usuario.
///Intenta acceder a la base de datos con un lock. 
pub fn create_professor(request: &mut Request, db: Arc<Mutex<DBEngine>>) -> HandlerResult {

    match is_admin_request(request, &db) {
        Ok(true) => {}
        Ok(false) => return string_response("Solo permitido para administradores.".to_string(),403),
        Err(_err) => return server_error("Error autenticando".into()),
    }

    let data: ProfessorAuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return string_response(format!("Bad Request: {}", err), 400),
    };

    if !check_password(&data.pass){
        return string_response("La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula.".to_string(), 400)
    }

    let password_hash = match hash_password(&data.pass) {
        Ok(hash) => hash,
        Err(_) =>  return server_error("No se pudo hashear la contraseña.".to_string())
    };

    let _ = match professor::create_professor_locked(&db, &data.user, &password_hash) {
        Ok(id) => id,
        Err(e) if e.message.as_deref() == Some("Cannot lock db") => {
            return server_error("Error interno: no se pudo acceder a la base de datos.".to_string())
        }
        Err(_) => return string_response("Ya existe un profesor con ese username.".to_string(), 409),
    };

    string_response("Usuario creado correctamente".to_string(), 200)
}

///Endpoint para el cambio de contrasena de un usuario.
pub fn change_pass(request: &mut Request, db: Arc<Mutex<DBEngine>>) -> HandlerResult {

    match is_admin_request(request, &db) {
        Ok(true) => {}
        Ok(false) => return string_response("Solo permitido para administradores.".to_string(),403),
        Err(_err) => return server_error("Error autenticando".into()),
    }

    let data: ProfessorAuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    if !check_password(&data.pass){
        return string_response("La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula.".to_string(), 400)
    }

    let password_hash = match hash_password(&data.pass) {
        Ok(hash) => hash,
        Err(_) =>  return server_error("No se pudo hashear la contraseña.".to_string())
    };


    match professor::change_password_by_username_locked(&db, &data.user, &password_hash) {
        Ok(_) => string_response("Contraseña cambiada correctamente".to_string(), 200),
        Err(_) => server_error("No se pudo cambiar la contraseña.".to_string())
    }

}

///Endpoint del login de la pagina; Se usa un token para generar una cookie para retornar. Esta es usada para el seguimiento de la sesion.
pub fn login(request: &mut Request, db: Arc<Mutex<DBEngine>>) -> HandlerResult {

    let data: Value = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return string_response(format!("Bad Request: {}", err), 400),
    };

    let (owner, username) = if data.get("code").is_some() {
        let data: StudentAuthData = match serde_json::from_value(data) {
            Ok(d) => d,
            Err(_) => return string_response("Bad Request".to_string(), 400),
        };

        let (student_id, student_name) = match student::verify_code_locked(&db, &data.code) {
            Ok(Some((id, name))) => (id, name),
            Ok(None) => return string_response("Datos incorrectos.".to_string(), 401),
            Err(_) => return server_error("Internal error.".to_string()),
        };

        (TokenOwner::Student(student_id), student_name)

    } else {
        let data: ProfessorAuthData = match serde_json::from_value(data) {
            Ok(d) => d,
            Err(_) => return string_response("Bad Request".to_string(), 400),
        };

        let professor_id = match professor::verify_professor_credentials_locked(
            &db,
            &data.user,
            &data.pass,
        ) {
            Ok(Some(id)) => id,
            Ok(None) => return string_response("Datos incorrectos.".to_string(), 401),
            Err(_) => return server_error("Internal error.".to_string()),
        };

        (TokenOwner::Professor(professor_id), data.user)
    };

    let token = generate_token();

    if auth::create_token_locked(&db, owner, &token, 7).is_err() {
        return server_error("Internal error.".to_string());
    }

    let cookie = match create_auth_cookie(&token) {
        Ok(cookie) => cookie,
        Err(_) => return server_error("Internal error.".to_string()),
    };

    let re = Response::from_string(username).with_header(cookie);
    (re.boxed(), 200, None)
}

///Se eliminan todos los registros de token y cookies para dar por cerradas todas las sesiones.
pub fn close_all(request: &mut Request, db: Arc<Mutex<DBEngine>>) -> HandlerResult {

    match is_admin_request(request, &db) {
        Ok(true) => {}
        Ok(false) => return string_response("Solo permitido para administradores.".to_string(),403),
        Err(_err) => return server_error("Error autenticando".into()),
    }

    if auth::delete_all_tokens_locked(&db).is_err() {
        return server_error("Internal error.".to_string());
    }

    string_response("Todos las sesiones fueron cerradas.".to_string(), 200)
}

///Se borra un token-cookie especifico para dar por finalizada una sesion.
pub fn close_session(request: &mut Request, db: Arc<Mutex<DBEngine>>) -> HandlerResult {

    let auth_token = match get_cookie(request, "auth_token") {
        Some(token) => token,
        None => {
            return string_response(
                "No hay sesión activa.".to_string(),
                401
            );
        }
    };

    if auth::delete_token_locked(&db, &auth_token).is_err() {
        return server_error("Internal error.".to_string());
    }

    let mut response = string_response(
        "Sesión cerrada.".to_string(),
        200
    );
    let cookie_killer = "Set-Cookie: auth_token=; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Path=/; HttpOnly; SameSite=Lax";
    if let Ok(header) = tiny_http::Header::from_str(cookie_killer) {
        response.0.add_header(header);
    } else {
        return server_error("Internal error.".to_string());
    }

    response
} 

///Valida si la request fue formada por alguen con permisos de administrador, basandose en el id en la request.
fn is_admin_request(
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

///Determina si una request fue formada en el sistema local.
fn is_local_request(request: &Request) -> bool {
    match request.remote_addr() {
        Some(addr) => addr.ip().is_loopback(),
        None => false,
    }
}


///Genera la cookie usada para la sesion, usando el toke previamente generado.
fn create_auth_cookie(
    token: &str,
) -> Result<Header, ()> {

    let cookie = format!(
        "auth_token={}; Path=/; Max-Age=604800; HttpOnly; SameSite=Lax; Domain=localhost",
        token
    );

    Header::from_bytes("Set-Cookie", cookie)
        .map_err(|_| ())
}

///Genera un token random.
fn generate_token() -> String {
    let bytes: [u8; 32] = rand :: rng().random();
    hex::encode(bytes)
}