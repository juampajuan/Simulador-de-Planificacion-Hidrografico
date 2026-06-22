use lucide_yew::{DraftingCompass, BookText};
use yew::prelude::*;
use crate::components::root::{Root};
pub mod components;
use self::components::img_viewer::IMGviewer;
use self::components::parameters_cont::ParamCont;
use self::components::path_params::PathParams;
use self::components::measure_params::{MeasuresParams, AttemptsState};
use self::components::info::InfoParams;
use crate::structs::state::PathState;
use crate::structs::state::SimulationUiState;
use crate::services::requests::get_system_limits;
use crate::structs::limits::ConfigLimits;

use crate::structs::project::AdminProjectView; 
use crate::services::requests::{get_student_project, StudentProjectResponse};

#[derive(PartialEq, Clone, Copy)]
enum ActiveTab {
    Parametros,
    Entorno,
}

// Setea imagen, parámetros, y entorno (info proyecto)
#[function_component(StudentPage)]
pub fn student_page() -> Html {
    let mensaje = use_state(|| "Seleccione parametros para el recorrido".to_string());
    let image_url = use_state(|| None::<String>);
    let loading = use_state(|| true);
    
    let map_base64 = use_state(|| None::<String>);
    let scale_base64 = use_state(|| None::<String>);
    let min_depth = use_state(|| 0.0f64);
    let max_depth = use_state(|| 0.0f64);
    
    let path_state = use_state(PathState::default);
    let limits_state = use_state(ConfigLimits::default);
    
    let project_state = use_state(|| None::<StudentProjectResponse>);
    
    let attempts_state = use_state(AttemptsState::default);

    let info_project_state = use_state(|| None::<AdminProjectView>);

    let active_tab = use_state(|| ActiveTab::Parametros);

    {
        let attempts_handle = attempts_state.clone();
        let info_project_handle = info_project_state.clone();
        let project_data = (*project_state).clone();

        use_effect_with(project_state.clone(), move |_| {
            if let Some(data) = project_data {
                // Setea los intentos reales extraídos de la DB
                attempts_handle.set(AttemptsState {
                    spent: data.attempts_spent,
                    limit: data.project.metadata.attempts_limit,
                });
                
                // Setea la info limpia del proyecto para el visualizador del entorno
                info_project_handle.set(Some(data.project));
            }
            || ()
        });
    }

    {
        let limits_handle = limits_state.clone();
        let mensaje_handle = mensaje.clone();
        let loading_handle = loading.clone();
        let project_handle = project_state.clone();

        use_effect_with((), move |_| {
            get_system_limits(limits_handle, mensaje_handle.clone(), loading_handle.clone());
            get_student_project(project_handle, mensaje_handle, loading_handle);
            || ()
        });
    }

    let ui_state = SimulationUiState {
        mensaje: mensaje.clone(),
        image_url: image_url.clone(),
        loading: loading.clone(),
        map_base64: map_base64.clone(),
        scale_base64: scale_base64.clone(),
        min_depth: min_depth.clone(),
        max_depth: max_depth.clone(),
    };

    let set_tab_parametros = {
        let active_tab = active_tab.clone();
        Callback::from(move |_| active_tab.set(ActiveTab::Parametros))
    };

    let set_tab_entorno = {
        let active_tab = active_tab.clone();
        Callback::from(move |_| active_tab.set(ActiveTab::Entorno))
    };

    html! {
        <Root title={"Simulador de Planificación Hidrográfico"}>
            <ParamCont
                header={html! {
                    <div class="flex gap-2 p-1 bg-zinc-900 border border-white/10 rounded w-full">
                        <button 
                            onclick={set_tab_parametros}
                            class={classes!(
                                "flex-1", "py-2", "text-xs", "font-semibold", "rounded", "transition-all", "cursor-pointer", "text-center", "flex", "justify-center", "items-center", "gap-2",
                                if *active_tab == ActiveTab::Parametros {
                                    vec!["bg-zinc-700", "text-white"]
                                } else {
                                    vec!["text-white/40", "hover:text-white/70"]
                                }
                            )}
                        >
                            <DraftingCompass size={18}/>
                            {"SIMULACIÓN"}
                        </button>
                        <button 
                            onclick={set_tab_entorno}
                            class={classes!(
                                "flex-1", "py-2", "text-xs", "font-semibold", "rounded", "transition-all", "cursor-pointer", "text-center", "flex", "justify-center", "items-center", "gap-2",
                                if *active_tab == ActiveTab::Entorno {
                                    vec!["bg-zinc-700", "text-white"]
                                } else {
                                    vec!["text-white/40", "hover:text-white/70"]
                                }
                            )}
                        >
                            <BookText size={18}/>
                            {"INFORMACIÓN"}
                        </button>
                    </div>
                }}
            >

                {
                    match *active_tab {
                        ActiveTab::Parametros => html! {
                            <>
                                <PathParams path_state={path_state.clone()} ui_state={ui_state.clone()} limits={limits_state.clone()} />
                                <MeasuresParams 
                                    ui_state={ui_state.clone()}
                                    path_state={path_state.clone()}
                                    limits={limits_state.clone()}
                                    attempts={attempts_state.clone()} 
                                /> 
                            </>
                        },
                        ActiveTab::Entorno => html! {
                            <InfoParams project_state={info_project_state.clone()} />
                        }
                    }
                }
            </ParamCont>
            <IMGviewer ui_state={ui_state.clone()} project_state={project_state.clone()}/>
        </Root>
    }
}