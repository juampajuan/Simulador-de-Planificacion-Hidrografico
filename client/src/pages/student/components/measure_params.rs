use yew::prelude::*;
use crate::services::requests::{run_simulation, run_coverage};
use crate::structs::state::{EchoState, PathState, SimulationUiState};
use crate::pages::student::components::transport::TransportParams;
use crate::pages::student::components::echosounder::EchosounderParams;
use crate::structs::limits::ConfigLimits;
use crate::components::subtitle::Subtitle;
use lucide_yew::Play;

#[derive(Clone, PartialEq, Default)]
pub struct AttemptsState {
    pub spent: i64,
    pub limit: i64,
}

#[derive(Properties, PartialEq)]
pub struct MeasuresProps {
    pub path_state: UseStateHandle<PathState>,
    pub ui_state: SimulationUiState,
    pub limits: UseStateHandle<ConfigLimits>,
    pub attempts: UseStateHandle<AttemptsState>,
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

    let is_limit_reached = props.attempts.limit != -1 && props.attempts.spent >= props.attempts.limit;

    // El botón se deshabilita si el formulario está incompleto, si está cargando, O si llegó al límite
    let is_simulation_disabled = !is_form_complete || is_loading || is_limit_reached;

    let disabled_buttons = if is_loading {
        "pointer-events-none [&_input]:opacity-50 [&_button]:opacity-50"
    } else {
        ""
    };

    let echo_state_handle = state.clone();
    let path_state_handle = props.path_state.clone();
    let ui_state_handle = props.ui_state.clone();
    let limits_handle = props.limits.clone();
    
    let attempts_handle = props.attempts.clone(); 

    let on_simulate_click = Callback::from(move |_| {
        run_simulation(
            &*echo_state_handle, 
            &*path_state_handle, 
            ui_state_handle.clone(),
            &*limits_handle,
            attempts_handle.clone()
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
        <div class={classes!("relative", disabled_buttons, "flex", "flex-col", "gap-3")}>
            
            <TransportParams echo_state={state.clone()} />
 
            <EchosounderParams echo_state={state} />

            <div class="p-4 pt-0 flex flex-col gap-3">
                <Subtitle text={"4. Simulación"} icon={html! { <Play size={18} /> }} />

                <div class="flex flex-col gap-2">
                    <p class="text-xs text-center text-zinc-400 mt-0.5">
                            {"Intentos gastados: "}
                            <span class="text-white font-bold">{props.attempts.spent}</span>
                            {" / "}
                            <span class="text-white font-bold">
                                {
                                    if props.attempts.limit == -1 {
                                        "∞".to_string()
                                    } else {
                                        props.attempts.limit.to_string()
                                    }
                                }
                            </span>
                        </p>
                    <button 
                        onclick={on_coverage_click}
                        class="uppercase text-center disabled:opacity-30 bg-zinc-600 p-3 text-white font-bold w-full hover:bg-zinc-500 transition-all rounded shadow-xl disabled:bg-zinc-700 text-sm"
                    >
                        {"Ver Cobertura"}
                    </button>
                    <button 
                        disabled={is_simulation_disabled}
                        onclick={on_simulate_click}
                        class={
                            if is_limit_reached {
                                "uppercase text-center bg-zinc-800 p-3 text-zinc-500 font-bold w-full cursor-not-allowed rounded shadow-xl border border-zinc-700 text-sm"
                            } else {
                                "uppercase text-center disabled:opacity-30 bg-cyan-200 p-3 text-black font-bold w-full hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100 text-sm"
                            }
                        }
                    >
                        {
                            if is_limit_reached {
                                "Límite de intentos alcanzado"
                            } else {
                                "Simular MEDICIÓN"
                            }
                        }
                    </button>
                </div>
            </div>
        </div>
    }
}