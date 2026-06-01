mod router;
mod pages;
mod components;
mod requests;
mod blob_client;
mod parser;

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