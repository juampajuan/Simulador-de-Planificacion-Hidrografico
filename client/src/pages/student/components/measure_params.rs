use yew::prelude::*;
use crate::{requests::{EchoState,PathState, run_simulation}};
use crate::pages::student::components::transport::TransportParams;
use crate::pages::student::components::echosounder::EchosounderParams;

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
    
    let is_form_complete = 
        [
            &state.min_limit, &state.max_limit, &state.pulse_repetition_interval, 
            &state.pulse_length, &state.transmited_potency, &state.gain, 
            &state.echosounder_velocity, &state.umbral
        ].iter().all(|v| !v.trim().is_empty());

    let disabled_buttons = if *loading {
        "pointer-events-none [&_input]:opacity-50 [&_button]:opacity-50"
    } else {
        ""
    };

    html! {
        <div class={classes!("space-y-3", disabled_buttons)}>
            
            <TransportParams echo_state={state.clone()} />

            <EchosounderParams echo_state={state.clone()} />

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