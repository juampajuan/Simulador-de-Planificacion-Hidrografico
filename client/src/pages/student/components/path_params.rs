use yew::prelude::*;
use crate::{
    components::subtitle::Subtitle,
    requests::{PathState, trigger_path_generation},
};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use lucide_yew::Route;

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
    let input_cls =
        "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white disabled:dark:text-white/50 disabled:dark:bg-zinc-600";

    let loading = use_state(|| false);

    let trigger = {
        let loading = loading.clone();
        let m = props.mensaje.clone();
        let img = props.image_url.clone();

        move |s: String, a: String, g: String| {
            let state = PathState {
                separacion: s,
                azimut: a,
                gnss_type: g,
            };
            trigger_path_generation(state, m.clone(), img.clone(), loading.clone());
        }
    };

    html! {
        <div class="border-white/15 p-3 bg-zinc-900 rounded-md border flex flex-col gap-3">

            <Subtitle
                text={"1. Recorrido"}
                icon={html! {
                    <Route size={18} />
                }}
            />

            <div class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-white/40 ml-1">
                    {"Separación (mts)"}
                </span>

                <input
                    type="number"
                    disabled={*loading}
                    placeholder="10"
                    class={input_cls}
                    value={(*props.separacion).clone()}
                    onchange={{
                        let s_handle = props.separacion.clone();

                        Callback::from(move |e: Event| {
                            let val = e
                                .target_unchecked_into::<HtmlInputElement>()
                                .value();

                            s_handle.set(val);
                        })
                    }}
                />
            </div>

            <div class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-white/40 ml-1">
                    {"Azimut (Grados)"}
                </span>

                <input
                    type="number"
                    disabled={*loading}
                    placeholder="45"
                    class={input_cls}
                    value={(*props.azimut).clone()}
                    onchange={{
                        let a_handle = props.azimut.clone();

                        Callback::from(move |e: Event| {
                            let val = e
                                .target_unchecked_into::<HtmlInputElement>()
                                .value();

                            a_handle.set(val);
                        })
                    }}
                />
            </div>

            <div class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-white/40 ml-1">
                    {"GNSS"}
                </span>

                <select
                    class={input_cls}
                    disabled={*loading}
                    onchange={{
                        let g_handle = props.gnss_type.clone();

                        Callback::from(move |e: Event| {
                            let val = e
                                .target_unchecked_into::<HtmlSelectElement>()
                                .value();

                            g_handle.set(val);
                        })
                    }}
                >
                    {
                        for [
                            "Corrección de Fase",
                            "Corrección DGPS",
                            "Sin corrección"
                        ]
                        .iter()
                        .map(|opt| html! {
                            <option
                                value={*opt}
                                selected={*props.gnss_type == *opt}
                            >
                                { opt }
                            </option>
                        })
                    }
                </select>
            </div>
 
            <div class="flex justify-end">
                <button
                    class="text-center w-48 disabled:opacity-30 bg-cyan-200 p-2 px-6 text-black text-sm font-bold hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100"
                    onclick={{
                        let trigger = trigger.clone();

                        let s = props.separacion.clone();
                        let a = props.azimut.clone();
                        let g = props.gnss_type.clone();

                        Callback::from(move |_| {
                            trigger(
                                (*s).clone(),
                                (*a).clone(),
                                (*g).clone(),
                            );
                        })
                    }}
                >

                    {
                        if *loading {
                            html! {<div class="loader"/>}
                        } else {
                            html! {"Visualizar recorrido"}
                        }
                    }
                    
                </button>
            </div>

        </div>
    }
}