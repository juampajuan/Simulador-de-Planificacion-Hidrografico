use yew::prelude::*;
use crate::structs::state::SimulationUiState;

#[derive(Properties, PartialEq)]
pub struct IMGviewerProps {
    pub ui_state: SimulationUiState,
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
                flex-1 bg-cyan-100 dark:bg-zinc-900 flex items-center justify-center
                overflow-hidden transition-colors p-2 border border-white/20
                rounded-md dot-grid relative dark:dot-grid-dark
            "
        >
            <div class="flex
                items-center
                justify-center
                h-full w-full overflow-hidden
                p-8 relative
            ">

            {
                if let (Some(m_b64), Some(s_b64)) = (map_base64, scale_base64) { //esto para la simulacion
                    html! {
                        <div class="relative flex items-center justify-center h-full w-full">
                            <img
                                src={format!("data:image/png;base64,{}", m_b64)}
                                class="h-full object-contain rounded-lg"
                            />

                            <div class="absolute right-4 top-1/2 -translate-y-1/2 flex flex-col items-center z-50">
                                
                                <span class="px-3 py-1 text-[11px] font-sans font-medium text-white bg-zinc-800/80 rounded-full border border-white/5 shadow-md mb-[-6px] z-10 min-w-[50px] text-center">
                                    { format!("{:.1}m", min_depth) }
                                </span>
                                
                                <img //LA barra
                                    src={format!("data:image/png;base64,{}", s_b64)} 
                                    class="h-[520px] w-[10px] rounded-full border border-white/10" 
                                />
                                
                                <span class="px-3 py-1 text-[11px] font-sans font-medium text-white bg-zinc-800/80 rounded-full border border-white/5 shadow-md mt-[-6px] z-10 min-w-[50px] text-center">
                                    { format!("{:.1}m", max_depth) }
                                </span>

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