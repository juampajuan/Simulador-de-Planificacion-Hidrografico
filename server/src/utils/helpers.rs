use rand::Rng;
use std::fs;
use std::path::Path;

// Utilidades generales transversales: creación de carpetas, validación de contraseñas,
// generación de códigos de alumno y lectura de cookies.

/// Crea las carpetas `geotiffs/` y `simulations/` dentro de `path` (incluyendo los padres).
/// Devuelve `None` si alguna creación falla.
pub fn create_dirs(path: &str) -> Option<()> {
    fs::create_dir_all(Path::new(path).join("geotiffs")).ok()?;
    fs::create_dir_all(Path::new(path).join("simulations")).ok()?;
    fs::create_dir_all(Path::new(path).join("images")).ok()?;
    Some(())
}

/// Valida que una contraseña cumpla los requisitos mínimos: al menos una mayúscula,
/// al menos un número y 8 caracteres o más.
pub fn check_password(pass: &str) -> bool {
    let has_upper = pass.chars().any(|c| c.is_uppercase());
    let has_number = pass.chars().any(|c| c.is_numeric());
    let ok_length = pass.len() >= 8;
    has_upper && has_number && ok_length
}

/// Genera un código de acceso aleatorio de 6 caracteres (letras mayúsculas y dígitos),
/// usado como "login" de los alumnos.
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

/// Extrae la cookie del header de la request, con el nombre introducido en la firma.
/// Devuelve su valor si existe, o `None` si no está la cookie o el header.
pub fn get_cookie(request: &tiny_http::Request, name: &str) -> Option<String> {
    let cookie_header = request.headers().iter().find(|h| h.field.equiv("Cookie"))?;

    cookie_header.value.as_str().split(';').find_map(|cookie| {
        let (key, value) = cookie.trim().split_once('=')?;
        if key == name {
            Some(value.to_string())
        } else {
            None
        }
    })
}

// Genera 5 letras random para el guardado de imagenes
pub fn random_letters(n: usize) -> String {
    const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::rng();

    (0..n)
        .map(|_| {
            let idx = rng.random_range(0..LETTERS.len());
            LETTERS[idx] as char
        })
        .collect()
}
