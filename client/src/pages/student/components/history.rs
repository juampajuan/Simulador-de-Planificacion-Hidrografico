use yew::prelude::*;
use crate::services::requests::StudentSimulation;
use crate::components::subtitle::Subtitle;
use crate::services::requests::select_exam_delivery;
use lucide_yew::{History, ChevronDown, ChevronUp, Check, Circle};

#[derive(Properties, PartialEq)]
pub struct HistoryProps {
    pub history_state: UseStateHandle<Vec<StudentSimulation>>,
    pub ui_mensaje: UseStateHandle<String>,
    pub exam_mode: bool,
}

/// Pestaña "Historial": Muestra los intentos de simulación del estudiante utilizando la persistencia real de la DB.
#[function_component(HistoryParams)]
pub fn history_params(props: &HistoryProps) -> Html {
    let input_cls = "rounded p-2 text-sm bg-zinc-700/50 text-white border border-white/5 w-full font-mono";
    let label_cls = "text-xs font-semibold text-white/40 ml-1";

    // Mantiene un registro de qué ID de intento está expandido (-1 significa ninguno)
    let expanded_id = use_state(|| -1i64);

    html! {
        <div class="flex flex-col w-full text-white divide-y divide-white/10">
            { if props.history_state.is_empty() {
                html! {
                    <div class="text-center py-8 text-white/40 italic text-sm w-full">
                        {"Las simulaciones que realices aparecerán aquí."}
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
                                Callback::from(move |_| {
                                    if *expanded_id == sim_id {
                                        expanded_id.set(-1);
                                    } else {
                                        expanded_id.set(sim_id);
                                    }
                                })
                            };

                            let msg_for_this_btn = props.ui_mensaje.clone();
                            let history_state = props.history_state.clone();

                            let on_select_delivery = Callback::from(move |e: MouseEvent| {
                                e.stop_propagation(); 
                                
                                let mut current_list = (*history_state).clone();
                                let mut ya_estaba_entregado = false;
                                
                                if let Some(actual) = current_list.iter().find(|item| item.id == sim_id) {
                                    ya_estaba_entregado = actual.selected;
                                }

                                // como estoy enviando none en el caso de que haya elegido para entregar uno que ya estaba entregado, 
                                // el backend lo interpretará como "quitar la entrega" y no como "entregar otro" -> clear_all_selected
                                let id_para_servidor = if ya_estaba_entregado { None } else { Some(sim_id) };

                                for item in current_list.iter_mut() {
                                    if item.id == sim_id {
                                        item.selected = !ya_estaba_entregado;
                                    } else {
                                        item.selected = false;
                                    }
                                }
                                
                                history_state.set(current_list);
                                select_exam_delivery(id_para_servidor, msg_for_this_btn.clone());
                            });

                            // Botón adaptado al estado de la entrega.
                            let btn_cls = if is_selected {
                                "flex items-center justify-center gap-2 w-full h-[28px] rounded text-xs font-bold transition-all bg-cyan-200 text-black hover:bg-cyan-300 shadow-lg shadow-cyan-500/10 border border-transparent shrink-0"
                            } else {
                                "flex items-center justify-center gap-2 w-full h-[28px] rounded text-xs font-bold transition-all bg-zinc-800 text-white/50 border border-white/5 hover:bg-zinc-700/50 hover:text-white shrink-0"
                            };

                            html! {
                                <div key={sim.id} class="p-3 first:pt-1 flex flex-col gap-2.5">
                                    <div 
                                        onclick={toggle_expand}
                                        class="flex flex-col gap-2 cursor-pointer select-none"
                                    >
                                        <div class="flex items-center justify-between w-full">
                                            <div class="flex items-center shrink-0">
                                                <Subtitle
                                                    text={format!("Intento #{}", sim.attempt_number)}
                                                    icon={html! { <History size={16} /> }}
                                                />
                                            </div>

                                            <div class="flex items-center gap-2">
                                                {
                                                    if !is_selected {
                                                        html!{
                                                            <div class="text-[11px] font-mono text-white/60 bg-zinc-800/40 px-2 py-0.5 rounded border border-white/5 flex gap-2 whitespace-nowrap">
                                                                <div>{"Mín: "}<span class="text-white font-semibold">{format!("{:.2}m", sim.result_min_depth)}</span></div>
                                                                <div class="text-white/25">{"|"}</div>
                                                                <div>{"Máx: "}<span class="text-white font-semibold">{format!("{:.2}m", sim.result_max_depth)}</span></div>
                                                            </div>
                                                        }
                                                    } else {
                                                        html!{
                                                            <div class="text-center uppercase text-red-500 font-bold">{"Entregado"}</div>
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
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{} m", sim.path_parameters.separacion)} />
                                                </div>
                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Azimut"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{}°", sim.path_parameters.azimut)} />
                                                </div>
                                                
                                                <div class="col-span-2 text-xs font-bold text-white/40 uppercase tracking-wider mt-2 mb-0.5">{"Parámetros de Transporte"}</div>
                                                <div class="flex flex-col gap-1 col-span-2">
                                                    <span class={label_cls}>{"Velocidad"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{} m/s", sim.transport_parameters.speed)} />
                                                </div>
                                                
                                                <div class="col-span-2 text-xs font-bold text-white/40 uppercase tracking-wider mt-1 mb-0.5">{"Sensores Activos"}</div>
                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Mareógrafo"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={if sim.transport_parameters.uses_mareograph { "Sí" } else { "No" }} />
                                                </div>
                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Perfilador de sonido"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={if sim.transport_parameters.uses_sound_profiler { "Sí" } else { "No" }} />
                                                </div>
                                                <div class="flex flex-col gap-1 col-span-2">
                                                    <span class={label_cls}>{"Sensor Inercial"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={if sim.transport_parameters.uses_inertial_sensor { "Sí" } else { "No" }} />
                                                </div>

                                                <div class="col-span-2 text-xs font-bold text-white/40 uppercase tracking-wider mt-2 mb-0.5">{"Configuración de Ecosonda"}</div>
                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Frecuencia"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={if sim.echosounder_parameters.uses_high_frecuency { "Alta" } else { "Baja" }} />
                                                </div>

                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Ganancia"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{} dB", sim.echosounder_parameters.gain)} />
                                                </div>

                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Umbral"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={sim.echosounder_parameters.threshold.to_string()} />
                                                </div>

                                                <div class="flex flex-col gap-1">
                                                    <span class={label_cls}>{"Velocidad del Sonido"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{} m/s", sim.echosounder_parameters.sound_speed)} />
                                                </div>

                                                <div class="flex flex-col gap-1 col-span-2">
                                                    <span class={label_cls}>{"Límites (Mín/Máx)"}</span>
                                                    <input type="text" readonly=true disabled=true class={input_cls} value={format!("{}m a {}m", sim.echosounder_parameters.min_limit, sim.echosounder_parameters.max_limit)} />
                                                </div>

                                                <div class="col-span-2">
                                                    { if props.exam_mode {
                                                        html! {
                                                            <div class="w-full pt-0.5">
                                                                <button onclick={on_select_delivery} class={btn_cls}>
                                                                    { if is_selected { html!{ <Check size={14} /> } } else { html!{ <Circle size={14} /> } } }
                                                                    { if is_selected { "ENTREGADO" } else { "ENTREGAR" } }
                                                                </button>
                                                            </div>
                                                        }
                                                    } else {
                                                        html! {}
                                                    } }
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html!{}
                                    } }
                                </div>
                            }
                        }) }
                    </>
                }
            } }
        </div>
    }
}