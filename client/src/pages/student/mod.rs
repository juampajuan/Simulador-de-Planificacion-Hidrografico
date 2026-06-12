use yew::prelude::*;
use crate::components::root::{Root};
pub mod components;
use self::components::img_viewer::IMGviewer;
use self::components::parameters_cont::ParamCont;
use self::components::path_params::PathParams;
use self::components::measure_params::MeasuresParams;
use crate::structs::state::PathState;
use crate::structs::state::SimulationUiState;
use crate::services::requests::get_system_limits;
use crate::structs::limits::ConfigLimits;

#[function_component(StudentPage)]
pub fn student_page() -> Html {
    let mensaje = use_state(|| "Seleccione parametros para el recorrido".to_string());
    let image_url = use_state(|| None::<String>);
    let loading = use_state(|| true);
    
    let path_state = use_state(PathState::default);
    let limits_state = use_state(ConfigLimits::default);

    {
        let limits_handle = limits_state.clone();
        let mensaje_handle = mensaje.clone();
        let loading_handle = loading.clone();

        use_effect_with((), move |_| {
            get_system_limits(limits_handle, mensaje_handle, loading_handle);
            || ()
        });
    }

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
                    limits={limits_state.clone()}
                />
                
                <MeasuresParams 
                    ui_state={ui_state.clone()}
                    path_state={path_state.clone()}
                    limits={limits_state.clone()}
                /> 
            </ParamCont>

            <IMGviewer
                ui_state={ui_state.clone()}
            />
        </Root>
    }
}