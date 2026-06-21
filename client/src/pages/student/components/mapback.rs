use gloo_timers::callback::Timeout;
use crate::services::requests::StudentProjectResponse;
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;
use crate::services::requests::GeoCorners;
const EARTH_RADIUS: f64 = 6378137.0;

#[derive(Properties, PartialEq)]
pub struct MapBackgroundProps {
    pub project_state: UseStateHandle<Option<StudentProjectResponse>>,
}

#[function_component(MapBackground)]
pub fn map_background(props: &MapBackgroundProps) -> Html {
    let centro = use_state(|| None::<(f64, f64)>);
    let zoom = use_state(|| None::<f64>);
    let api_key = use_state(|| None::<String>);

    let resize_timeout = use_mut_ref(|| None::<Timeout>);

    let window_size = use_state(|| {
        let window = web_sys::window().unwrap();

        (
            window.inner_width().unwrap().as_f64().unwrap(),
            window.inner_height().unwrap().as_f64().unwrap(),
        )
    });
 
    {
        let window_size = window_size.clone();
        let resize_timeout = resize_timeout.clone();

        use_effect(move || {
            let closure = Closure::<dyn FnMut()>::new(move || {
                resize_timeout.borrow_mut().take();

                let window_size = window_size.clone();
                let resize_timeout = resize_timeout.clone();

                let timeout = Timeout::new(200, move || {
                    let window = web_sys::window().unwrap();

                    let width =
                        window.inner_width().unwrap().as_f64().unwrap();
                    let height =
                        window.inner_height().unwrap().as_f64().unwrap();

                    window_size.set((width, height));
                });

                *resize_timeout.borrow_mut() = Some(timeout);
            });

            let window = web_sys::window().unwrap();

            window
                .add_event_listener_with_callback(
                    "resize",
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();

            move || {
                window
                    .remove_event_listener_with_callback(
                        "resize",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        });
    }
 
    {
        let centro = centro.clone();
        let zoom = zoom.clone();
        let api_key = api_key.clone();
        let project_state = props.project_state.clone();
        let window_size = window_size.clone();

        use_effect_with(
            (project_state.clone(), *window_size),
            move |_| {
                if let Some(project) = &*project_state {
                    let (width, height) = *window_size;
                    let c = project.coordinates.centro;
                    let z = calculate_zoom(
                        &project.coordinates,
                        (width * 1.25) - 400.0,
                        (height * 1.25) - 70.0,
                    );
 
                    // console::log_1(&format!("zoom: {:?}", z).into());
                    centro.set(Some(c));
                    zoom.set(Some(z));
                    api_key.set(Some(project.maptiler_api_key.clone()));
                }
                || ()
            },
        );
    }

    html! {<>
        <div class="absolute w-full h-full scale-110 opacity-20 no-interaction">
            {
                if let (Some((lat, lng)), Some(z), Some(key)) = (*centro, *zoom, (*api_key).clone()) {
                    html! {
                        <iframe
                            width="100%"
                            height="100%"
                            allow="geolocation"
                            src={format!(
                                "https://api.maptiler.com/maps/backdrop/?key={}#{}/{}/{}",
                                key,    // antes era: EVEAYM1Cx9nGoDR5OVX6
                                z, lat, lng
                            )}
                        />
                    }
                } else {
                    Html::default()
                }
            }
            
        </div>
        <div class="absolute bottom-2 text-xs left-2 italic text-white/60">
                {"La zona a relevar puede ser mas grande que la representada en el mapa."}
        </div>
    </>}
}


fn lon_to_x(lon: f64) -> f64 {
    EARTH_RADIUS * lon.to_radians()
}

fn lat_to_y(lat: f64) -> f64 {
    let lat = lat.clamp(-85.05112878, 85.05112878);
    EARTH_RADIUS
        * ((std::f64::consts::PI / 4.0 + lat.to_radians() / 2.0)
            .tan())
        .ln()
}

pub fn calculate_zoom(
    corners: &GeoCorners,
    width_px: f64,
    height_px: f64,
) -> f64 {
    const TILE_SIZE: f64 = 512.0;
    const WORLD_SIZE: f64 =
        2.0 * std::f64::consts::PI * EARTH_RADIUS;

    let min_lon = corners.sup_izq.0.min(corners.inf_izq.0);
    let max_lon = corners.sup_der.0.max(corners.inf_der.0);

    let min_lat = corners.inf_izq.1.min(corners.inf_der.1);
    let max_lat = corners.sup_izq.1.max(corners.sup_der.1);

    let min_x = lon_to_x(min_lon);
    let max_x = lon_to_x(max_lon);

    let min_y = lat_to_y(min_lat);
    let max_y = lat_to_y(max_lat);

    let bbox_width_m = (max_x - min_x).abs();
    let bbox_height_m = (max_y - min_y).abs();

    let resolution_x = bbox_width_m / width_px;
    let resolution_y = bbox_height_m / height_px;

    let resolution = resolution_x.max(resolution_y);

    (WORLD_SIZE / (TILE_SIZE * resolution)).log2()
}