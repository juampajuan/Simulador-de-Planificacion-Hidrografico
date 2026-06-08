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

    html! {
        <div
            class="
                flex-1 bg-cyan-100 dark:bg-zinc-900 flex items-center justify-center
                overflow-hidden transition-colors p-2 border border-white/20
                rounded-md dot-grid relative dark:dot-grid-dark
            "
        >

            // <div class="absolute w-full h-full scale-110 opacity-60 no-interaction">
            //     <iframe width="100%" height="100%" allow="geolocation" src="https://api.maptiler.com/maps/dataviz-v4/?key=EVEAYM1Cx9nGoDR5OVX6#16.7/-34.59543331/-58.36668597"></iframe>
            //     // winter-v4
            //     // backdrop
            // </div>

            <div class="flex
                items-center
                justify-center
                h-full w-full overflow-hidden
                p-8
            ">

            {
                if let Some(url) = image_url {
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