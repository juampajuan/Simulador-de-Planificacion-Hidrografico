use yew::prelude::*; 
use lucide_yew::Compass;
 
/// Página 404 que se muestra ante una ruta inexistente.
#[function_component(NotFound)]
pub fn not_found() -> Html {

    html! {
        <div class="bg-slate-900 h-screen w-screen p-3 dark">
            <div class="h-full dot-grid text-white flex flex-col items-center justify-center">
                <Compass size={64} stroke_width={1usize}/>
                <h1 class="text-3xl font-semibold pt-5 mb-8">
                    { "Creemos que te perdiste." }
                </h1>
            </div>
        </div>
    }
}