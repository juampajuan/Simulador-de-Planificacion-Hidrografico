use crate::components::image_selector::ImageSelector;
use crate::components::root::Root;
use crate::pages::student::components::mapback::MapBackground;
use lucide_yew::{BookText, DraftingCompass, History};
use yew::prelude::*;
pub mod components;
use self::components::history::HistoryParams;
use self::components::img_viewer::IMGviewer;
use self::components::info::InfoParams;
use self::components::measure_params::{AttemptsState, MeasuresParams};
use self::components::parameters_cont::ParamCont;
use self::components::path_params::PathParams;
use crate::services::requests::{
    StudentProjectResponse, StudentSimulation, get_student_project,
    get_student_simulations_history, get_system_limits,
};
use crate::structs::limits::ConfigLimits;
use crate::structs::project::AdminProjectView;
use crate::structs::state::PathState;
use crate::structs::state::SimulationUiState;

#[derive(PartialEq, Clone, Copy)]
enum ActiveTab {
    Entorno,
    Parametros,
    Historial,
}

#[function_component(StudentPage)]
pub fn student_page() -> Html {
    let mensaje = use_state(|| "Seleccione parametros para el recorrido".to_string());
    let image_url = use_state(|| None::<String>);
    let loading = use_state(|| true);

    let min_depth = use_state(|| 0.0f64);
    let max_depth = use_state(|| 0.0f64);

    let path_state = use_state(PathState::default);
    let limits_state = use_state(ConfigLimits::default);

    let project_state = use_state(|| None::<StudentProjectResponse>);
    let attempts_state = use_state(AttemptsState::default);
    let info_project_state = use_state(|| None::<AdminProjectView>);

    let active_tab = use_state(|| ActiveTab::Entorno);
    let history_state = use_state(Vec::<StudentSimulation>::new);

    let active_layers_sim = use_state(|| None::<StudentSimulation>);

    let ui_state = SimulationUiState {
        mensaje: mensaje.clone(),
        image_url: image_url.clone(),
        loading: loading.clone(),
        show_legend: use_state(|| false),
        min_depth: min_depth.clone(),
        max_depth: max_depth.clone(),
        simulation_image_path: use_state(|| None::<String>),
        coverage_image_path: use_state(|| None::<String>),
        difference_image_path: use_state(|| None::<String>),
    };

    {
        let active_layers_sim = active_layers_sim.clone();
        use_effect_with(active_tab.clone(), move |_| {
            active_layers_sim.set(None);
            || ()
        });
    }

    {
        let attempts_handle = attempts_state.clone();
        let info_project_handle = info_project_state.clone();
        let min_depth_handle = min_depth.clone();
        let max_depth_handle = max_depth.clone();
        let project_data = (*project_state).clone();

        use_effect_with(project_state.clone(), move |_| {
            if let Some(data) = project_data {
                attempts_handle.set(AttemptsState {
                    spent: data.attempts_spent,
                    limit: data.project.metadata.attempts_limit,
                });
                info_project_handle.set(Some(data.project.clone()));

                min_depth_handle.set(data.project.metadata.geotiff_min_depth);
                max_depth_handle.set(data.project.metadata.geotiff_max_depth);
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
            get_system_limits(
                limits_handle,
                mensaje_handle.clone(),
                loading_handle.clone(),
            );
            get_student_project(project_handle, mensaje_handle, loading_handle);
            || ()
        });
    }

    {
        let active_tab_clone = active_tab.clone();
        let history_handle = history_state.clone();
        let mensaje_handle = mensaje.clone();
        let loading_handle = loading.clone();

        use_effect_with(active_tab.clone(), move |_| {
            if *active_tab_clone == ActiveTab::Historial {
                get_student_simulations_history(
                    None,
                    history_handle,
                    mensaje_handle,
                    loading_handle,
                );
            }
            || ()
        });
    }

    let set_tab_entorno = {
        let active_tab = active_tab.clone();
        Callback::from(move |_| active_tab.set(ActiveTab::Entorno))
    };

    let set_tab_parametros = {
        let active_tab = active_tab.clone();
        Callback::from(move |_| active_tab.set(ActiveTab::Parametros))
    };

    let set_tab_historial = {
        let active_tab = active_tab.clone();
        Callback::from(move |_| active_tab.set(ActiveTab::Historial))
    };

    let base_btn = "py-2 text-xs font-semibold rounded-sm transition-all cursor-pointer text-center flex justify-center items-center gap-2 h-9";

    let (entorno_cls, entorno_text) = if *active_tab == ActiveTab::Entorno {
        (
            format!("{} w-11 flex-none bg-zinc-700 text-white", base_btn),
            html! {},
        )
    } else {
        (
            format!("{} flex-1 text-white/40 hover:text-white/70 px-3", base_btn),
            html! { <span>{"INFORMACIÓN"}</span> },
        )
    };

    let (parametros_cls, parametros_text) = if *active_tab == ActiveTab::Parametros {
        (
            format!("{} w-11 flex-none bg-zinc-700 text-white", base_btn),
            html! {},
        )
    } else {
        (
            format!("{} flex-1 text-white/40 hover:text-white/70 px-3", base_btn),
            html! { <span>{"SIMULACIÓN"}</span> },
        )
    };

    let (historial_cls, historial_text) = if *active_tab == ActiveTab::Historial {
        (
            format!("{} w-11 flex-none bg-zinc-700 text-white", base_btn),
            html! {},
        )
    } else {
        (
            format!("{} flex-1 text-white/40 hover:text-white/70 px-3", base_btn),
            html! { <span>{"HISTORIAL"}</span> },
        )
    };

    let (geo_min, geo_max) = if let Some(p_data) = &*project_state {
        (
            p_data.project.metadata.geotiff_min_depth,
            p_data.project.metadata.geotiff_max_depth,
        )
    } else if let Some(info_p) = &*info_project_state {
        (
            info_p.metadata.geotiff_min_depth,
            info_p.metadata.geotiff_max_depth,
        )
    } else {
        (0.0, 0.0)
    };

    let (current_sim_min, current_sim_max) = match &*active_layers_sim {
        Some(sim) => (sim.result_min_depth, sim.result_max_depth),
        None => (*ui_state.min_depth, *ui_state.max_depth),
    };

    {
        let ui_state = ui_state.clone();
        let active = active_layers_sim.clone();
        use_effect_with(active_layers_sim.clone(), move |_| {
            ui_state.image_url.set(None);
            ui_state.show_legend.set(false);
            ui_state.simulation_image_path.set(None);
            ui_state.coverage_image_path.set(None);
            ui_state.difference_image_path.set(None);

            if let Some(sim) = &*active {
                ui_state.min_depth.set(sim.result_min_depth);
                ui_state.max_depth.set(sim.result_max_depth);
                ui_state.mensaje.set(String::new());
            } else {
                ui_state
                    .mensaje
                    .set("Seleccione un intento del historial para revisar".to_string());
            }
            || ()
        });
    }

    html! {
        <Root title={"Simulador de Planificación Hidrográfico"}>

            <div class="overflow-hidden border border-white/20 rounded-2xl relative w-full">

                <MapBackground project_state={project_state.clone()} />

                <div class="flex w-full dot-grid-dark bg-slate-950/50 p-2 relative h-full gap-2">

                    <ParamCont
                        header={html! {
                            <div class="flex gap-1.5 p-1 bg-zinc-900 border border-white/10 rounded w-full items-center">
                                <button onclick={set_tab_entorno} class={entorno_cls} title="Información">
                                    <BookText size={16}/> {entorno_text}
                                </button>
                                <button onclick={set_tab_parametros} class={parametros_cls} title="Simulación">
                                    <DraftingCompass size={16}/> {parametros_text}
                                </button>
                                <button onclick={set_tab_historial} class={historial_cls} title="Historial">
                                    <History size={16}/> {historial_text}
                                </button>
                            </div>
                        }}
                    >
                        {
                            match *active_tab {
                                ActiveTab::Entorno => html! { <InfoParams project_state={info_project_state.clone()} /> },
                                ActiveTab::Parametros => html! {
                                    <>
                                        <PathParams path_state={path_state.clone()} ui_state={ui_state.clone()} limits={limits_state.clone()} />
                                        <MeasuresParams ui_state={ui_state.clone()} path_state={path_state.clone()} limits={limits_state.clone()} attempts={attempts_state.clone()} active_layers_sim={active_layers_sim.clone()} history_state={history_state.clone()} />
                                    </>
                                },
                                ActiveTab::Historial => {
                                    let is_exam_mode = info_project_state.as_ref().map(|p| p.metadata.exam_mode).unwrap_or(false);
                                    let due_date = info_project_state.as_ref().and_then(|p| p.metadata.due_date.clone());

                                    html! {
                                        <HistoryParams
                                            history_state={history_state.clone()}
                                            ui_mensaje={mensaje.clone()}
                                            exam_mode={is_exam_mode}
                                            due_date={due_date.clone()}
                                            ui_state={ui_state.clone()}
                                            active_layers_sim={active_layers_sim.clone()}
                                        />
                                    }
                                }
                            }
                        }
                    </ParamCont>

                    <div class="flex-1 relative flex flex-col h-full">

                        { if let Some(sim) = &*active_layers_sim {
                            html! {
                                <div class="absolute top-0 right-0 z-10">
                                    <ImageSelector
                                        key={sim.id.to_string()}
                                        ui_state={ui_state.clone()}
                                        active_sim={sim.clone()}
                                    />
                                </div>
                            }
                        } else { html!{} } }

                        <IMGviewer
                            ui_state={ui_state.clone()}
                            geotiff_min={geo_min}
                            geotiff_max={geo_max}
                            sim_min={current_sim_min}
                            sim_max={current_sim_max}
                        />
                    </div>

                </div>

            </div>
        </Root>
    }
}
