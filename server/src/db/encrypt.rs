use bcrypt::{hash, verify, DEFAULT_COST};

/// Genera el hash bcrypt de una contraseña (con el costo por defecto) para guardarlo en la base.
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Compara una contraseña en texto plano contra su hash bcrypt.
/// Devuelve `true` solo si coinciden; ante cualquier error de verificación devuelve `false`.
pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}