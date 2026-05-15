use yew::prelude::*;
use yew_router::prelude::*;

use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;

use crate::router::Route;

#[function_component(StudentPage)]
pub fn student_page() -> Html {

    let mensaje = use_state(|| "Cargando...".to_string());

    {
        let mensaje = mensaje.clone();

        use_effect_with((), move |_| {

            spawn_local(async move {

                let response = Request::get("/")
                    .send()
                    .await
                    .unwrap();

                let text = response
                    .text()
                    .await
                    .unwrap();

                mensaje.set(text);
            });

            || ()
        });
    }

    html! {
        <div class="h-screen bg-zinc-900 text-white flex flex-col items-center justify-center">

            <h1 class="text-5xl font-bold mb-8">
                { "STUDENT" }
            </h1>

            <p class="text-2xl p-3 bg-green-800 rounded">
                { (*mensaje).clone() }
            </p>

        </div>
    }
}