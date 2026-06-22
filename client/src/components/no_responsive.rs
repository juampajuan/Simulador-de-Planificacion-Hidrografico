use yew::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;
use lucide_yew::Smartphone; 
use wasm_bindgen::closure::Closure;

/// Aviso a pantalla completa que aparece si la ventana es muy angosta (< 768px),
/// pidiendo usar una pantalla más grande. Se actualiza al redimensionar.
#[function_component(NoResponsive)]
pub fn no_responsive() -> Html {
    let max_width = 768;

    let width = use_state(|| {
        window()
            .unwrap()
            .inner_width()
            .unwrap()
            .as_f64()
            .unwrap() as i32
    });

    {
        let width = width.clone();

        use_effect_with((), move |_| {
            let window = window().unwrap();

            let closure = Closure::<dyn FnMut(web_sys::Event)>::wrap(Box::new(move |_event: web_sys::Event| {
                let window = web_sys::window().unwrap();

                if let Ok(w) = window.inner_width() 
                    && let Some(w) = w.as_f64() {
                        width.set(w as i32);
                }
                
            }));

            window
                .add_event_listener_with_callback(
                    "resize",
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();

            // importante: evitar drop del closure
            closure.forget();

            || ()
        });
    }

    if *width >= max_width {
        return html! {};
    }


    html! {
        <div class="bg-slate-900/50 backdrop-blur h-screen w-screen p-3 dark absolute top-0 left-0 z-2000">
            <div class="h-full dot-grid text-white flex flex-col items-center justify-center">
                <Smartphone size={64} stroke_width={1usize}/>
                <h1 class="text-2xl font-semibold pt-5 text-center px-12">
                    { "Lamentablemente, por el momento, no soportamos este tamaño de pantalla." }
                </h1>
                <h1 class="pt-2 mb-8 text-center px-12 text-white/80">
                    { "Para un uso óptimo, recomendamos acceder desde una computadora o tablet." }
                </h1>
            </div>
        </div>
    }
}