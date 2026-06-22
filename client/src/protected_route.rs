use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::router::Route;

#[derive(PartialEq, Clone, Copy)]
pub enum Role {
    Student,
    Admin,
}

#[derive(Properties, PartialEq)]
pub struct ProtectedProps {
    pub children: Children,
    pub required_role: Role,
}

/// Revisa que la url que se solicita perteneza al dominio del rol.
// Si el required_role para visitar la pagina children no coincide con el rol actual, se devuelve a su pagina correspondiente
// Si el required role coincide con el rol actual, todo sigue normal.
#[function_component(ProtectedRoute)]
pub fn protected_route(props: &ProtectedProps) -> Html {
    let navigator = use_navigator().unwrap();
    
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    
    let session = storage.get_item("group_or_user_name").unwrap_or(None);
    let role_saved = storage.get_item("user_role").unwrap_or(None);
    
    let is_admin = role_saved.as_deref() == Some("admin");

    if session.is_none() {
        navigator.replace(&Route::Login);
        return html! {
            <div class="h-screen w-screen bg-zinc-950 flex items-center justify-center text-white font-semibold text-sm">
                {"Redireccionando al inicio de sesión..."}
            </div>
        };
    }

    match props.required_role {
        Role::Student if is_admin => {
            navigator.replace(&Route::AdminProjects);
            return html! {};
        }
        Role::Admin if !is_admin => {
            navigator.replace(&Route::Student);
            return html! {};
        }
        _ => {}
    }

    html! { <>{ props.children.clone() }</> }
}