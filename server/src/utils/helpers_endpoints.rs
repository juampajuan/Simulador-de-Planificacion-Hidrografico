use std::sync::{Arc, Mutex};

use crate::{db::{engine::DBEngine, queries::auth::TokenOwner, queries_interface::auth}, utils::helpers::get_cookie};


pub fn check_profesor_auth(request: &tiny_http::Request, db: &Arc<Mutex<DBEngine>>) -> Result<Option<i64>,String> {

    let Some(token) = get_cookie(request, "auth_token") else {
        return Ok(None);
    };

    match auth::get_user_by_token_locked(&db, &token) {
        Ok(Some(TokenOwner::Professor(id))) => Ok(Some(id)),
        Ok(_) => Ok(None),
        Err(e) => Err(e.to_string())
    }
}

pub fn check_student_auth(request: &tiny_http::Request, db: &Arc<Mutex<DBEngine>>) -> Result<Option<i64>,String> {

    let Some(token) = get_cookie(request, "auth_token") else {
        return Ok(None);
    };

    match auth::get_user_by_token_locked(&db, &token) {
        Ok(Some(TokenOwner::Student(id))) => Ok(Some(id)),
        Ok(_) => Ok(None),
        Err(e) => Err(e.to_string())
    }
}
