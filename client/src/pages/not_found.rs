use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {

    html! {
        <div class="h-screen bg-zinc-900 text-white flex flex-col items-center justify-center">

            <h1 class="text-5xl font-bold mb-8">
                { "TE PERDISTE" }
            </h1>

        </div>
    }
}