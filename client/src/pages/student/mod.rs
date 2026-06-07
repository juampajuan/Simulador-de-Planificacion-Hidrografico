use yew::prelude::*;
use crate::components::root::{Root};
pub mod components;
use self::components::img_viewer::IMGviewer;
use self::components::parameters_cont::ParamCont;
use self::components::path_params::PathParams;
use self::components::measure_params::MeasuresParams;
use crate::requests::PathState;

#[function_component(StudentPage)]
pub fn student_page() -> Html {
    let mensaje = use_state(|| "Seleccione parametros para el recorrido".to_string());
    let image_url = use_state(|| None::<String>);
    let loading = use_state(|| false);
    
    let path_state = use_state(PathState::default);

    html! {
        <Root title={"Simulador de Planificación Hidrográfico"}>
            <ParamCont>
                <PathParams 
                    path_state={path_state.clone()}
                    mensaje={mensaje.clone()} 
                    image_url={image_url.clone()} 
                    loading={loading.clone()} 
                />
                
                <MeasuresParams 
                    mensaje={mensaje.clone()} 
                    image_url={image_url.clone()} 
                    loading={loading.clone()} 
                    path_state={path_state.clone()}
                /> 
            </ParamCont>

            <IMGviewer
                image_url={(*image_url).clone()}
                mensaje={(*mensaje).clone()}
                loading={loading.clone()} 
            />
        </Root>
    }
}