mod router;
mod pages;
mod components;
mod requests;

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