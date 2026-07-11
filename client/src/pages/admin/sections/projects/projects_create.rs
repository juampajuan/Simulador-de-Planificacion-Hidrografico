use crate::components::form_inputs::FormInput;
use crate::components::modal::Modal;
use crate::pages::admin::sections::projects::projects_fields::ProjectFormFields;
use crate::pages::admin::sections::projects::projects_params_form::ProjectParamsForm;
use crate::services::requests::create_project;
use crate::structs::project::{NewProject, Project};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CreateProjectModalProps {
    pub is_open: UseStateHandle<bool>,
    pub projects_state: UseStateHandle<Vec<Project>>,
}

/// Modal de alta de proyecto: nombre, descripción, GeoTIFF, Modo Entrega y parámetros iniciales.
#[function_component(CreateProjectModal)]
pub fn create_project_modal(props: &CreateProjectModalProps) -> Html {
    let name_input = use_state(String::new);
    let description_input = use_state(String::new);
    let selected_file = use_state(|| None::<web_sys::File>);
    let exam_mode_input = use_state(|| false);
    let due_date_input = use_state(String::new);

    let form_fields_state = use_state(ProjectFormFields::new_empty);

    let error_msg = use_state(String::new);
    let modal_loading = use_state(|| false);

    let on_submit = {
        let is_open = props.is_open.clone();
        let projects_state = props.projects_state.clone();
        let file_opt = (*selected_file).clone();
        let name = (*name_input).clone();
        let description = (*description_input).clone();
        let is_exam = *exam_mode_input;
        let due_date = (*due_date_input).clone();

        let fields = form_fields_state.clone();
        let error_msg = error_msg.clone();
        let modal_loading = modal_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let file = match &file_opt {
                Some(f) => f.clone(),
                None => {
                    error_msg
                        .set("Tenés que seleccionar un archivo GeoTIFF obligatorio".to_string());
                    return;
                }
            };
            if name.is_empty() {
                error_msg.set("El nombre del proyecto es obligatorio".to_string());
                return;
            }

            // Validación de la fecha límite solo si el Modo Entrega está activo
            let due_date_opt = if is_exam {
                if due_date.is_empty() {
                    error_msg.set(
                        "La fecha límite de entrega es obligatoria para el modo examen".to_string(),
                    );
                    return;
                }
                Some(due_date.clone())
            } else {
                None
            };

            let attempts = match fields.attempts_limit.parse::<i64>() {
                Ok(n) => {
                    if n <= 0 {
                        error_msg.set("El límite de intentos debe ser mayor a cero".to_string());
                        return;
                    }
                    n
                }
                Err(_) => {
                    error_msg.set("El límite de intentos debe ser un número entero".to_string());
                    return;
                }
            };
            let b = match fields.budget.parse::<f64>() {
                Ok(n) => n,
                Err(_) => {
                    error_msg.set("El presupuesto debe ser un número válido".to_string());
                    return;
                }
            };
            let mind = match fields.min_depth.parse::<f64>() {
                Ok(n) => n,
                Err(_) => {
                    error_msg.set("La profundidad mínima debe ser un número válido".to_string());
                    return;
                }
            };
            let maxd = match fields.max_depth.parse::<f64>() {
                Ok(n) => n,
                Err(_) => {
                    error_msg.set("La profundidad máxima debe ser un número válido".to_string());
                    return;
                }
            };
            if mind >= maxd {
                error_msg.set("La profundidad máxima debe ser mayor a la mínima".to_string());
                return;
            }

            error_msg.set(String::new());

            create_project(
                NewProject {
                    name: name.clone(),
                    description: description.clone(),
                    file,
                    attempts_limit: attempts,
                    exam_mode: is_exam,
                    due_date: due_date_opt,
                    weather: fields.weather.clone(),
                    seabed_hardness: fields.seabed_hardness.clone(),
                    budget: b,
                    geotiff_min_depth: mind,
                    geotiff_max_depth: maxd,
                },
                projects_state.clone(),
                error_msg.clone(),
                modal_loading.clone(),
            );

            is_open.set(false);
        })
    };

    let is_open_close = props.is_open.clone();
    let is_open_cancel = props.is_open.clone();
    let exam_mode_clone = exam_mode_input.clone();
    let due_date_clone = due_date_input.clone();

    html! {
        <Modal
            title="Crear Nuevo Proyecto"
            subtitle="Ingresá los detalles e inicializá el archivo geográfico del entorno."
            on_close={Callback::from(move |_| is_open_close.set(false))}
            max_width_class={Some("max-w-2xl".to_string())}
        >

            <form onsubmit={on_submit} class="space-y-4">

                if !error_msg.is_empty() {
                    <div class="p-2 bg-red-500/15 border border-red-500/20 rounded-lg text-red-400 text-xs font-semibold flex items-center gap-1.5">
                        <span>{ &*error_msg }</span>
                    </div>
                }

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <FormInput label="Nombre del Proyecto" value={(*name_input).clone()}
                        oninput={Callback::from(move |e: InputEvent| name_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />

                    <div class="flex flex-col space-y-1">
                        <label class="text-xs font-semibold text-white/80">{"Archivo GeoTIFF"}</label>
                        <input type="file" accept=".tif,.tiff" class="bg-slate-950 text-sm p-2 rounded-lg border border-white/10 focus:border-cyan-400 focus:outline-none file:mr-4 file:py-1 file:px-3 file:rounded-full file:border-0 file:text-xs file:font-semibold file:bg-cyan-200 file:text-black file:cursor-pointer"
                            onchange={Callback::from(move |e: Event| {
                                if let Some(files) = e.target_unchecked_into::<web_sys::HtmlInputElement>().files()
                                    && let Some(f) = files.get(0) { selected_file.set(Some(f)); }
                            })} />
                    </div>
                </div>

                <div class="flex flex-col space-y-1">
                    <label class="text-xs font-semibold text-white/80">{"Descripción (Opcional)"}</label>
                    <textarea value={(*description_input).clone()} rows="2" class="bg-slate-950 text-sm p-2.5 rounded-lg border border-white/10 focus:border-cyan-400 focus:outline-none resize-none"
                        oninput={Callback::from(move |e: InputEvent| description_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="flex flex-col space-y-1 select-none">
                        <label class="text-xs font-semibold text-white/80 mb-1">{"Modo de Trabajo"}</label>
                        <div class="flex items-center gap-3 p-2 bg-slate-950 rounded-lg border border-white/10 focus-within:border-cyan-400 h-[38px] transition-colors">
                            <input
                                id="exam_mode_checkbox"
                                type="checkbox"
                                checked={*exam_mode_input}
                                onchange={Callback::from(move |e: Event| {
                                    let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    exam_mode_clone.set(target.checked());
                                    if !target.checked() { due_date_clone.set(String::new()); }
                                })}
                                class="w-4 h-4 rounded text-cyan-400 bg-slate-900 border-white/20 focus:ring-0 focus:ring-offset-0 accent-cyan-200 cursor-pointer ml-1"
                            />
                            <label for="exam_mode_checkbox" class="text-sm text-white/90 cursor-pointer pt-0.5">
                                {"Habilitar Modo Entrega"}
                            </label>
                        </div>
                    </div>

                    <div class="flex flex-col space-y-1">
                        <label class="text-xs font-semibold text-white/80 mb-1">{"Fecha Límite de Entrega"}</label>
                        <input
                            type="date"
                            value={(*due_date_input).clone()}
                            disabled={!*exam_mode_input}
                            oninput={Callback::from(move |e: InputEvent| due_date_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                            class="bg-slate-950 text-sm p-2 rounded-lg border border-white/10 focus:border-cyan-400 focus:outline-none text-white w-full h-[38px] font-mono scheme-dark disabled:opacity-40 disabled:cursor-not-allowed disabled:border-white/5 transition-all cursor-pointer"
                        />
                    </div>
                </div>

                <div class="border-t border-white/5 my-2" />
                <div class="text-xs font-bold text-cyan-300 uppercase tracking-wider">{"Parámetros Iniciales de Simulación"}</div>

                <ProjectParamsForm form_state={form_fields_state} exam_mode={*exam_mode_input} />

                <div class="flex justify-end gap-3 pt-4">
                    <button type="button" onclick={move |_| is_open_cancel.set(false)} class="px-4 py-2 text-sm font-medium bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 transition-colors cursor-pointer">{"Cancelar"}</button>
                    <button type="submit" class="px-4 py-2 text-sm font-semibold bg-cyan-200 text-black/90 rounded-lg hover:bg-cyan-300 transition-colors cursor-pointer">{"Crear Proyecto"}</button>
                </div>
            </form>
        </Modal>
    }
}
