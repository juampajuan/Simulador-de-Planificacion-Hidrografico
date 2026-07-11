use yew::prelude::*;
use crate::structs::project::AdminProjectView;
use crate::components::subtitle::Subtitle;
use lucide_yew::{FolderSync, TriangleAlert, ChevronDown, ChevronUp, CalendarClock};

#[derive(Properties, PartialEq)]
pub struct InfoProps {
    pub project_state: UseStateHandle<Option<AdminProjectView>>,
}

/// Pestaña "Información": muestra los datos del proyecto asignado al alumno.
#[function_component(InfoParams)]
pub fn info_params(props: &InfoProps) -> Html {
    let input_cls = "rounded p-2 text-sm bg-zinc-700/50 text-white border border-white/5 w-full font-mono"; // Agregado font-mono para consistencia de datos
    let normal_input_cls = "rounded p-2 text-sm bg-zinc-700/50 text-white border border-white/5 w-full";

    let is_project_open = use_state(|| true);
    let is_restrictions_open = use_state(|| true);

    let toggle_project = {
        let is_project_open = is_project_open.clone();
        Callback::from(move |_| is_project_open.set(!*is_project_open))
    };

    let toggle_restrictions = {
        let is_restrictions_open = is_restrictions_open.clone();
        Callback::from(move |_| is_restrictions_open.set(!*is_restrictions_open))
    };

    let due_date_to_render = props.project_state.as_ref()
        .and_then(|p| p.metadata.due_date.as_deref())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    html! {
        <div class="flex flex-col w-full text-white">
            if let Some(p) = &*props.project_state {
                
                <div class="border-white/25 p-3 pt-0 border-b flex flex-col gap-3">
                    <div 
                        onclick={toggle_project}
                        class="flex justify-between items-center transition-colors cursor-pointer py-1"
                    >
                        <Subtitle
                            text={"Proyecto Asignado"}
                            icon={html! { <FolderSync size={18} /> }}
                        />
                        <span class="text-white/40 hover:text-white/70 transition-all">
                            { if *is_project_open { html!{ <ChevronUp size={16} /> } } else { html!{ <ChevronDown size={16} /> } } }
                        </span>
                    </div>
                    
                    if *is_project_open {
                        <div class="flex flex-col gap-3">
                            <div class="flex flex-col gap-1">
                                <span class="text-xs font-semibold text-white/40 ml-1">{"Nombre"}</span>
                                <input type="text" readonly=true disabled=true class={normal_input_cls} value={p.metadata.name.clone()} />
                            </div>
                            
                            <div class="flex flex-col gap-1">
                                <span class="text-xs font-semibold text-white/40 ml-1">{"Archivo Geográfico"}</span>
                                <input type="text" readonly=true disabled=true class={format!("{} text-xs text-slate-300", input_cls)} value={p.filename.clone()} />
                            </div>

                            if let Some(desc) = &p.metadata.description {
                                <div class="flex flex-col gap-1">
                                    <span class="text-xs font-semibold text-white/40 ml-1">{"Descripción"}</span>
                                    <textarea readonly=true disabled=true class={format!("{} italic resize-none h-16", normal_input_cls)} value={desc.clone()} />
                                </div>
                            }

                            if let Some(due) = due_date_to_render {
                                <div class="flex p-2.5 bg-red-500/10 border border-red-500/20 rounded-lg items-center gap-3 select-none mt-1 animate-fade-in">
                                    <span class="text-red-400 shrink-0"><CalendarClock size={18} /></span>
                                    <div class="flex flex-col">
                                        <span class="text-[10px] font-bold text-red-400/60 uppercase tracking-wider">{"Fecha Límite de Entrega"}</span>
                                        <span class="text-sm font-mono font-semibold text-red-300">{ due }</span>
                                    </div>
                                </div>
                            } else {
                                <div class="flex p-2.5 bg-blue-500/10 border border-blue-500/20 rounded-lg items-center gap-3 select-none mt-1 animate-fade-in">
                                    <span class="text-blue-400 shrink-0"><CalendarClock size={18} /></span>
                                    <div class="flex flex-col">
                                        <span class="text-xs font-semibold text-blue-300">{"Entorno de práctica libre (Sin entrega)"}</span>
                                    </div>
                                </div>
                            }
                        </div>
                    }
                </div>

                <div class="border-white/25 p-3 flex flex-col gap-3">
                    <div 
                        onclick={toggle_restrictions}
                        class="flex justify-between items-center transition-colors cursor-pointer py-1"
                    >
                        <Subtitle
                            text={"Restricciones Técnicas"}
                            icon={html! { <TriangleAlert size={18} /> }}
                        />
                        <span class="text-white/40 hover:text-white/70 transition-all">
                            { if *is_restrictions_open { html!{ <ChevronUp size={16} /> } } else { html!{ <ChevronDown size={16} /> } } }
                        </span>
                    </div>
                    
                    if *is_restrictions_open {
                        <div class="grid grid-cols-2 gap-3">
                            <div class="flex flex-col gap-1">
                                <span class="text-xs font-semibold text-white/40 ml-1">{"Límite de Intentos"}</span>
                                <input type="text" readonly=true disabled=true class={input_cls} value={p.metadata.attempts_limit.to_string()} />
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-xs font-semibold text-white/40 ml-1">{"Estado del Clima"}</span>
                                <input type="text" readonly=true disabled=true class={normal_input_cls} value={p.metadata.weather.clone()} />
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-xs font-semibold text-white/40 ml-1">{"Dureza del Fondo"}</span>
                                <input type="text" readonly=true disabled=true class={normal_input_cls} value={p.metadata.seabed_hardness.clone()} />
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-xs font-semibold text-white/40 ml-1">{"Presupuesto Máximo"}</span>
                                <input type="text" readonly=true disabled=true class={input_cls} value={format!("${:.2}", p.metadata.budget)} />
                            </div>
                            <div class="flex flex-col gap-1 col-span-2">
                                <span class="text-xs font-semibold text-white/40 ml-1">{"Rango Profundidad GeoTIFF"}</span>
                                <input type="text" readonly=true disabled=true class={input_cls} value={format!("{}m a {}m", p.metadata.geotiff_min_depth, p.metadata.geotiff_max_depth)} />
                            </div>
                        </div>
                    }
                </div>
                
            } else {
                <div class="text-center py-8 text-white/30 italic text-sm w-full">
                    {"Cargando datos del entorno..."}
                </div>
            }
        </div>
    }
}