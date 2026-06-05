use yew::prelude::*;
use web_sys::HtmlInputElement;
use common::EcosondaMode;
use common::Transport;
use crate::{components::subtitle::Subtitle, requests::{EchoState,PathState, run_simulation}};
use lucide_yew::{Radio, Ship};

#[derive(Properties, PartialEq)]
pub struct MeasuresProps {
    pub mensaje: UseStateHandle<String>,
    pub image_url: UseStateHandle<Option<String>>,
    pub loading: UseStateHandle<bool>,
    pub path_state: UseStateHandle<PathState>,
}

#[function_component(MeasuresParams)]
pub fn measures_params(props: &MeasuresProps) -> Html {
    let state = use_state(EchoState::new);
    let mensaje = props.mensaje.clone();
    let image_url = props.image_url.clone();
    let loading = props.loading.clone();
    let path_state = props.path_state.clone();
    
    let input_cls = "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white";

    let is_form_complete = 
        [
            &state.min_limit, &state.max_limit, &state.pulse_repetition_interval, 
            &state.pulse_length, &state.transmited_potency, &state.gain, 
            &state.echosounder_velocity, &state.umbral
        ].iter().all(|v| !v.trim().is_empty());

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
                        "f" => s.uses_high_frecuency = input.checked(),
                        _ => ()
                    };
                    state.set(s);
                })} /> {label}
            </label>
        }
    };

    let disabled_buttons = if *loading {
        "pointer-events-none [&_input]:opacity-50 [&_button]:opacity-50"
    } else {
        ""
    };

    html! {
        <div class={classes!("space-y-3", disabled_buttons)}>
            <div class="border-white/15 p-3 bg-zinc-900 rounded-md border flex flex-col gap-3">
                <Subtitle text={"2. Embarcación"} 
                    icon={html! {
                        <Ship size={18} />
                    }}
                />
 
                <div class="flex gap-2 p-1 bg-zinc-700 rounded border border-white/15">
                    <button 
                        type="button"
                        class={format!("flex-1 p-2 text-[10px] font-bold rounded transition-colors {}", 
                            if state.transport == Transport::Ship { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" }
                        )}
                        onclick={Callback::from({let state = state.clone(); move |_| {
                            let mut s = (*state).clone();
                            s.transport = Transport::Ship;
                            state.set(s);
                        }})}
                    >
                        {"BARCO"}
                    </button>
                    
                    <button 
                        type="button"
                        class={format!("flex-1 p-2 text-[10px] font-bold rounded transition-colors {}", 
                            if state.transport == Transport::Boat { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" }
                        )}
                        onclick={Callback::from({let state = state.clone(); move |_| {
                            let mut s = (*state).clone();
                            s.transport = Transport::Boat;
                            state.set(s);
                        }})}
                    >
                        {"BOTE"}
                    </button>

                    <button 
                        type="button"
                        class={format!("flex-1 p-2 text-[10px] font-bold rounded transition-colors {}", 
                            if state.transport == Transport::Launch { "bg-cyan-200 text-black" } else { "text-white hover:bg-zinc-600" }
                        )}
                        onclick={Callback::from({let state = state.clone(); move |_| {
                            let mut s = (*state).clone();
                            s.transport = Transport::Launch;
                            state.set(s);
                        }})}
                    >
                        {"LANCHA"}
                    </button>
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

            <div class="border-white/15 p-3 bg-zinc-900 rounded-md border flex flex-col gap-3">
                <Subtitle text={"3. Ecosonda"} 
                    icon={html! {
                        <Radio size={18} />
                    }}
                />
                 
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

            <div class="w-full mt-auto">
                <button 
                    disabled={!is_form_complete || *props.loading} 
                    onclick={Callback::from({
                        let state = state.clone();
                        let mensaje = mensaje.clone();
                        let image_url = image_url.clone();
                        let loading = loading.clone();
                        let path_state = path_state.clone();
                        move |_| run_simulation((*state).clone(), (*path_state).clone(), mensaje.clone(), image_url.clone(), loading.clone())
                    })} 
                    class="uppercase text-center disabled:opacity-30 bg-cyan-200 p-3 text-black font-bold w-full hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100"
                >
                    {"Simular MEDICIÓN"}
                </button>
            </div>
        </div>
    }
}