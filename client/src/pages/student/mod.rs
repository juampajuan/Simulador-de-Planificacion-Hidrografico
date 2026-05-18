use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use crate::components::title::{Title};

#[function_component(StudentPage)]
pub fn student_page() -> Html {

    let mensaje = use_state(|| "Cargando...".to_string());

    {
        let mensaje = mensaje.clone();

        use_effect_with((), move |_| {

            spawn_local(async move {

                // TODO: usar el metodo sin terminar de /requests.rs
                let response = Request::get("localhost:3000/api/v1/users")
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
        <div class="h-screen bg-blue-900 text-white flex flex-col items-center justify-center">

            // Esto es un componente generico
            <Title text={"STUDENT".to_string()} />
 
            <p class="text-2xl p-3 bg-green-800 rounded">
                { (*mensaje).clone() }
            </p>

        </div>
    }
}


// generar_recorrido()

// simular()