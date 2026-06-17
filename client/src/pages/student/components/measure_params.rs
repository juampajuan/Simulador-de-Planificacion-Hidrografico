use yew::prelude::*;
use crate::services::requests::{run_simulation, run_coverage};
use crate::structs::state::{EchoState, PathState, SimulationUiState};
use crate::pages::student::components::transport::TransportParams;
use crate::pages::student::components::echosounder::EchosounderParams;
use crate::structs::limits::ConfigLimits;

#[derive(Properties, PartialEq)]
pub struct MeasuresProps {
    pub path_state: UseStateHandle<PathState>,
    pub ui_state: SimulationUiState,
    pub limits: UseStateHandle<ConfigLimits>,
}

#[function_component(MeasuresParams)]
pub fn measures_params(props: &MeasuresProps) -> Html {
    let state = use_state(EchoState::new);
    let is_form_complete = [
        &state.min_limit, &state.max_limit, &state.pulse_repetition_interval, 
        &state.transmited_potency, &state.gain, 
        &state.sound_speed, &state.umbral
    ].iter().all(|v| !v.trim().is_empty());

    let is_loading = *props.ui_state.loading;

    let disabled_buttons = if is_loading {
        "pointer-events-none [&_input]:opacity-50 [&_button]:opacity-50"
    } else {
        ""
    };

    let echo_state_handle = state.clone();
    let path_state_handle = props.path_state.clone();
    let ui_state_handle = props.ui_state.clone();
    let limits_handle = props.limits.clone();

    let on_simulate_click = Callback::from(move |_| {
        run_simulation(
            &*echo_state_handle, 
            &*path_state_handle, 
            ui_state_handle.clone(),
            &*limits_handle
        );
    });

    let echo_state_handle_cov = state.clone();
    let path_state_handle_cov = props.path_state.clone();
    let ui_state_handle_cov = props.ui_state.clone();
    let limits_handle_cov = props.limits.clone();

    let on_coverage_click = Callback::from(move |_| {
        run_coverage(
            &*echo_state_handle_cov,
            &*path_state_handle_cov,
            ui_state_handle_cov.clone(),
            &*limits_handle_cov
        );
    });


    html! {
        <div class={classes!("space-y-3", "relative", disabled_buttons)}>
            
            <TransportParams echo_state={state.clone()} />
 
            <EchosounderParams echo_state={state} />
 
            <div class="pb-1 px-3 flex flex-col gap-2">
                <button 
                    disabled={!is_form_complete || is_loading} 
                    onclick={on_coverage_click}
                    class="uppercase text-center disabled:opacity-30 bg-zinc-600 p-3 text-white font-bold w-full hover:bg-zinc-500 transition-all rounded shadow-xl disabled:bg-zinc-700"
                >
                    {"Ver Cobertura"}
                </button>
                <button 
                    disabled={!is_form_complete || is_loading} 
                    onclick={on_simulate_click}
                    class="uppercase text-center disabled:opacity-30 bg-cyan-200 p-3 text-black font-bold w-full hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100"
                >
                    {"Simular MEDICIÓN"}
                </button>
            </div>
        </div>
    }
}