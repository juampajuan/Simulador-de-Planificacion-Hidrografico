use crate::components::subtitle::Subtitle;
use crate::services::requests::select_exam_delivery;
use crate::structs::state::SimulationUiState;
use common::EcosondaMode;
use common::GnssType;
use common::StudentSimulation;
use lucide_yew::{CalendarX, Check, ChevronDown, ChevronUp, Circle, History};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryProps {
    pub history_state: UseStateHandle<Vec<StudentSimulation>>,
    pub ui_mensaje: UseStateHandle<String>,
    pub exam_mode: bool,
    pub due_date: Option<String>,
    pub ui_state: SimulationUiState,
    pub active_layers_sim: UseStateHandle<Option<StudentSimulation>>,
}

/// Pestaña "Historial": Muestra los intentos de simulación del estudiante utilizando la persistencia real de la DB.
#[function_component(HistoryParams)]
pub fn history_params(props: &HistoryProps) -> Html {
    let input_cls =
        "rounded p-2 text-sm bg-zinc-700/50 text-white border border-white/5 w-full font-mono";
    let label_cls = "text-xs font-semibold text-white/40 ml-1";

    // Mantiene un registro de qué ID de intento está expandido (-1 significa ninguno)
    let expanded_id = use_state(|| -1i64);

    let today_str = {
        let date = js_sys::Date::new_0();
        let year = date.get_full_year();
        let month = date.get_month() + 1;
        let day = date.get_date();
        format!("{:04}-{:02}-{:02}", year, month, day)
    };

    html! {
        <div class="flex flex-col w-full text-white divide-y divide-white/10">
            { if props.history_state.is_empty() {
                html! {
                    <div class="text-center py-8 text-white/40 italic text-sm w-full">
                        {"Las simulaciones realizadas aparecerán aquí."}
                    </div>
                }
            } else {
                html! {
                    <>
                        { for (*props.history_state).iter().map(|sim| {
                            let sim_id = sim.id;
                            let is_expanded = *expanded_id == sim_id;
                            let is_selected = sim.selected;

                            let toggle_expand = {
                                let expanded_id = expanded_id.clone();
                                let active_layers_sim = props.active_layers_sim.clone();
                                let sim_clone = sim.clone();
                                Callback::from(move |_| {
                                    if *expanded_id == sim_id {
                                        expanded_id.set(-1);
                                        active_layers_sim.set(None);
                                    } else {
                                        expanded_id.set(sim_id);
                                        active_layers_sim.set(Some(sim_clone.clone()));
                                    }
                                })
                            };

                            let msg_for_this_btn = props.ui_mensaje.clone();
                            let history_state = props.history_state.clone();
                            let due_date_clone = props.due_date.clone();
                            let today_clone = today_str.clone();

                            let is_expired = if props.exam_mode {
                                if let Some(due) = &due_date_clone { today_clone > *due } else { false }
                            } else {
                                false // Si no es un examen/entrega, el plazo jamás puede vencer
                            };

                            let on_select_delivery = Callback::from(move |e: MouseEvent| {
                                e.stop_propagation();
                                if is_expired && !is_selected {
                                    msg_for_this_btn.set("Error: El plazo de entrega para este proyecto ha expirado.".to_string());
                                    return;
                                }

                                let mut current_list = (*history_state).clone();
                                let mut ya_estaba_entregado = false;
                                if let Some(actual) = current_list.iter().find(|item| item.id == sim_id) {
                                    ya_estaba_entregado = actual.selected;
                                }

                                // como estoy enviando none en el caso de que haya elegido para entregar uno que ya estaba entregado,
                                // el backend lo interpretará como "quitar la entrega" y no como "entregar otro" -> clear_all_selected
                                let id_para_servidor = if ya_estaba_entregado { None } else { Some(sim_id) };

                                for item in current_list.iter_mut() {
                                    if item.id == sim_id { item.selected = !ya_estaba_entregado; } else { item.selected = false; }
                                }
                                history_state.set(current_list);
                                select_exam_delivery(id_para_servidor, msg_for_this_btn.clone());
                            });

                            // Botón adaptado al estado de la entrega.
                            let btn_cls = if is_selected {
                                "flex items-center justify-center gap-2 w-full h-[28px] rounded text-xs font-bold transition-all bg-cyan-200 text-black hover:bg-cyan-300 shadow-lg shadow-cyan-500/10 border border-transparent shrink-0"
                            } else if is_expired {
                                "flex items-center justify-center gap-2 w-full h-[28px] rounded text-xs font-bold transition-all bg-zinc-800/30 text-red-400/40 border border-red-500/10 cursor-not-allowed shrink-0 select-none"
                            } else {
                                "flex items-center justify-center gap-2 w-full h-[28px] rounded text-xs font-bold transition-all bg-zinc-800 text-white/50 border border-white/5 hover:bg-zinc-700/50 hover:text-white shrink-0"
                            };

                            let gnss_str = match sim.data.path_parameters.gnss_type {
                                GnssType::NoCorrection => "Sin Corrección",
                                GnssType::DGPSCorrection => "Con corrección DGPS",
                                GnssType::PhaseCorrection => "Con correccion por fase",
                            };

                            let echosounder_mode_str = match sim.data.echosounder_parameters.mode {
                                EcosondaMode::Monohaz => "Monohaz",
                                EcosondaMode::Multihaz => "Multihaz",
                            };

                            html! {
                                <div key={sim.id} class="p-3 first:pt-1 flex flex-col gap-2.5">
                                    <div onclick={toggle_expand} class="flex flex-col gap-2 cursor-pointer select-none">
                                        <div class="flex items-center justify-between w-full">
                                            <div class="flex items-center shrink-0">
                                                <Subtitle text={format!("Intento #{}", sim.data.attempt_number)} icon={html! { <History size={16} /> }} />
                                            </div>
                                            <div class="flex items-center gap-2">
                                                {
                                                    if is_selected {
                                                        html!{ <div class="text-center uppercase text-lime-400 font-bold text-xs mr-1 select-none">{"Entregado"}</div> }
                                                    } else if is_expired {
                                                        html!{ <div class="text-center uppercase text-red-400/60 font-bold text-xs mr-1 select-none">{"Plazo Vencido"}</div> }
                                                    } else {
                                                        html!{
                                                            <div class="text-[11px] font-mono text-white/60 bg-zinc-800/40 px-2 py-0.5 rounded border border-white/5 flex gap-2 whitespace-nowrap">
                                                                <div>{"Mín: "}<span class="text-white font-semibold">{format!("{:.2}m", sim.data.result_min_depth)}</span></div>
                                                                <div class="text-white/25">{"|"}</div>
                                                                <div>{"Máx: "}<span class="text-white font-semibold">{format!("{:.2}m", sim.data.result_max_depth)}</span></div>
                                                            </div>
                                                        }
                                                    }
                                                }
                                                <span class="text-white/40 hover:text-white/70 transition-colors w-4 flex justify-end">
                                                    { if is_expanded { html!{ <ChevronUp size={16} /> } } else { html!{ <ChevronDown size={16} /> } } }
                                                </span>
                                            </div>
                                        </div>
                                    </div>

                                    { if is_expanded {
                                        html! {
                                            <div class="grid grid-cols-2 gap-3 mt-1 pb-1">
                                                <div class="col-span-2 text-xs font-bold text-white/40 uppercase tracking-wider mb-0.5">{"Parámetros de Recorrido"}</div>
                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Separación"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{} m", sim.data.path_parameters.separacion)} />
                                                </div>
                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Azimut"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{}°", sim.data.path_parameters.azimut)} />
                                                </div>

                                                <div class="flex flex-col gap-1 col-span-2">
                                                    <span class={label_cls}>{"GNSS"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={gnss_str} />
                                                </div>

                                                <div class="col-span-2 text-xs font-bold text-white/40 uppercase tracking-wider mt-2 mb-0.5">{"Parámetros de Transporte"}</div>
                                                <div class="flex flex-col gap-1 col-span-2">
                                                    <span class={label_cls}>{"Velocidad"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{} m/s", sim.data.transport_parameters.speed)} />
                                                </div>

                                                <div class="col-span-2 text-xs font-bold text-white/40 uppercase tracking-wider mt-1 mb-0.5">{"Sensores Activos"}</div>
                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Mareógrafo"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={if sim.data.transport_parameters.uses_mareograph { "Sí" } else { "No" }} />
                                                </div>
                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Perfilador de sonido"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={if sim.data.transport_parameters.uses_sound_profiler { "Sí" } else { "No" }} />
                                                </div>
                                                <div class="flex flex-col gap-1 col-span-2">
                                                    <span class={label_cls}>{"Sensor Inercial"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={if sim.data.transport_parameters.uses_inertial_sensor { "Sí" } else { "No" }} />
                                                </div>

                                                <div class="col-span-2 text-xs font-bold text-white/40 uppercase tracking-wider mt-2 mb-0.5">{"Configuración de Ecosonda"}</div>

                                                <div class="flex flex-col gap-1 col-span-2">
                                                    <span class={label_cls}>{"Tipo de Ecosonda"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={echosounder_mode_str} />
                                                </div>

                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Frecuencia"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={if sim.data.echosounder_parameters.uses_high_frecuency { "Alta" } else { "Baja" }} />
                                                </div>

                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Ganancia"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{} dB", sim.data.echosounder_parameters.gain)} />
                                                </div>

                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Umbral"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={sim.data.echosounder_parameters.threshold.to_string()} />
                                                </div>

                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Velocidad del Sonido"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{} m/s", sim.data.echosounder_parameters.sound_speed)} />
                                                </div>

                                                <div class="flex flex-col gap-1 col-span-2">
                                                    <span class={label_cls}>{"Límites (Mín/Máx)"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{}m a {}m", sim.data.echosounder_parameters.min_limit, sim.data.echosounder_parameters.max_limit)} />
                                                </div>

                                                <div class="col-span-2">
                                                    { if props.exam_mode {
                                                        html! {
                                                            <div class="w-full pt-1">
                                                                <button onclick={on_select_delivery} class={btn_cls} disabled={is_expired && !is_selected}>
                                                                    { if is_selected { html!{ <Check size={14} /> } } else if is_expired { html!{ <CalendarX size={14} /> } } else { html!{ <Circle size={14} /> } } }
                                                                    { if is_selected { "ENTREGADO" } else if is_expired { "PLAZO VENCIDO" } else { "ENTREGAR" } }
                                                                </button>
                                                            </div>
                                                        }
                                                    } else { html! {} } }
                                                </div>
                                            </div>
                                        }
                                    } else { html!{} } }
                                </div>
                            }
                        }) }
                    </>
                }
            } }
        </div>
    }
}
