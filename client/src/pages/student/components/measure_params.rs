use yew::prelude::*;

#[function_component(MeasuresParams)]
pub fn measures_params() -> Html {
    let input_cls = "rounded p-2 text-black dark:bg-zinc-700 dark:text-white";
    let echo_params = [
        "Profundidad mínima", "Profundidad máxima", "Intervalo de repetición del pulso",
        "Frecuencia", "Velocidad del sonido", "Longitud del pulso",
        "Potencia transmitida", "Ganancia", "Umbral"
    ];

    html! {
        <>
            <div class="border-b border-dashed border-white/40 p-3 flex flex-col">
                <label class="font-semibold">{"Embarcación"}</label>
                <input type="text" placeholder="Seleccione la embarcacion" class={input_cls} />
                { for ["Uso de monógrafo", "Uso de perfilador de sonido", "Uso de sensor inercial"].iter().map(|lab| html! {
                    <label class="flex items-center gap-2"><input type="checkbox"/>{ lab }</label>
                })}
            </div>

            <div class="flex flex-col p-3">
                <label class="mb-2 font-semibold">{"GNSS"}</label>
                <select class={input_cls}>
                    { for ["Corrección de Fase", "Corrección DGPS", "Sin corrección"].iter().map(|opt| html! {
                        <option>{ opt }</option>
                    })}
                </select>
            </div>

            <div class="p-3">
                <h3 class="text-2xl font-bold mb-4">{"Parámetros de Ecosonda"}</h3>
                <div class="flex flex-col gap-4">
                    { for echo_params.iter().map(|lp| html! {
                        <input type="number" placeholder={*lp} class={input_cls} />
                    })}
                </div>
            </div>
        </>
    }
}