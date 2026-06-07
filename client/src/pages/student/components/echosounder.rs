use yew::prelude::*;
use web_sys::HtmlInputElement;
use common::EcosondaMode;
use lucide_yew::Radio;
use crate::{components::subtitle::Subtitle, requests::EchoState};

#[derive(Properties, PartialEq)]
pub struct EchosounderProps {
    pub echo_state: UseStateHandle<EchoState>,
}

#[function_component(EchosounderParams)]
pub fn echosounder_params(props: &EchosounderProps) -> Html {
    let state = props.echo_state.clone();
    let input_cls = "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white w-full";

    html! {
        <div class="border-white/15 p-3 bg-zinc-900 rounded-md border flex flex-col gap-3">
            <Subtitle text={"3. Ecosonda"} icon={html! { <Radio size={18} /> }} />
            <div class="flex gap-2 p-1 bg-zinc-700 rounded border border-white/15">
                <button 
                    type="button"
                    class={format!("flex-1 p-2 text-[10px] font-bold rounded transition-colors {}", 
                        if state.mode == EcosondaMode::Monohaz { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" }
                    )}
                    onclick={Callback::from({let state = state.clone(); move |_| {
                        let mut s = (*state).clone();
                        s.mode = EcosondaMode::Monohaz;
                        state.set(s);
                    }})}
                >
                    {"MONOHAZ"}
                </button>
                <button 
                    type="button"
                    class={format!("flex-1 p-2 text-[10px] font-bold rounded transition-colors {}", 
                        if state.mode == EcosondaMode::Multihaz { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" }
                    )}
                    onclick={Callback::from({let state = state.clone(); move |_| {
                        let mut s = (*state).clone();
                        s.mode = EcosondaMode::Multihaz;
                        state.set(s);
                    }})}
                >
                    {"MULTIHAZ"}
                </button>
            </div>
            <div class="flex gap-2 p-1 bg-zinc-700 rounded border border-white/15">
                <button 
                    type="button"
                    class={format!("flex-1 p-2 text-[10px] font-bold rounded transition-colors {}", 
                        if state.uses_high_frecuency { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" }
                    )}
                    onclick={Callback::from({let state = state.clone(); move |_| {
                        let mut s = (*state).clone();
                        s.uses_high_frecuency = true;
                        state.set(s);
                    }})}
                >
                    {"FRECUENCIA ALTA"}
                </button>
                <button 
                    type="button"
                    class={format!("flex-1 p-2 text-[10px] font-bold rounded transition-colors {}", 
                        if !state.uses_high_frecuency { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" }
                    )}
                    onclick={Callback::from({let state = state.clone(); move |_| {
                        let mut s = (*state).clone();
                        s.uses_high_frecuency = false;
                        state.set(s);
                    }})}
                >
                    {"FRECUENCIA BAJA"}
                </button>
            </div>

            <div class="grid grid-cols-2 gap-3">
                {for vec![
                    ("min_limit", "P. Mínima"), ("max_limit", "P. Máxima"), 
                    ("intervalo", "Intervalo Pulso"), ("velocidad", "V. Sonido"), 
                    ("longitud", "Longitud Pulso"), ("potencia", "Potencia"), 
                    ("ganancia", "Ganancia"), ("umbral", "Umbral")
                ].into_iter().map(|(id, l)| {
                    let state = state.clone();
                    let current_val = match id {
                        "min_limit" => state.min_limit.clone(), 
                        "max_limit" => state.max_limit.clone(),
                        "intervalo" => state.pulse_repetition_interval.clone(), 
                        "velocidad" => state.echosounder_velocity.clone(), 
                        "longitud" => state.pulse_length.clone(), 
                        "potencia" => state.transmited_potency.clone(), 
                        "ganancia" => state.gain.clone(), 
                        _ => state.umbral.clone(),
                    };
                    html! { 
                        <div class="flex flex-col gap-1">
                            <span class="text-xs text-white/40 ml-1">{l}</span>
                            <input type="number" class={input_cls} value={current_val}
                                oninput={Callback::from(move |e: InputEvent| { 
                                    let mut s = (*state).clone(); 
                                    let v = e.target_unchecked_into::<HtmlInputElement>().value();
                                    match id { 
                                        "min_limit" => s.min_limit = v, 
                                        "max_limit" => s.max_limit = v, 
                                        "intervalo" => s.pulse_repetition_interval = v, 
                                        "velocidad" => s.echosounder_velocity = v, 
                                        "longitud" => s.pulse_length = v, 
                                        "potencia" => s.transmited_potency = v, 
                                        "ganancia" => s.gain = v, 
                                        _ => s.umbral = v 
                                    };
                                    state.set(s);
                                })} 
                            /> 
                        </div>
                    }
                })}
            </div>
        </div>
    }
}