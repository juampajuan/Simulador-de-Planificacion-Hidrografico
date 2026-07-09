use tiny_http::{Header, Request};
use rand::Rng;

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