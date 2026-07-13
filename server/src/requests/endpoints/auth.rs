use super::generic::{server_error, string_response};
use crate::db::encrypt::hash_password;
use crate::db::engine::DBEngine;
use crate::db::queries::auth::TokenOwner;
use crate::db::queries_interface::{auth, professor, student};
use crate::helpers::utils::{check_password, get_cookie};
use crate::logging::logger::send_message_to_logger;
use crate::logging::structs::{LogType, ThreadMessage};
use crate::requests::http_helper::parse_json_body;
use crate::structs::request::HandlerResult;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use tiny_http::{Request, Response};

use crate::helpers::auth::{create_auth_cookie, generate_token, is_admin_request};
use serde_json::Value;
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

/// Endpoint para la creacion de profesores.
/// Antes de crearlo, comprueba si el CLI esta autenticado, datos de la request y los atributos que tendra el nuevo usuario.
pub fn create_professor(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "Iniciando la creación de un nuevo profesor.".to_string(),
        LogType::Debug,
    );

    match is_admin_request(request, &db) {
        Ok(true) => {}
        Ok(false) => {
            send_message_to_logger(
                tx,
                "Intento de creación de profesor sin permisos de administrador.".to_string(),
                LogType::Warn,
            );
            return string_response("Solo permitido para administradores.".to_string(), 403);
        }
        Err(_err) => return server_error("Error autenticando".into()),
    }

    let data: ProfessorAuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return string_response(format!("Bad Request: {}", err), 400),
    };

    if !check_password(&data.pass) {
        return string_response(
            "La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula."
                .to_string(),
            400,
        );
    }

    let password_hash = match hash_password(&data.pass) {
        Ok(hash) => hash,
        Err(_) => return server_error("No se pudo hashear la contraseña.".to_string()),
    };

    let _ = match professor::create_professor_locked(&db, &data.user, &password_hash) {
        Ok(id) => id,
        Err(e) if e.message.as_deref() == Some("Cannot lock db") => {
            send_message_to_logger(
                tx,
                format!(
                    "Intento de crear profesor con username duplicado: '{}'.",
                    data.user
                ),
                LogType::Warn,
            );
            return server_error(
                "Error interno: no se pudo acceder a la base de datos.".to_string(),
            );
        }
        Err(_) => {
            return string_response("Ya existe un profesor con ese username.".to_string(), 409);
        }
    };

    send_message_to_logger(
        tx,
        format!("Profesor '{}' creado correctamente.", data.user),
        LogType::Info,
    );
    string_response("Usuario creado correctamente".to_string(), 200)
}

/// Endpoint para el cambio de contrasena de un usuario de profesor.
/// Previamente comprueba si el CLI esta autenticado
pub fn change_pass(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "se solicito el cambio de contraseña.".to_string(),
        LogType::Debug,
    );

    match is_admin_request(request, &db) {
        Ok(true) => {}
        Ok(false) => {
            send_message_to_logger(
                tx,
                "Intento de cambio de contraseña sin permisos de administrador.".to_string(),
                LogType::Warn,
            );
            return string_response("Solo permitido para administradores.".to_string(), 403);
        }
        Err(_err) => return server_error("Error autenticando".into()),
    }

    let data: ProfessorAuthData = match parse_json_body(request) {
        Ok(d) => d,
        Err(err) => return server_error(format!("Bad Request: {}", err)),
    };

    send_message_to_logger(
        tx,
        format!("Iniciando el cambio de contraseña para {}.", data.user),
        LogType::Debug,
    );

    if !check_password(&data.pass) {
        return string_response(
            "La contraseña debe contener 8 caracteres y al menos 1 numero y una mayuscula."
                .to_string(),
            400,
        );
    }

    let password_hash = match hash_password(&data.pass) {
        Ok(hash) => hash,
        Err(_) => return server_error("No se pudo hashear la contraseña.".to_string()),
    };

    match professor::change_password_by_username_locked(&db, &data.user, &password_hash) {
        Ok(_) => {
            send_message_to_logger(
                tx,
                format!("Contraseña de '{}' actualizada.", data.user),
                LogType::Info,
            );
            string_response("Contraseña cambiada correctamente".to_string(), 200)
        }
        Err(_) => server_error("No se pudo cambiar la contraseña.".to_string()),
    }
}

