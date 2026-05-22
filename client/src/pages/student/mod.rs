use yew::prelude::*; 
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request; 
use crate::components::root::{Root};
pub mod components;
use self::components::img_viewer::IMGviewer;
use self::components::parameters_cont::ParamCont;

#[function_component(StudentPage)]
pub fn student_page() -> Html {

    let mensaje = use_state(|| {
        "Seleccione parametros para el recorrido".to_string()
    });

    let image_url = use_state(|| None::<String>);

    let separacion = use_state(|| "".to_string());
    let azimut = use_state(|| "".to_string());

     

    html! {

        <Root title={"Simulador"}>

            // Panel izquierdo
            <ParamCont>

                // Recorrido
                <div class="border-b border-dashed border-white/40 p-3">

                    <h3 class="mb-4">
                        { "Parámetros de Recorrido" }
                    </h3>

                    <div class="flex flex-col gap-4">

                        // Separación
                        <input
                            type="number"
                            placeholder="Separación (mts)"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                            value={(*separacion).clone()}
                            onchange={{
                                let separacion = separacion.clone();
                                let azimut = azimut.clone();
                                let mensaje = mensaje.clone();
                                let image_url = image_url.clone();

                                Callback::from(move |e: Event| {

                                    let input: web_sys::HtmlInputElement =
                                        e.target_unchecked_into();

                                    let value = input.value();

                                    separacion.set(value.clone());

                                    let az = (*azimut).clone();

                                    if !value.is_empty() && !az.is_empty() {
                                        mensaje.set(
                                                "Generando recorrido...".to_string()
                                        );

                                        let mensaje = mensaje.clone();
                                        let image_url = image_url.clone();

                                        spawn_local(async move {

                                            let body = format!(
                                                r#"{{
                                                    "separacion": {},
                                                    "azimut": {}
                                                }}"#,
                                                value,
                                                az
                                            );

                                            let response = Request::post(
                                                "http://localhost:3000/api/v1/create_path"
                                            )
                                            .header("Content-Type", "application/json")
                                            .body(body)
                                            .unwrap()
                                            .send()
                                            .await
                                            .unwrap();

                                            let bytes = response
                                                .binary()
                                                .await
                                                .unwrap();

                                            let uint8_array =
                                                js_sys::Uint8Array::from(bytes.as_slice());

                                            let array = js_sys::Array::new();

                                            array.push(&uint8_array.buffer());

                                            let blob =
                                                web_sys::Blob::new_with_u8_array_sequence(
                                                    &array
                                                ).unwrap();

                                            let url =
                                                web_sys::Url::create_object_url_with_blob(
                                                    &blob
                                                ).unwrap();

                                            image_url.set(Some(url));

                                            mensaje.set(
                                                "Imagen generada".to_string()
                                            );
                                        });
                                    }
                                })
                            }}
                        />

                        // Azimut
                        <input
                            type="number"
                            placeholder="Azimut"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                            value={(*azimut).clone()}
                            onchange={{
                                let azimut = azimut.clone();
                                let separacion = separacion.clone();
                                let mensaje = mensaje.clone();
                                let image_url = image_url.clone();

                                Callback::from(move |e: Event| {

                                    let input: web_sys::HtmlInputElement =
                                        e.target_unchecked_into();

                                    let value = input.value();

                                    azimut.set(value.clone());

                                    let sep = (*separacion).clone();

                                    if !value.is_empty() && !sep.is_empty() {

                                        let mensaje = mensaje.clone();
                                        let image_url = image_url.clone();

                                        spawn_local(async move {
                                            mensaje.set(
                                                "Generando recorrido...".to_string()
                                            );

                                            let body = format!(
                                                r#"{{
                                                    "separacion": {},
                                                    "azimut": {}
                                                }}"#,
                                                sep,
                                                value
                                            );

                                            let response = Request::post(
                                                "http://localhost:3000/api/v1/create_path"
                                            )
                                            .header("Content-Type", "application/json")
                                            .body(body)
                                            .unwrap()
                                            .send()
                                            .await
                                            .unwrap();

                                            let bytes = response
                                                .binary()
                                                .await
                                                .unwrap();

                                            let uint8_array =
                                                js_sys::Uint8Array::from(bytes.as_slice());

                                            let array = js_sys::Array::new();

                                            array.push(&uint8_array.buffer());

                                            let blob =
                                                web_sys::Blob::new_with_u8_array_sequence(
                                                    &array
                                                ).unwrap();

                                            let url =
                                                web_sys::Url::create_object_url_with_blob(
                                                    &blob
                                                ).unwrap();

                                            image_url.set(Some(url));

                                            mensaje.set(
                                                "Imagen generada".to_string()
                                            );
                                        });
                                    }
                                })
                            }}
                        />

                    </div>

                </div>

                <div class="border-b border-dashed border-white/40 p-3">
                
                    <div class="flex flex-col">

                        <label class=" font-semibold">
                            { "Embarcación" }
                        </label>

                        // Si son fijas, deberia de ser un select
                        <input
                            type="text"
                            placeholder="Seleccione la embarcacion"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                        <label class="flex items-center gap-2">
                        <input type="checkbox"/>
                        { "Uso de monógrafo" }
                        </label>

                        <label class="flex items-center gap-2">
                            <input type="checkbox"/>
                            { "Uso de perfilador de sonido" }
                        </label>

                        <label class="flex items-center gap-2">
                            <input type="checkbox"/>
                            { "Uso de sensor inercial" }
                        </label>

                    </div>

                </div>
 
                // GNSS
                <div class="flex flex-col p-3">

                    <label class="mb-2 font-semibold">
                        { "GNSS" }
                    </label>

                    <select class="
                        rounded
                        p-2
                        text-black
                        dark:bg-zinc-700
                        dark:text-white
                    ">

                        <option>
                            { "Sin corrección" }
                        </option>

                        <option>
                            { "Corrección DGPS" }
                        </option>

                        <option>
                            { "Corrección de Fase" }
                        </option>

                    </select>

                </div>

                // Ecosonda
                <div class="p-3">

                    <h3 class="text-2xl font-bold mb-4">
                        { "Parámetros de Ecosonda" }
                    </h3>

                    <div class="flex flex-col gap-4">

                        <input
                            type="number"
                            placeholder="Profundidad mínima"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                        <input
                            type="number"
                            placeholder="Profundidad máxima"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                        <input
                            type="number"
                            placeholder="Intervalo de repetición del pulso"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                        <input
                            type="number"
                            placeholder="Frecuencia"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                        <input
                            type="number"
                            placeholder="Velocidad del sonido"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                        <input
                            type="number"
                            placeholder="Longitud del pulso"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                        <input
                            type="number"
                            placeholder="Potencia transmitida"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                        <input
                            type="number"
                            placeholder="Ganancia"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                        <input
                            type="number"
                            placeholder="Umbral"
                            class="
                                rounded
                                p-2
                                text-black
                                dark:bg-zinc-700
                                dark:text-white
                            "
                        />

                    </div>

                </div>
 

            </ParamCont>

            <IMGviewer
                image_url={(*image_url).clone()}
                mensaje={(*mensaje).clone()}
            />
 
 
        </Root>
    }
}