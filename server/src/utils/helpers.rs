use std::path::Path;
use std::fs;
use rand::Rng;

pub fn create_dirs(path: &str) -> Option<()> {
    fs::create_dir_all(Path::new(path).join("geotiffs")).ok()?;
    fs::create_dir_all(Path::new(path).join("simulations")).ok()?;
    Some(())
}

pub fn check_password(pass: &str) -> bool {
    let has_upper = pass.chars().any(|c| c.is_uppercase());
    let has_number = pass.chars().any(|c| c.is_numeric());
    let ok_length = pass.len() >= 8;
    has_upper && has_number && ok_length
}

pub fn generate_code() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();

    (0..6)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn get_cookie(request: &tiny_http::Request, name: &str) -> Option<String> {
    let cookie_header = request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Cookie"))?;

    cookie_header
        .value
        .as_str()
        .split(';')
        .find_map(|cookie| {
            let (key, value) = cookie.trim().split_once('=')?;
            if key == name {
                Some(value.to_string())
            } else {
                None
            }
        })
}