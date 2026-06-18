use yew::prelude::*;

#[function_component(LoginWelcome)]
pub fn login_welcome() -> Html {
    html! {
        <div class="relative flex justify-center flex-col p-8 pt-0 pb-12 items-center">
            <div class="space-y-2">
                <h1 class="font-semibold text-4xl font-shadow dark:text-cyan-100 text-cyan-800 flex items-center gap-2">
                    { "Bienvenido al" }
                </h1>
                <h1 class="font-semibold text-6xl font-shadow dark:text-cyan-100 text-cyan-800 flex items-center gap-2">
                    { "Simulador de Planificación" }
                </h1>
                <h1 class="font-semibold text-6xl font-shadow dark:text-cyan-100 text-cyan-800 flex items-center gap-2">
                    { "Hidrográfico" }
                </h1>
            </div>
        </div>
    }
}