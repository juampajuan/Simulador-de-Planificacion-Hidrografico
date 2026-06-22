mod router;
mod pages;
mod components;
use crate::components::no_responsive::NoResponsive;
mod services;
mod parser;
mod structs;
mod protected_route;
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