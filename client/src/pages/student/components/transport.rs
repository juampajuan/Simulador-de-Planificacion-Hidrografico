use yew::prelude::*;
use web_sys::HtmlInputElement;
use common::Transport;
use lucide_yew::Ship;
use crate::{components::subtitle::Subtitle, structs::state::EchoState};

#[derive(Properties, PartialEq)]
pub struct TransportParamsProps {
    pub echo_state: UseStateHandle<EchoState>,
}

// Setea los parámetros de transporte: Embarcación, velocidad, mareógrafo, sensor, perfilador.
#[function_component(TransportParams)]
pub fn transport_params(props: &TransportParamsProps) -> Html {
    let state = props.echo_state.clone();
    let input_cls = "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white w-full";

    let render_check = |label: &'static str, value: bool, id: &'static str| {
        let state = state.clone();
        html! {
            <label class="flex items-center gap-2 cursor-pointer dark:text-white hover:text-cyan-200 transition-colors text-sm">
                <input type="checkbox" class="w-4 h-4" checked={value} onchange={Callback::from(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    let mut s = (*state).clone();
                    match id { 
                        "m" => s.uses_mareograph = input.checked(), 
                        "s" => s.uses_sound_profiler = input.checked(), 
                        "i" => s.uses_inertial_sensor = input.checked(),
                        _ => ()
                    };
                    state.set(s);
                })} /> {label}
            </label>
        }
    };

    html! {
        <div class="border-white/25 p-3 pt-0 border-b flex flex-col gap-3">
            <Subtitle text={"2. Embarcación"} icon={html! { <Ship size={18} /> }} />

            <div class="flex gap-2 p-1 bg-zinc-700 rounded border border-white/15">
                {for vec![
                    (Transport::Ship, "BARCO"),
                    (Transport::Boat, "BOTE"),
                    (Transport::Launch, "LANCHA")
                ].into_iter().map(|(t, label)| {
                    let state = state.clone();
                    let is_selected = state.transport == t;
                    html! {
                        <button 
                            type="button"
                            class={format!("flex-1 p-2 text-[10px] font-bold rounded transition-colors {}", 
                                if is_selected { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" }
                            )}
                            onclick={Callback::from(move |_| {
                                let mut s = (*state).clone();
                                s.transport = t;
                                state.set(s);
                            })}
                        >
                            {label}
                        </button>
                    }
                })}
            </div>

            <div class="flex flex-col gap-1">
                <span class="text-xs text-white/40 ml-1">{"Velocidad de la embarcación (m/s)"}</span>
                <input 
                    type="number" 
                    step="0.1"
                    placeholder="1.0"
                    class={input_cls} 
                    value={state.speed.clone()}
                    oninput={Callback::from({let state = state.clone(); move |e: InputEvent| {
                        let mut s = (*state).clone();
                        s.speed = e.target_unchecked_into::<HtmlInputElement>().value();
                        state.set(s);
                    }})}
                />
            </div>

            <div class="grid grid-cols-1 gap-1 mt-2">
                {render_check("Uso de mareógrafo", state.uses_mareograph, "m")}
                {render_check("Uso de perfilador de sonido", state.uses_sound_profiler, "s")}
                {render_check("Uso de sensor inercial", state.uses_inertial_sensor, "i")}
            </div>
        </div>
    }
}