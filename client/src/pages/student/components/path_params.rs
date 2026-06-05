use yew::prelude::*;
use crate::{
    components::subtitle::Subtitle,
    requests::{PathState, trigger_path_generation},
};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use lucide_yew::Route;

#[derive(Properties, PartialEq)]
pub struct PathProps {
    pub path_state: UseStateHandle<PathState>,
    pub mensaje: UseStateHandle<String>,
    pub image_url: UseStateHandle<Option<String>>,
    pub loading: UseStateHandle<bool>
}

#[function_component(PathParams)]
pub fn path_params(props: &PathProps) -> Html {
    let input_cls =
        "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white disabled:dark:text-white/50 disabled:dark:bg-zinc-600";

    // let loading = use_state(|| false);

    let trigger = {
        let loading = props.loading.clone();
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
                    disabled={*props.loading}
                    placeholder="10"
                    class={input_cls}
                    value={(*props.path_state).separacion.clone()}
                    onchange={{
                        let s_handle = props.path_state.clone();

                        Callback::from(move |e: Event| {
                            let val = e
                                .target_unchecked_into::<HtmlInputElement>()
                                .value();
                            let mut nuevo_estado = (*s_handle).clone();
                            nuevo_estado.separacion = val;
                            s_handle.set(nuevo_estado);
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
                    disabled={*props.loading}
                    placeholder="45"
                    class={input_cls}
                    value={(*props.path_state).azimut.clone()}
                    onchange={{
                        let a_handle = props.path_state.clone();

                        Callback::from(move |e: Event| {
                            let val = e
                                .target_unchecked_into::<HtmlInputElement>()
                                .value();

                            let mut nuevo_estado = (*a_handle).clone();
                            nuevo_estado.azimut = val;
                            a_handle.set(nuevo_estado);
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
                    disabled={*props.loading}
                    onchange={{
                        let g_handle = props.path_state.clone();

                        Callback::from(move |e: Event| {
                            let val = e
                                .target_unchecked_into::<HtmlSelectElement>()
                                .value();

                            let mut nuevo_estado = (*g_handle).clone();
                            nuevo_estado.gnss_type = val;
                            g_handle.set(nuevo_estado);
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
                                selected={(*props.path_state).gnss_type == *opt}
                            >
                                { opt }
                            </option>
                        })
                    }
                </select>
            </div>
 
            <div class="flex justify-end">
                <button
                    disabled={*props.loading}
                    class="text-center w-48 disabled:opacity-30 bg-cyan-200 p-2 px-6 text-black text-sm font-bold hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100"
                    onclick={{
                        let trigger = trigger.clone();

                        let s = props.path_state.clone();
                        let a = props.path_state.clone();
                        let g = props.path_state.clone();

                        Callback::from(move |_| {
                            trigger(
                                (*s).separacion.clone(),
                                (*a).azimut.clone(),
                                (*g).gnss_type.clone(),
                            );
                        })
                    }}
                >
                    {"Visualizar recorrido"}
                </button>
            </div>

        </div>
    }
}