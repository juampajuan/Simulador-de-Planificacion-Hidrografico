use yew::prelude::*;
use crate::components::modal::Modal;
use crate::structs::project::Project;
use crate::components::image_selector::ImageSelector;
use crate::pages::student::components::img_viewer::IMGviewer;
use crate::pages::student::components::history::HistoryParams; 
use crate::pages::student::components::parameters_cont::ParamCont;
use crate::structs::state::SimulationUiState;
use crate::services::requests::{get_student_simulations_history, StudentSimulation};
use lucide_yew::History;

#[derive(Properties, PartialEq)]
pub struct AttemptsModalProps {
    pub is_open: bool,
    pub project: Project,
    pub student_id: i64, 
    pub student_code: String,
    pub on_close: Callback<()>,
}

#[function_component(AttemptsModal)]
pub fn attempts_modal(props: &AttemptsModalProps) -> Html {
    let mensaje = use_state(|| "Seleccione un intento del historial para revisar".to_string());
    let image_url = use_state(|| None::<String>);
    let loading = use_state(|| false);
    
    let map_base64 = use_state(|| None::<String>);
    let scale_base64 = use_state(|| None::<String>);
    let min_depth = use_state(|| 0.0f64);
    let max_depth = use_state(|| 0.0f64);

    let ui_state = SimulationUiState {
        mensaje: mensaje.clone(),
        image_url: image_url.clone(),
        loading: loading.clone(),
        map_base64: map_base64.clone(),
        scale_base64: scale_base64.clone(),
        min_depth: min_depth.clone(),
        max_depth: max_depth.clone(),
    };

    let history_state = use_state(Vec::<StudentSimulation>::new);
    let active_layers_sim = use_state(|| None::<StudentSimulation>); 

    {
        let history_handle = history_state.clone();
        let mensaje_handle = mensaje.clone();
        let loading_handle = loading.clone();
        let student_id = props.student_id;
        let is_open = props.is_open;
        
        use_effect_with(props.is_open, move |_| {
            if is_open {
                get_student_simulations_history(
                    Some(student_id), 
                    history_handle,
                    mensaje_handle,
                    loading_handle,
                );
            }
            || ()
        });
    }

    {
        let ui_state = ui_state.clone();
        
        use_effect_with(active_layers_sim.clone(), move |active| {
            if let Some(sim) = &**active {
                if let Some(path) = &sim.simulation_image_path {
                    ui_state.image_url.set(Some(format!("/images/{}", path)));
                }
                ui_state.min_depth.set(sim.result_min_depth);
                ui_state.max_depth.set(sim.result_max_depth);
                ui_state.mensaje.set(String::new());
            } else {
                ui_state.image_url.set(None);
                ui_state.mensaje.set("Seleccione un intento del historial para revisar".to_string());
            }
            || ()
        });
    }

    if !props.is_open { 
        return html! {}; 
    }

    let on_close_callback = props.on_close.clone();

    let filtered_history = (*history_state)
        .iter()
        .filter(|sim| sim.project_id == props.project.id)
        .cloned()
        .collect::<Vec<StudentSimulation>>();
        
    let filtered_history_handle = use_state(|| filtered_history);

    {
        let filtered_history_handle = filtered_history_handle.clone();
        let history_state_raw = history_state.clone();
        let project_id = props.project.id;
        use_effect_with(history_state.clone(), move |_| {
            let filtered = (*history_state_raw).iter().filter(|sim| sim.project_id == project_id).cloned().collect::<Vec<StudentSimulation>>();
            filtered_history_handle.set(filtered);
            || ()
        });
    }

    html! {
        <Modal
            title={format!("{} — RESULTADOS DE: {}", props.project.name, props.student_code)}
            subtitle=""
            on_close={on_close_callback}
            max_width_class={Some("max-w-[95vw] w-[1450px] h-[88vh] flex flex-col".to_string())}
        >
            <div class="flex-1 flex w-full dot-grid-dark bg-slate-950/50 p-2 relative h-full gap-2 rounded-xl overflow-hidden border border-white/10">
                
                <ParamCont
                    header={html! {
                        <div class="flex gap-1.5 p-1 bg-zinc-900 border border-white/10 rounded w-full items-center">
                            <div class="py-2 text-xs font-semibold rounded-sm bg-zinc-700 text-white w-full h-9 flex justify-center items-center gap-2 select-none">
                                <History size={16}/> 
                                <span>{"HISTORIAL DE INTENTOS"}</span>
                            </div>
                        </div>
                    }}
                >
                    <div class="h-full overflow-y-auto">
                        <HistoryParams 
                            history_state={filtered_history_handle} 
                            ui_mensaje={mensaje.clone()}
                            exam_mode={false} 
                            due_date={props.project.due_date.clone()}
                            ui_state={ui_state.clone()} 
                            active_layers_sim={active_layers_sim.clone()}
                        />
                    </div>
                </ParamCont>
                
                <div class="flex-1 relative flex flex-col h-full rounded-lg overflow-hidden">
                    
                    { if let Some(sim) = &*active_layers_sim {
                        html! { 
                            <div class="absolute top-0 left-3 z-50">
                                <ImageSelector ui_state={ui_state.clone()} active_sim={sim.clone()} /> 
                            </div>
                        }
                    } else { html!{} } }

                    <IMGviewer ui_state={ui_state.clone()} />
                </div>

            </div>
        </Modal>
    }
}