use yew::prelude::*;
use crate::components::root::{Root};
pub mod components;
use self::components::img_viewer::IMGviewer;
use self::components::parameters_cont::ParamCont;
use self::components::path_params::PathParams;
use self::components::measure_params::MeasuresParams;
use self::components::info::InfoParams; // Nuevo import
use crate::structs::state::PathState;
use crate::structs::state::SimulationUiState;
use crate::services::requests::get_system_limits;
use crate::structs::limits::ConfigLimits;

use crate::structs::project::AdminProjectView; 
use crate::services::requests::get_student_project;

#[derive(PartialEq, Clone, Copy)]
enum ActiveTab {
    Parametros,
    Entorno,
}

#[function_component(StudentPage)]
pub fn student_page() -> Html {
    let mensaje = use_state(|| "Seleccione parametros para el recorrido".to_string());
    let image_url = use_state(|| None::<String>);
    let loading = use_state(|| true);
    
    let path_state = use_state(PathState::default);
    let limits_state = use_state(ConfigLimits::default);
    let project_state = use_state(|| None::<AdminProjectView>);
    let active_tab = use_state(|| ActiveTab::Parametros);

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
            
            <ParamCont>
                <div class="flex gap-2 p-1 mb-4 bg-zinc-900 border border-white/10 rounded w-full">
                    <button 
                        onclick={set_tab_parametros}
                        class={classes!(
                            "flex-1", "py-2", "text-xs", "font-semibold", "rounded", "transition-all", "cursor-pointer", "text-center",
                            if *active_tab == ActiveTab::Parametros {
                                vec!["bg-zinc-700", "text-white"]
                            } else {
                                vec!["text-white/40", "hover:text-white/70"]
                            }
                        )}
                    >
                        {"SIMULACIÓN"}
                    </button>
                    <button 
                        onclick={set_tab_entorno}
                        class={classes!(
                            "flex-1", "py-2", "text-xs", "font-semibold", "rounded", "transition-all", "cursor-pointer", "text-center",
                            if *active_tab == ActiveTab::Entorno {
                                vec!["bg-zinc-700", "text-white"]
                            } else {
                                vec!["text-white/40", "hover:text-white/70"]
                            }
                        )}
                    >
                        {"INFORMACIÓN"}
                    </button>
                </div>

                {
                    match *active_tab {
                        ActiveTab::Parametros => html! {
                            <>
                                <PathParams 
                                    path_state={path_state.clone()}
                                    ui_state={ui_state.clone()} 
                                    limits={limits_state.clone()}
                                />
                                <MeasuresParams 
                                    ui_state={ui_state.clone()}
                                    path_state={path_state.clone()}
                                    limits={limits_state.clone()}
                                /> 
                            </>
                        },
                        ActiveTab::Entorno => html! {
                            <InfoParams project_state={project_state.clone()} />
                        }
                    }
                }
            </ParamCont>

            <IMGviewer ui_state={ui_state.clone()} />
        </Root>
    }
}