/// Endpoint del login de la pagina y el CLI.
/// Genera un token, el cual se introduce en la response como una Cookie.
pub fn login(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "Se esta iniciando el login.".to_string(),
        LogType::Debug,
    );

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
            Ok(None) => {
                send_message_to_logger(
                    tx,
                    "Intento de login fallido de alumno (código incorrecto).".to_string(),
                    LogType::Warn,
                );
                return string_response("Datos incorrectos.".to_string(), 401);
            }
            Err(_) => return server_error("Internal error.".to_string()),
        };

        (TokenOwner::Student(student_id), student_name)
    } else {
        let data: ProfessorAuthData = match serde_json::from_value(data) {
            Ok(d) => d,
            Err(_) => return string_response("Bad Request".to_string(), 400),
        };

        let professor_id =
            match professor::verify_professor_credentials_locked(&db, &data.user, &data.pass) {
                Ok(Some(id)) => id,
                Ok(None) => {
                    send_message_to_logger(
                        tx,
                        format!("Intento de login fallido para el usuario '{}'.", data.user),
                        LogType::Warn,
                    );
                    return string_response("Datos incorrectos.".to_string(), 401);
                }
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

    send_message_to_logger(tx, format!("Login exitoso: '{}'.", username), LogType::Info);

    let re = Response::from_string(username).with_header(cookie);
    (re.boxed(), 200, None)
}

/// Elimina todos los token de la DB, efectimante cerrando la sesion de todos.
pub fn close_all(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "Se intenta cerrar todas las sesiones.".to_string(),
        LogType::Debug,
    );

    match is_admin_request(request, &db) {
        Ok(true) => {}
        Ok(false) => {
            send_message_to_logger(
                tx,
                "Intento de cierre masivo de sesiones sin permisos de administrador.".to_string(),
                LogType::Warn,
            );
            return string_response("Solo permitido para administradores.".to_string(), 403);
        }
        Err(_err) => return server_error("Error autenticando".into()),
    }

    if auth::delete_all_tokens_locked(&db).is_err() {
        return server_error("Internal error.".to_string());
    }

    string_response("Todos las sesiones fueron cerradas.".to_string(), 200)
}

/// Cierra la sesion de un solo usuario, sea alumno o profesor.
/// Ademas reemplaza la cookie en la response, para que el navegador rediriga al login.
pub fn close_session(
    request: &mut Request,
    db: Arc<Mutex<DBEngine>>,
    tx: &Sender<ThreadMessage>,
) -> HandlerResult {
    send_message_to_logger(
        tx,
        "Se intenta cerrar la sesion.".to_string(),
        LogType::Debug,
    );

    let auth_token = match get_cookie(request, "auth_token") {
        Some(token) => token,
        None => {
            return string_response("No hay sesión activa.".to_string(), 401);
        }
    };

    if auth::delete_token_locked(&db, &auth_token).is_err() {
        return server_error("Error borrando el token de la DB.".to_string());
    }

    send_message_to_logger(
        tx,
        format!(
            "Sesión cerrada por el usuario con el token {}, ahora inválido.",
            auth_token
        ),
        LogType::Info,
    );

    let mut response = string_response("Sesión cerrada.".to_string(), 200);
    let cookie_killer = "Set-Cookie: auth_token=; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Path=/; HttpOnly; SameSite=Lax";
    if let Ok(header) = tiny_http::Header::from_str(cookie_killer) {
        response.0.add_header(header);
    } else {
        return server_error("Internal error.".to_string());
    }

    response
}
