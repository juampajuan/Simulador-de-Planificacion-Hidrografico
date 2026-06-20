use yew::prelude::*;
use crate::{pages::student::components::{depthcolors::DepthLegend, mapback::MapBackground}, services::requests::StudentProjectResponse, structs::state::SimulationUiState};

#[derive(Properties, PartialEq)]
pub struct IMGviewerProps {
    pub ui_state: SimulationUiState,
    pub project_state: UseStateHandle<Option<StudentProjectResponse>>,
}

#[function_component(IMGviewer)]
pub fn imgviewer(props: &IMGviewerProps) -> Html {
    let mensaje = &*props.ui_state.mensaje;
    let loading = *props.ui_state.loading;

    let image_url = (*props.ui_state.image_url).as_ref();

    let map_base64 = (*props.ui_state.map_base64).as_ref();
    let scale_base64 = (*props.ui_state.scale_base64).as_ref();
    let min_depth = *props.ui_state.min_depth;
    let max_depth = *props.ui_state.max_depth;

    html! {
        <div
            class="
                flex-1 bg-cyan-100 dark:bg-slate-950 flex items-center justify-center
                overflow-hidden transition-colors border border-white/20
                rounded-lg dot-grid relative dark:dot-grid-dark
            "
        >
            <div class="flex
                items-center
                justify-center
                h-full w-full overflow-hidden
                p-8 relative
            ">

                <MapBackground project_state={&props.project_state.clone()} />

                {
                    if let (Some(m_b64), Some(_)) = (map_base64, scale_base64) { //esto para la simulacion
                        html! {
                            <div class="relative flex items-center justify-center h-full w-full">
                                <img
                                    src={format!("data:image/png;base64,{}", m_b64)}
                                    class="h-full object-contain rounded-lg"
                                />

                                <div class="absolute right-0 h-full top-1/2 -translate-y-1/2 flex flex-col items-center z-50">
                                    <DepthLegend
                                        start_m={min_depth}
                                        end_m={max_depth}
                                    />
                                </div>
                            
                            </div>
                        }
                    } else if let Some(url) = image_url { //esto para path, usa el blob
                        html! {
                            <img
                                key={url.to_string()}
                                src={url.clone()}
                                class="h-full object-contain rounded-lg"
                            />
                        }
                    } else {
                        html! {
                            <h2 class="text-2xl font-bold dark:text-white text-center p-8">
                                { mensaje }
                            </h2>
                        }
                    }
                }
                
                { 
                    if loading {
                        html! {
                            <div class="flex flex-col absolute top-0 z-[100] backdrop-blur left-0 dark:bg-black/50 w-full h-full justify-center items-center">
                                <div class="loader2"/>
                                <h2 class="dark:text-cyan-200 font-bold text-center p-5">
                                    { mensaje }
                                </h2>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

            </div>
        </div>
    }
}