use crate::services::utils::set_local_storage;

/// Persiste de manera segura los datos de la sesión tras un login exitoso.
pub fn process_local_login(display_name: &str, role: &str) {
    set_local_storage("group_or_user_name", display_name);
    set_local_storage("user_role", role);
}

/// Limpia por completo las credenciales locales y redirige de forma dura al login.
pub fn process_local_logout(redirection: &str) {
    set_local_storage("group_or_user_name", "");
    set_local_storage("user_role", ""); // Aseguramos limpiar también el rol

    if let Some(w) = web_sys::window() {
        let _ = w.location().set_href(redirection);
    }
}
