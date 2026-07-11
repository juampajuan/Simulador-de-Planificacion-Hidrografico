use crate::services::requests::GeoCorners;
use crate::services::requests::StudentProjectResponse;
use gloo_timers::callback::Timeout;
use wasm_bindgen::{JsCast, closure::Closure};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MapBackgroundProps {
    pub project_state: UseStateHandle<Option<StudentProjectResponse>>,
}

// Muestra el mapa real de fondo.
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

                    let width = window.inner_width().unwrap().as_f64().unwrap();
                    let height = window.inner_height().unwrap().as_f64().unwrap();

                    window_size.set((width, height));
                });

                *resize_timeout.borrow_mut() = Some(timeout);
            });

            let window = web_sys::window().unwrap();

            window
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();

            move || {
                window
                    .remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
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

        use_effect_with((project_state.clone(), *window_size), move |_| {
            if let Some(project) = &*project_state {
                let (width, height) = *window_size;
                let c = project.coordinates.centro;
                let z = calculate_zoom(
                    &project.coordinates,
                    (width * 1.0) - 400.0,
                    (height * 1.0) - 150.0,
                );

                // console::log_1(&format!("zoom: {:?}", z).into());
                centro.set(Some(c));
                zoom.set(Some(z));
                api_key.set(Some(project.maptiler_api_key.clone()));
            }
            || ()
        });
    }

    html! {<>
        <div class="absolute w-full h-full opacity-20 no-interaction">
            {
                if let (Some((lat, lng)), Some(z), Some(key)) = (*centro, *zoom, (*api_key).clone()) {
                    html! {
                        <iframe
                            class="absolute inset-0 w-[calc(100%+390px)] h-full"
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
        <div class="absolute bottom-2 text-xs right-3 italic text-white/70">
                {"La zona a relevar puede ser más grande que la representada en el mapa."}
        </div>
    </>}
}

const TILE_SIZE: f64 = 512.0;

fn lat_rad(lat: f64) -> f64 {
    let sin = (lat.to_radians()).sin();
    ((1.0 + sin) / (1.0 - sin)).ln() / 2.0
}

/// Calcula el nivel de zoom del mapa según el tamaño real de la zona.
pub fn calculate_zoom(corners: &GeoCorners, viewport_width: f64, viewport_height: f64) -> f64 {
    let padding = 0.0;

    let width = viewport_width * (1.0 - 2.0 * padding);
    let height = viewport_height * (1.0 - 2.0 * padding);

    let min_lon = corners.sup_izq.0.min(corners.inf_izq.0);
    let max_lon = corners.sup_der.0.max(corners.inf_der.0);

    let min_lat = corners.inf_izq.1.min(corners.inf_der.1);
    let max_lat = corners.sup_izq.1.max(corners.sup_der.1);

    let lon_fraction = ((max_lon - min_lon).abs() / 360.0).max(1e-12);

    let lat_fraction = {
        let top = lat_rad(max_lat);
        let bottom = lat_rad(min_lat);
        ((top - bottom).abs() / (2.0 * std::f64::consts::PI)).max(1e-12)
    };

    let zoom_x = (width / TILE_SIZE / lon_fraction).log2();
    let zoom_y = (height / TILE_SIZE / lat_fraction).log2();

    zoom_x.min(zoom_y) + 0.7
}
