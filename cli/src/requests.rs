use serde::Serialize;
use reqwest::blocking::Client;

#[derive(Serialize)]
struct UserRequest {
    user: String,
    pass: String,
}

/// Genera la estructura de cliente HTTP, habilitando las cookies
// Para poder hacer las requets autenticado.
pub fn generate_client() -> Result<Client, reqwest::Error> {
    let client = Client::builder()
    .cookie_store(true)
    .build()?;

    Ok(client)
}

/// Ejecuta la request de login, para autenticar el CLI
pub fn login(host: &str, pass: &str, client: &Client) -> Result<(String, u16), Box<dyn std::error::Error>> {

    println!("\n\x1b[36mIniciando sesion como admin...\x1b[0m");

    let body = UserRequest {
        user: "admin".into(),
        pass: pass.into(),
    };

    let response = client
        .post(format!("{}/api/v1/auth/login", host))
        .json(&body)
        .send()?;

    let code = response.status().as_u16();
    let text = response.text()?;
    Ok((text, code))
}

/// Realiza la request para crear un nuevo usuario de Docente
pub fn create_user(
    client: &Client,
    host: &str,
    user: &str,
    pass: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    let body = UserRequest {
        user: user.into(),
        pass: pass.into(),
    };
 
    let response = client
        .post(format!("{}/api/v1/auth/create_professor_user", host))
        .json(&body)
        .send()?;

    let text = response.text()?;
    Ok(text)
}

/// Realiza la request para cambiar la pass del Docente
pub fn change_pass(
    client: &Client,
    host: &str,
    user: &str,
    pass: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    let body = UserRequest {
        user: user.into(),
        pass: pass.into(),
    };

    let response = client
        .post(format!("{}/api/v1/auth/change_professor_pass", host))
        .json(&body)
        .send()?;

    let text = response.text()?;
    Ok(text)
}

/// Realiza la request para cerrar todas las sesiones
pub fn close_all(
    client: &Client,
    host: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    let response = client
        .post(format!("{}/api/v1/auth/close_all", host))
        .send()?;

    let text = response.text()?;
    Ok(text)
}