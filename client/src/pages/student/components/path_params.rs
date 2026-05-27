use yew::prelude::*;
use crate::requests::{trigger_path_generation, PathState}; // Importamos PathState
use web_sys::{HtmlInputElement, HtmlSelectElement};

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

    let trigger = {
        let m = props.mensaje.clone();
        let img = props.image_url.clone();
        move |s: String, a: String, g: String| {
            let state = PathState {
                separacion: s,
                azimut: a,
                gnss_type: g,
            };
            trigger_path_generation(state, m.clone(), img.clone());
        }
    };

    html! {
        <div class="border-b border-dashed border-white/40 p-3">
            <h3 class="mb-4 font-bold text-white">{ "Parámetros de Recorrido" }</h3>
            <div class="flex flex-col gap-4">
                <input
                    type="number"
                    placeholder="Separación (mts)"
                    class={input_cls}
                    value={(*props.separacion).clone()}
                    onchange={{
                        let trigger = trigger.clone();
                        let a = (*props.azimut).clone();
                        let g = (*props.gnss_type).clone();
                        let s_handle = props.separacion.clone();
                        Callback::from(move |e: Event| {
                            let val = e.target_unchecked_into::<HtmlInputElement>().value();
                            s_handle.set(val.clone());
                            trigger(val, a.clone(), g.clone());
                        })
                    }}
                />
                <input
                    type="number"
                    placeholder="Azimut"
                    class={input_cls}
                    value={(*props.azimut).clone()}
                    onchange={{
                        let trigger = trigger.clone();
                        let s = (*props.separacion).clone();
                        let g = (*props.gnss_type).clone();
                        let a_handle = props.azimut.clone();
                        Callback::from(move |e: Event| {
                            let val = e.target_unchecked_into::<HtmlInputElement>().value();
                            a_handle.set(val.clone());
                            trigger(s.clone(), val, g.clone());
                        })
                    }}
                />
            </div>

            <div class="flex flex-col mt-4">
                <label class="mb-2 font-semibold text-white">{"GNSS"}</label>
                <select 
                    class={input_cls}
                    onchange={{
                        let trigger = trigger.clone();
                        let s = (*props.separacion).clone();
                        let a = (*props.azimut).clone();
                        let g_handle = props.gnss_type.clone();
                        Callback::from(move |e: Event| {
                            let val = e.target_unchecked_into::<HtmlSelectElement>().value();
                            g_handle.set(val.clone());
                            trigger(s.clone(), a.clone(), val);
                        })
                    }}
                >
                    { for ["Corrección de Fase", "Corrección DGPS", "Sin corrección"].iter().map(|opt| html! {
                        <option value={*opt} selected={*props.gnss_type == *opt}>{ opt }</option>
                    })}
                </select>
            </div>
        </div>
    }
}