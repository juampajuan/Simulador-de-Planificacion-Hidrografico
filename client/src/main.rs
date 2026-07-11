mod components;
mod pages;
mod router;
use crate::components::no_responsive::NoResponsive;
mod parser;
mod protected_route;
mod services;
mod structs;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {<>
        <router::AppRouter/>
        <NoResponsive/>
    </>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
