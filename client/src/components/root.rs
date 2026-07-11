use crate::services::requests::trigger_logout;
use crate::services::utils::get_local_storage;
use yew::prelude::*;

use super::title::Title;
use lucide_yew::LogOut;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RootProps {
    pub title: AttrValue,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Root)]
pub fn root(props: &RootProps) -> Html {
    let location = use_location();
    let is_login = location.as_ref().is_some_and(|l| l.path() == "/login");

    html! {
        <main class="
            h-screen
            w-full
            bg-slate-900
            flex
            flex-col
            transition-colors
            duration-300
        ">
            if !is_login {
                <div class="pt-2 px-3 flex justify-between items-center h-14">
                    <div>
                        <Title text={&props.title}
                            icon={html! {
                                <img width="36px" src="/static/icon.png"/>
                            }}
                        />
                    </div>
                    <div class="flex gap-2 items-center">
                        <UserButton/>
                    </div>
                </div>
            }

            // Contenido principal
            <section class="h-full flex flex-1 gap-2 p-2 overflow-hidden">
                { for props.children.iter() }
            </section>
        </main>
    }
}

/// Muestra el nombre del usuario logueado y un botón de logout. Se oculta en login o sin sesión.
#[function_component(UserButton)]
pub fn user_button() -> Html {
    let location = use_location();
    let is_login = location.as_ref().is_some_and(|l| l.path() == "/login");
    let token = get_local_storage("group_or_user_name").unwrap_or_default();

    let on_logout_click = Callback::from(move |_| {
        trigger_logout();
    });

    if is_login || token.is_empty() {
        html! {}
    } else {
        html! {
            <div class="flex pl-3 gap-3 p-1 items-center rounded-full text-white bg-white/10">
                <p class="text-sm">{token}</p>
                <button
                    onclick={on_logout_click}
                    class="hover:text-white rounded-full p-2 hover:bg-red-700 cursor-pointer transition-colors"
                >
                    <LogOut size={18}/>
                </button>
            </div>
        }
    }
}
