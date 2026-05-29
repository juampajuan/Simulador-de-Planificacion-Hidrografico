use yew::prelude::*;
use web_sys::HtmlInputElement;
use common::EcosondaMode;
use crate::{components::subtitle::Subtitle, requests::{EchoState, run_simulation}};
use lucide_yew::{Radio, Ship};

#[derive(Properties, PartialEq)]
pub struct MeasuresProps {
    pub mensaje: UseStateHandle<String>,
    pub image_url: UseStateHandle<Option<String>>,
}

#[function_component(MeasuresParams)]
pub fn measures_params(props: &MeasuresProps) -> Html {
    let state = use_state(EchoState::new);
    let mensaje = props.mensaje.clone();
    let image_url = props.image_url.clone();
    
    let input_cls = "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white";

    let is_form_complete = !state.boat.trim().is_empty() && 
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
                        "m" => s.uses_mathegapher = input.checked(), 
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

    html! {
        <>
            <div class="border-white/15 p-3 bg-zinc-900 rounded-md border flex flex-col gap-3">
                <Subtitle text={"2. Embarcación"} 
                    icon={html! {
                        <Ship size={18} />
                    }}
                />
 
                <input type="text" placeholder="W o Y" class={input_cls} 
                    value={state.boat.clone()}
                    oninput={Callback::from({let state = state.clone(); move |e: InputEvent| {
                        let mut s = (*state).clone();
                        s.boat = e.target_unchecked_into::<HtmlInputElement>().value();
                        state.set(s);
                    }})}
                />
                <div class="grid grid-cols-1 gap-1 mt-2">
                    {render_check("Uso de monógrafo", state.uses_mathegapher, "m")}
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
                        class={format!("flex-1 p-2 text-[10px] font-bold rounded {}", if state.mode == EcosondaMode::Monohaz { "bg-cyan-200 text-black" } else { "text-white" })}
                        onclick={Callback::from({let state = state.clone(); move |_| {
                            let mut s = (*state).clone();
                            s.mode = EcosondaMode::Monohaz; // Simple y prolijo
                            state.set(s);
                        }})}
                    >{"MONOHAZ"}</button>
                    
                    <button 
                        class={format!("flex-1 p-2 text-[10px] font-bold rounded {}", if state.mode == EcosondaMode::Multihaz { "bg-cyan-200 text-black" } else { "text-white" })}
                        onclick={Callback::from({let state = state.clone(); move |_| {
                            let mut s = (*state).clone();
                            s.mode = EcosondaMode::Multihaz;
                            state.set(s);
                        }})}
                    >{"MULTIHAZ"}</button>
                </div>
               
                <div class="p-2 bg-zinc-700 rounded border border-white/15">
                    <div class="flex justify-between items-center">
                        <span class="text-sm font-medium text-white">{"Frecuencia de Trabajo"}</span>
                        {render_check(if state.uses_high_frecuency { "ALTA" } else { "BAJA" }, state.uses_high_frecuency, "f")}
                    </div>
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
                    disabled={!is_form_complete} 
                    onclick={Callback::from({
                        let state = state.clone();
                        let mensaje = mensaje.clone();
                        let image_url = image_url.clone();
                        move |_| run_simulation((*state).clone(), mensaje.clone(), image_url.clone())
                    })} 
                    class="uppercase text-center disabled:opacity-30 bg-cyan-200 p-3 text-black font-bold w-full hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100"
                >
                    {"Simular MEDICIÓN"}
                </button>
            </div>
        </>
    }
}