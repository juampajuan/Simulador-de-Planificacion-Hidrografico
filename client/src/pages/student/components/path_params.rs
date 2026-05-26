use yew::prelude::*;
use crate::requests::trigger_path_generation;

#[derive(Properties, PartialEq)]
pub struct PathProps {
    pub separacion: UseStateHandle<String>,
    pub azimut: UseStateHandle<String>,
    pub gnss_type: UseStateHandle<String>,
    pub mensaje: UseStateHandle<String>,
    pub image_url: UseStateHandle<Option<String>>,
}

#[function_component(PathParams)]
pub fn path_params(props: &PathProps) -> Html {
    let input_cls = "rounded p-2 text-black dark:bg-zinc-700 dark:text-white";

    // Función genérica para disparar la generación
    let trigger = {
        let s = props.separacion.clone();
        let a = props.azimut.clone();
        let g = props.gnss_type.clone();
        let m = props.mensaje.clone();
        let img = props.image_url.clone();
        
        move |new_s: String, new_a: String, new_g: String| {
            trigger_path_generation(new_s, new_a, new_g, m.clone(), img.clone());
        }
    };

    html! {
        <div class="border-b border-dashed border-white/40 p-3">
            <h3 class="mb-4 font-bold">{ "Parámetros de Recorrido" }</h3>
            <div class="flex flex-col gap-4">
                // Input Separación
                <input
                    type="number"
                    placeholder="Separación (mts)"
                    class={input_cls}
                    value={(*props.separacion).clone()}
                    onchange={{
                        let trigger = trigger.clone();
                        let a = props.azimut.clone();
                        let g = props.gnss_type.clone();
                        let s = props.separacion.clone();
                        Callback::from(move |e: Event| {
                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            s.set(val.clone());
                            trigger(val, (*a).clone(), (*g).clone());
                        })
                    }}
                />
                // Input Azimut
                <input
                    type="number"
                    placeholder="Azimut"
                    class={input_cls}
                    value={(*props.azimut).clone()}
                    onchange={{
                        let trigger = trigger.clone();
                        let s = props.separacion.clone();
                        let g = props.gnss_type.clone();
                        let a = props.azimut.clone();
                        Callback::from(move |e: Event| {
                            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            a.set(val.clone());
                            trigger((*s).clone(), val, (*g).clone());
                        })
                    }}
                />
            </div>

            // GNSS movido aquí adentro para que use el estado
            <div class="flex flex-col mt-4">
                <label class="mb-2 font-semibold">{"GNSS"}</label>
                <select 
                    class={input_cls}
                    onchange={{
                        let trigger = trigger.clone();
                        let s = props.separacion.clone();
                        let a = props.azimut.clone();
                        let g = props.gnss_type.clone();
                        Callback::from(move |e: Event| {
                            let val = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                            g.set(val.clone());
                            trigger((*s).clone(), (*a).clone(), val);
                        })
                    }}
                >
                    { for ["Sin corrección", "Corrección DGPS", "Corrección de Fase"].iter().map(|opt| html! {
                        <option value={*opt} selected={*props.gnss_type == *opt}>{ opt }</option>
                    })}
                </select>
            </div>
        </div>
    }
}