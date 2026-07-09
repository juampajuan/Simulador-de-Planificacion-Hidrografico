use yew::prelude::*;
use crate::services::requests::{run_simulation, run_coverage, StudentSimulation};
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
    pub active_layers_sim: UseStateHandle<Option<StudentSimulation>>,
    pub history_state: UseStateHandle<Vec<StudentSimulation>>,
}

// Muestra los parámetros de medición
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

    {
        let map_base64_state = props.ui_state.map_base64.clone();
        let active_layers_sim = props.active_layers_sim.clone();
        let min_depth = props.ui_state.min_depth.clone();
        let max_depth = props.ui_state.max_depth.clone();
        let history = props.history_state.clone(); // Necesitamos pasarle el historial o generar el ID aproximado

        use_effect_with(map_base64_state.clone(), move |m_b64| {
            if let Some(b64_str) = &**m_b64 {
                if !b64_str.is_empty() {
                    // Calculamos el número de intento basado en el historial existente + 1
                    let next_attempt = (history.len() + 1) as i64;
                    
                    // Reconstruimos las rutas exactas que el backend crea físicamente en disco
                    let live_sim = StudentSimulation {
                        id: 0, 
                        attempt_number: next_attempt,
                        selected: false,
                        result_min_depth: *min_depth,
                        result_max_depth: *max_depth,
                        student_id: 0,
                        project_id: 0,
                        // Asignamos los nombres de archivo estándar que el backend genera en /images/
                        simulation_image_path: Some(format!("intento_{}_simulacion.png", next_attempt)), 
                        coverage_image_path: Some(format!("intento_{}_cobertura.png", next_attempt)), 
                        difference_image_path: Some(format!("intento_{}_diferencias.png", next_attempt)),
                        path_parameters: Default::default(),
                        transport_parameters: Default::default(),
                        echosounder_parameters: Default::default(),
                    };
                    
                    active_layers_sim.set(Some(live_sim));
                }
            }
            || ()
        });
    }

    let echo_state_handle = state.clone();
    let path_state_handle = props.path_state.clone();
    let ui_state_handle = props.ui_state.clone();
    let limits_handle = props.limits.clone();
    let attempts_handle = props.attempts.clone(); 
    let active_layers_sim_btn = props.active_layers_sim.clone();

    let on_simulate_click = Callback::from(move |_| {
        active_layers_sim_btn.set(None);
        run_simulation(
            &echo_state_handle, 
            &path_state_handle, 
            ui_state_handle.clone(),
            &limits_handle,
            attempts_handle.clone()
        );
    });

    let echo_state_handle_cov = state.clone();
    let path_state_handle_cov = props.path_state.clone();
    let ui_state_handle_cov = props.ui_state.clone();
    let limits_handle_cov = props.limits.clone();
    let active_layers_sim_cov = props.active_layers_sim.clone();

    let on_coverage_click = Callback::from(move |_| {
        active_layers_sim_cov.set(None);
        run_coverage(
            &echo_state_handle_cov,
            &path_state_handle_cov,
            ui_state_handle_cov.clone(),
            &limits_handle_cov
        );
    });

    html! {
        <div class={classes!("relative", disabled_buttons, "flex", "flex-col", "gap-3")}>
            
            <TransportParams echo_state={state.clone()} />
 
            <EchosounderParams echo_state={state} />

            <div class="p-3 pt-0 flex flex-col gap-3">
                <Subtitle text={"4. Simulación"} icon={html! { <Play size={18} /> }} />

                <div class="flex flex-col gap-2">
                    <div class="flex justify-between">
                        <p class="text-xs text-center text-zinc-400 mt-0.5">
                            {"Intentos utilizados"}
                        </p>
                        <p class="text-xs text-center text-zinc-400 mt-0.5">
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
                    </div>
                    <button 
                        onclick={on_coverage_click}
                        class="uppercase text-center disabled:opacity-30 bg-zinc-600 p-3 text-white font-bold w-full hover:bg-zinc-500 transition-all rounded shadow-xl disabled:bg-zinc-700 text-sm cursor-pointer"
                    >
                        {"Ver Cobertura"}
                    </button>
                    <button 
                        disabled={is_simulation_disabled}
                        onclick={on_simulate_click}
                        class={
                            if is_limit_reached {
                                "uppercase text-center bg-zinc-800 p-3 text-zinc-500 font-bold w-full cursor-not-allowed rounded shadow-xl border border-zinc-700 text-sm select-none"
                            } else {
                                "uppercase text-center disabled:opacity-30 bg-cyan-200 p-3 text-black font-bold w-full hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100 text-sm cursor-pointer"
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