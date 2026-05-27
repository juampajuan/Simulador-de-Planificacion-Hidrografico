use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::requests::{run_simulation, EchoState};

#[derive(Properties, PartialEq)]
pub struct MeasuresProps {
    pub mensaje: UseStateHandle<String>,
    pub image_url: UseStateHandle<Option<String>>,
}

#[function_component(MeasuresParams)]
pub fn measures_params(props: &MeasuresProps) -> Html {
    let state = use_state(EchoState::default);
    let mensaje = props.mensaje.clone();
    let image_url = props.image_url.clone();
    
    let input_cls = "rounded p-2 text-black dark:bg-zinc-700 dark:text-white w-full";

    let is_form_complete = !state.boat.trim().is_empty() && 
        [
            &state.min_limit, &state.max_limit, &state.pulse_repetition_interval, 
            &state.pulse_length, &state.transmited_potency, &state.gain, 
            &state.echosounder_velocity, &state.umbral
        ].iter().all(|v| !v.trim().is_empty());

    let render_check = |label: &'static str, value: bool, id: &'static str| {
        let state = state.clone();
        html! {
            <label class="flex items-center gap-2 cursor-pointer hover:text-cyan-200 transition-colors">
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
            <div class="border-b border-dashed border-white/40 p-3 flex flex-col gap-2">
                <label class="font-semibold">{"Embarcación"}</label>
                <input type="text" placeholder="Seleccione la embarcación (W o Y)" class={input_cls} 
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

            <div class="p-3">
                <h3 class="text-2xl font-bold mb-4 text-white">{"Parámetros de Ecosonda"}</h3>
                
                <div class="mb-4 p-2 bg-white/5 rounded border border-white/10">
                    {render_check("Frecuencia Alta", state.uses_high_frecuency, "f")}
                </div>

                <div class="flex flex-col gap-4">
                    {for vec![
                        ("min_limit", "Profundidad mínima"), 
                        ("max_limit", "Profundidad máxima"), 
                        ("intervalo", "Intervalo de repetición"), 
                        ("velocidad", "Velocidad del sonido"), 
                        ("longitud", "Longitud del pulso"), 
                        ("potencia", "Potencia transmitida"), 
                        ("ganancia", "Ganancia"), 
                        ("umbral", "Umbral de detección")
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
                            <input type="number" placeholder={l} class={input_cls} value={current_val}
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
                        }
                    })}
                </div>
            </div>

            <div class="w-full mt-auto sticky bottom-0">
                <button 
                    disabled={!is_form_complete} 
                    onclick={Callback::from(move |_| run_simulation((*state).clone(), mensaje.clone(), image_url.clone()))} 
                    class="text-center disabled:opacity-30 bg-cyan-200 p-2 text-black font-semibold w-full hover:bg-cyan-300 transition-colors"
                >
                    {"Realizar Medición"}
                </button>
            </div>
        </>
    }
}