use yew::prelude::*;
use crate::components::root::{Root};
pub mod components;
use self::components::img_viewer::IMGviewer;
use self::components::parameters_cont::ParamCont;
use self::components::path_params::PathParams;
use self::components::measure_params::MeasuresParams;
use crate::structs::state::PathState;
use crate::structs::state::SimulationUiState;

#[function_component(StudentPage)]
pub fn student_page() -> Html {
    let mensaje = use_state(|| "Seleccione parametros para el recorrido".to_string());
    let image_url = use_state(|| None::<String>);
    let loading = use_state(|| false);
    
    let path_state = use_state(PathState::default);

    let ui_state = SimulationUiState {
        mensaje: mensaje.clone(),
        image_url: image_url.clone(),
        loading: loading.clone(),
    };

    html! {
        <Root title={"Simulador de Planificación Hidrográfico"}>
            <ParamCont>
                <PathParams 
                    path_state={path_state.clone()}
                    ui_state={ui_state.clone()} 
                />
                
                <MeasuresParams 
                    ui_state={ui_state.clone()}
                    path_state={path_state.clone()}
                /> 
            </ParamCont>

            <IMGviewer
                ui_state={ui_state.clone()}
            />
        </Root>
    }
}