use yew::prelude::*;
use crate::{
    components::subtitle::Subtitle,
    services::requests::trigger_path_generation,
    structs::state::{PathState, SimulationUiState},
};
use crate::structs::limits::ConfigLimits;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use lucide_yew::Route;

#[derive(Properties, PartialEq)]
pub struct PathProps {
    pub path_state: UseStateHandle<PathState>,
    pub ui_state: SimulationUiState,
    pub limits: UseStateHandle<ConfigLimits>,
}

// Muestra parámetros para path: separacion, azimut y gnss. Bo
#[function_component(PathParams)]
pub fn path_params(props: &PathProps) -> Html {
    let input_cls =
        "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white disabled:dark:text-white/50 disabled:dark:bg-zinc-600";

    let ui_state_clone = props.ui_state.clone();
    let path_state_clone = props.path_state.clone();
    let limits_clone = props.limits.clone();

    let on_visualize_click = Callback::from(move |_| {
        trigger_path_generation(&path_state_clone, ui_state_clone.clone(), &limits_clone);
    });

    html! {
        // border-white/15 p-3 bg-zinc-900 rounded-md border flex flex-col gap-3
        <div class="border-white/25 p-3 pt-0 border-b flex flex-col gap-3">

            <Subtitle
                text={"1. Recorrido"}
                icon={html! {
                    <Route size={18} />
                }}
            />

            <div class="grid grid-cols-2 gap-3">

                <div class="flex flex-col gap-1">
                    <span class="text-xs font-semibold text-white/40 ml-1">
                        {"Separación (mts)"}
                    </span>

                    <input
                        type="number"
                        disabled={*props.ui_state.loading}
                        placeholder="10"
                        class={input_cls}
                        value={(props.path_state).separacion.clone()}
                        onchange={{
                            let s_handle = props.path_state.clone();
                            Callback::from(move |e: Event| {
                                let val = e.target_unchecked_into::<HtmlInputElement>().value();
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
                        disabled={*props.ui_state.loading}
                        placeholder="45"
                        class={input_cls}
                        value={(props.path_state).azimut.clone()}
                        onchange={{
                            let a_handle = props.path_state.clone();
                            Callback::from(move |e: Event| {
                                let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                let mut nuevo_estado = (*a_handle).clone();
                                nuevo_estado.azimut = val;
                                a_handle.set(nuevo_estado);
                            })
                        }}
                    />
                </div>

            </div>

            <div class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-white/40 ml-1">
                    {"GNSS"}
                </span>

                <select
                    class={input_cls}
                    disabled={*props.ui_state.loading}
                    onchange={{
                        let g_handle = props.path_state.clone();
                        Callback::from(move |e: Event| {
                            let val = e.target_unchecked_into::<HtmlSelectElement>().value();
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
                                selected={(props.path_state).gnss_type == *opt}
                            >
                                { opt }
                            </option>
                        })
                    }
                </select>
            </div>
 
            <div class="flex justify-end">
                <button
                    disabled={*props.ui_state.loading}
                    class="text-center w-48 disabled:opacity-30 bg-cyan-200 p-2 px-6 text-black text-sm font-bold hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100"
                    onclick={on_visualize_click} 
                >
                    {"Visualizar recorrido"}
                </button>
            </div>

        </div>
    }
}