use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use web_sys::{window, HtmlElement};
use js_sys::{Array, Uint8Array};

#[function_component(StudentPage)]
pub fn student_page() -> Html {

    let mensaje = use_state(|| {
        "Seleccione parametros para el recorrido".to_string()
    });

    let image_url = use_state(|| None::<String>);

    let separacion = use_state(|| "".to_string());
    let azimut = use_state(|| "".to_string());

    let dark_mode = use_state(|| false);

    // Toggle dark mode
    let toggle_dark_mode = {
        let dark_mode = dark_mode.clone();

        Callback::from(move |_| {

            let document = window()
                .unwrap()
                .document()
                .unwrap();

            let html = document
                .document_element()
                .unwrap();

            let html: HtmlElement =
                html.dyn_into().unwrap();

            let class_list = html.class_list();

            if *dark_mode {
                class_list.remove_1("dark").unwrap();
                dark_mode.set(false);

            } else {
                class_list.add_1("dark").unwrap();
                dark_mode.set(true);
            }
        })
    };

    html! {

        <div class="
            min-h-screen
            w-full
            bg-stone-100
            dark:bg-zinc-900
            flex
            flex-col
            p-6
            transition-colors
            duration-300
            relative
        ">

            // Botón dark mode
            <button
                class="
                    absolute
                    top-6
                    right-6
                    text-3xl
                    p-3
                    rounded-full
                    hover:bg-black/10
                    dark:hover:bg-white/10
                    transition
                "
                onclick={toggle_dark_mode}
            >
                {
                    if *dark_mode {
                        "☀️"
                    } else {
                        "🌙"
                    }
                }
            </button>

            // Título
            <div class="w-full flex justify-center mb-8">

                <h1 class="
                    text-5xl
                    font-bold
                    text-green-900
                    dark:text-green-400
                    transition-colors
                ">
                    { "Simulador" }
                </h1>

            </div>

            // Contenido principal
            <div class="flex flex-1 gap-6">

                // Panel izquierdo
                <div class="
                    w-[420px]
                    bg-green-900
                    dark:bg-zinc-800
                    text-white
                    rounded-xl
                    p-6
                    shadow-lg
                    overflow-y-auto
                    transition-colors
                ">

                    <h2 class="text-3xl font-bold mb-6 text-center">
                        { "Parámetros" }
                    </h2>

                    <div class="flex flex-col gap-5">

                        // Recorrido
                        <div class="mt-4">

                            <h3 class="text-2xl font-bold mb-4 text-center">
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

                        // Embarcación
                        <div class="flex flex-col">

                            <label class="mb-1 font-semibold">
                                { "Embarcación" }
                            </label>

                            <input
                                type="text"
                                placeholder="Nombre"
                                class="
                                    rounded
                                    p-2
                                    text-black
                                    dark:bg-zinc-700
                                    dark:text-white
                                "
                            />

                        </div>

                        // Sensores
                        <div class="flex flex-col gap-3">

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

                        // GNSS
                        <div class="flex flex-col">

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
                        <div class="mt-4">

                            <h3 class="text-2xl font-bold mb-4 text-center">
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

                    </div>

                </div>

                <div class="
                    flex-1
                    bg-green-900
                    dark:bg-zinc-800
                    rounded-xl
                    shadow-lg
                    flex
                    items-center
                    justify-center
                    overflow-hidden
                    transition-colors
                ">

                    {
                        if let Some(url) = &*image_url {

                            html! {
                                <img
                                    src={url.clone()}
                                    class="w-full h-full object-contain"
                                />
                            }

                        } else {

                            html! {
                                <h2 class="text-2xl font-bold text-white text-center p-8">
                                    { (*mensaje).clone() }
                                </h2>
                            }
                        }
                    }

                </div>

            </div>

        </div>
    }
}