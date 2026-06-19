use yew::prelude::UseStateHandle;
use crate::services::utils::set_local_storage;

pub fn process_local_login(display_name: &str, redirection: &str, mensaje: UseStateHandle<String>) {
    mensaje.set("Login exitoso. Redirigiendo...".to_string());
    set_local_storage("group_or_user_name", display_name);

    if let Some(w) = web_sys::window() {
        let _ = w.location().set_href(redirection); 
    }
}

pub fn process_local_logout(redirection: &str) {
    set_local_storage("group_or_user_name", "");
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_href(redirection);
    }
}