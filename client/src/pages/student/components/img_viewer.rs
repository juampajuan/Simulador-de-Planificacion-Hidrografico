use yew::prelude::*;
use crate::{pages::student::components::depthcolors::DepthLegend, structs::state::SimulationUiState};
use lucide_yew::TriangleAlert;

#[derive(Properties, PartialEq)]
pub struct IMGviewerProps {
    pub ui_state: SimulationUiState,
}

#[function_component(IMGviewer)]
pub fn imgviewer(props: &IMGviewerProps) -> Html {
    let mensaje = &*props.ui_state.mensaje;
    let loading = *props.ui_state.loading;

    let image_url = (*props.ui_state.image_url).as_ref();
    let min_depth = *props.ui_state.min_depth;
    let max_depth = *props.ui_state.max_depth;
    let show_legend = *props.ui_state.show_legend;

    html! {
        <div class="flex-1 relative flex items-center justify-center overflow-hidden rounded-lg bg-slate-900/30">
            <div class="flex items-center justify-center h-full w-full overflow-hidden py-8 px-4 relative">
                {
                    if let Some(url) = image_url { 
                        html! {
                            <div class="relative flex items-center justify-center h-full w-full gap-4">
                                <div class="flex-1 flex items-center justify-center h-full overflow-hidden">
                                    <img key={url.to_string()} src={url.clone()} class="max-h-full max-w-full object-contain rounded-lg shadow-lg border border-white/5" />
                                </div>
                                
                                <div class={
                                    if show_legend {
                                        "h-full flex flex-col items-center justify-center z-10 py-4 shrink-0 opacity-100 transition-opacity duration-200"
                                    } else {
                                        "h-full flex flex-col items-center justify-center z-10 py-4 shrink-0 opacity-0 pointer-events-none transition-opacity duration-200"
                                    }
                                }>
                                    <DepthLegend start_m={min_depth} end_m={max_depth} />
                                </div>
                            </div>
                        }
                    } else if !mensaje.is_empty()  {
                        html! {  
                            <div class="flex flex-col absolute top-0 z-[100] left-0 w-full h-full justify-center items-center">
                                <div class="flex flex-col gap-5 text-cyan-200 rounded-lg px-12 py-5 items-center bg-slate-950/60 backdrop-blur border border-white/15">
                                    <TriangleAlert size={42} />
                                    <h2 class="text-cyan-200 font-bold text-center">{ mensaje }</h2>
                                </div>
                            </div>  
                        }
                    } else {
                        Html::default()
                    }
                }
                
                if loading {
                    <div class="flex flex-col absolute top-0 z-[100] left-0 w-full h-full justify-center items-center">
                        <div class="flex flex-col gap-5 rounded-lg px-16 py-8 items-center bg-slate-950/60 backdrop-blur border border-white/15">
                            <div class="loader2"/>
                            <h2 class="text-cyan-200 font-bold text-center">{ mensaje }</h2>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}