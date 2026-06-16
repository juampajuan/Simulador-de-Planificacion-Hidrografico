use yew::prelude::*;
use web_sys::HtmlInputElement;
use common::EcosondaMode;
use lucide_yew::Radio;
use crate::{components::subtitle::Subtitle, structs::state::EchoState};

#[derive(Properties, PartialEq)]
pub struct EchosounderProps {
    pub echo_state: UseStateHandle<EchoState>
}

#[function_component(EchosounderParams)]
pub fn echosounder_params(props: &EchosounderProps) -> Html {
    let state = props.echo_state.clone();
    let input_cls = "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white w-full";
    let btn_pill_base = "flex-1 p-2 text-[10px] font-bold rounded transition-colors";
    let profiler_in_use: bool = state.uses_sound_profiler;

    html! {
        <div class="border-white/25 p-3 pt-0 border-b flex flex-col gap-3">
            <Subtitle text={"3. Ecosonda"} icon={html! { <Radio size={18} /> }} />
            <div class="flex gap-2 p-1 bg-zinc-700 rounded border border-white/15">
                <button 
                    type="button"
                    class={format!("{} {}", btn_pill_base, if state.mode == EcosondaMode::Monohaz { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" })}
                    onclick={Callback::from({let state = state.clone(); move |_| {
                        let mut s = (*state).clone(); s.mode = EcosondaMode::Monohaz; state.set(s);
                    }})}
                >
                    {"MONOHAZ"}
                </button>
                <button 
                    type="button"
                    class={format!("{} {}", btn_pill_base, if state.mode == EcosondaMode::Multihaz { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" })}
                    onclick={Callback::from({let state = state.clone(); move |_| {
                        let mut s = (*state).clone(); s.mode = EcosondaMode::Multihaz; state.set(s);
                    }})}
                >
                    {"MULTIHAZ"}
                </button>
            </div>
            <div class="flex gap-2 p-1 bg-zinc-700 rounded border border-white/15">
                <button 
                    type="button"
                    class={format!("{} {}", btn_pill_base, if state.uses_high_frecuency { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" })}
                    onclick={Callback::from({let state = state.clone(); move |_| {
                        let mut s = (*state).clone(); s.uses_high_frecuency = true; state.set(s);
                    }})}
                >
                    {"FRECUENCIA ALTA"}
                </button>
                <button 
                    type="button"
                    class={format!("{} {}", btn_pill_base, if !state.uses_high_frecuency { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" })}
                    onclick={Callback::from({let state = state.clone(); move |_| {
                        let mut s = (*state).clone(); s.uses_high_frecuency = false; state.set(s);
                    }})}
                >
                    {"FRECUENCIA BAJA"}
                </button>
            </div>

            <div class="grid grid-cols-2 gap-3">
                {for vec![
                    ("min_limit", "Profundidad Min. (mts)"), ("max_limit", "Profundidad Máx. (mts)"), 
                    ("intervalo", "Intervalo de repetición del pulso (Hz)"), ("velocidad", "Velocidad del sonido (m/s)"), 
                    ("umbral", "Umbral (%)")
                ].into_iter().map(|(id, l)| {
                    let state = state.clone();
                    let current_val = match id {
                        "min_limit" => state.min_limit.clone(), 
                        "max_limit" => state.max_limit.clone(),
                        "intervalo" => state.pulse_repetition_interval.clone(), 
                        "velocidad" => state.sound_speed.clone(),
                        _ => state.umbral.clone(),
                    };
                    let profiler_is_disabled = id == "velocidad" && profiler_in_use;
                    
                    let container_cls = if id == "intervalo" { "flex flex-col gap-1 col-span-2" } else { "flex flex-col gap-1" };
                    let dynamic_input_cls = if profiler_is_disabled { 
                        format!("{} opacity-40 cursor-not-allowed select-none bg-zinc-800", input_cls) 
                    } else { 
                        input_cls.to_string() 
                    };
                    html! { 
                        <div class={container_cls}>
                            <span class="text-xs text-white/40 ml-1">
                                {l}
                            </span>
                            <input 
                                type="number" 
                                class={dynamic_input_cls} 
                                value={current_val}
                                disabled={profiler_is_disabled} 
                                oninput={Callback::from(move |e: InputEvent| { 
                                    let mut s = (*state).clone(); 
                                    let v = e.target_unchecked_into::<HtmlInputElement>().value();
                                    match id { 
                                        "min_limit" => s.min_limit = v, 
                                        "max_limit" => s.max_limit = v, 
                                        "intervalo" => s.pulse_repetition_interval = v, 
                                        "velocidad" => s.sound_speed = v, 
                                        _ => s.umbral = v 
                                    };
                                    state.set(s);
                                })} 
                            /> 
                        </div>
                    }
                })}
            </div>
            {if profiler_in_use {
                html! {
                    <div class="text-[11px] text-amber-400 bg-amber-500/10 border border-amber-500/20 rounded p-2 text-center mt-1">
                        {"Campo 'Velocidad del sonido' bloqueado automáticamente por uso de perfilador."}
                    </div>
                }
            } else {
                html! {}
            }}
            <div class="flex flex-col gap-1">
                <span class="text-xs text-white/40 ml-1">{"Potencia"}</span>
                <div class="flex gap-2 p-1 bg-zinc-700 rounded border border-white/15">
                    {for vec![("25", "BAJA (25%)"), ("50", "MEDIA (50%)"), ("100", "ALTA (100%)")].into_iter().map(|(val, label)| {
                        let state = state.clone();
                        html! {
                            <button type="button"
                                class={format!("{} {}", btn_pill_base, if state.transmited_potency == val { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" })}
                                onclick={Callback::from(move |_| {
                                    let mut s = (*state).clone(); s.transmited_potency = val.to_string(); state.set(s);
                                })}
                            >
                                {label}
                            </button>
                        }
                    })}
                </div>
            </div>

            <div class="flex flex-col gap-1">
                <span class="text-xs text-white/40 ml-1">{"Ganancia"}</span>
                <div class="flex gap-2 p-1 bg-zinc-700 rounded border border-white/15">
                    {for vec!["12", "24", "36"].into_iter().map(|val| {
                        let state = state.clone();
                        html! {
                            <button type="button"
                                class={format!("{} {}", btn_pill_base, if state.gain == val { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" })}
                                onclick={Callback::from(move |_| {
                                    let mut s = (*state).clone(); s.gain = val.to_string(); state.set(s);
                                })}
                            >
                                {format!("{} dB", val)}
                            </button>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}