use yew::prelude::*;
use crate::requests::trigger_path_generation;

#[derive(Properties, PartialEq)]
pub struct PathProps {
    pub separacion: UseStateHandle<String>,
    pub azimut: UseStateHandle<String>,
    pub mensaje: UseStateHandle<String>,
    pub image_url: UseStateHandle<Option<String>>,
}

#[function_component(PathParams)]
pub fn path_params(props: &PathProps) -> Html {
    let on_change = |is_sep: bool, 
                     s: UseStateHandle<String>, 
                     a: UseStateHandle<String>, 
                     m: UseStateHandle<String>, 
                     img: UseStateHandle<Option<String>>| {
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            
            let (curr_sep, curr_az) = if is_sep {
                s.set(val.clone());
                (val, (*a).clone())
            } else {
                a.set(val.clone());
                ((*s).clone(), val)
            };

            trigger_path_generation(curr_sep, curr_az, m.clone(), img.clone());
        })
    };

    html! {
        <div class="border-b border-dashed border-white/40 p-3">
            <h3 class="mb-4 font-bold">{ "Parámetros de Recorrido" }</h3>
            <div class="flex flex-col gap-4">
                <input
                    type="number"
                    placeholder="Separación (mts)"
                    class="rounded p-2 text-black dark:bg-zinc-700 dark:text-white"
                    value={(*props.separacion).clone()}
                    onchange={on_change(true, props.separacion.clone(), props.azimut.clone(), props.mensaje.clone(), props.image_url.clone())}
                />
                <input
                    type="number"
                    placeholder="Azimut"
                    class="rounded p-2 text-black dark:bg-zinc-700 dark:text-white"
                    value={(*props.azimut).clone()}
                    onchange={on_change(false, props.separacion.clone(), props.azimut.clone(), props.mensaje.clone(), props.image_url.clone())}
                />
            </div>
        </div>
    }
}