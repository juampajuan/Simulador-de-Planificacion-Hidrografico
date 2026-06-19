mod router;
mod pages;
mod components;
mod services;
mod parser;
mod structs;
mod protected_route;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <router::AppRouter />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}