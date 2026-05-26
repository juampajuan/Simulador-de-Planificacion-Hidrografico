use yew::prelude::*;
use crate::components::root::{Root};
pub mod components;
use self::components::img_viewer::IMGviewer;
use self::components::parameters_cont::ParamCont;
use self::components::path_params::PathParams;
use self::components::measure_params::MeasuresParams;

#[function_component(StudentPage)]
pub fn student_page() -> Html {
    let mensaje = use_state(|| "Seleccione parametros para el recorrido".to_string());
    let image_url = use_state(|| None::<String>);
    let separacion = use_state(|| "".to_string());
    let azimut = use_state(|| "".to_string());

    html! {
        <Root title={"Simulador"}>
            <ParamCont>
                <PathParams 
                    separacion={separacion} 
                    azimut={azimut} 
                    mensaje={mensaje.clone()} 
                    image_url={image_url.clone()} 
                />
                
                <MeasuresParams />
            </ParamCont>

            <IMGviewer
                image_url={(*image_url).clone()}
                mensaje={(*mensaje).clone()}
            />
        </Root>
    }
}