use yew::prelude::*;
use crate::{components::subtitle::Subtitle, requests::{PathState, trigger_path_generation}}; // Importamos PathState
use web_sys::{HtmlInputElement, HtmlSelectElement};
use lucide_yew::{Route};

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
    let input_cls = "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white";

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
        <div class="border-white/15 p-3 bg-zinc-900 rounded-md border flex flex-col gap-3">
            <Subtitle text={"1. Recorrido"} 
                icon={html! {
                    <Route size={18} />
                }}
            />
             
            <div class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-white/40 ml-1">{"Separación (mts)"}</span>
                <input
                    type="number"
                    placeholder="10"
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
            </div>

            <div class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-white/40 ml-1">{"Azimut (Grados)"}</span>
                <input
                    type="number"
                    placeholder="45"
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
    
            <div class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-white/40 ml-1">{"GNSS"}</span>
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