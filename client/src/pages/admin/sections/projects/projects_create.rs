use yew::prelude::*;
use crate::structs::project::{NewProject, Project};
use crate::components::modal::Modal;
use crate::services::requests::create_project;
use crate::components::form_inputs::FormInput;
use crate::pages::admin::sections::projects::projects_fields::ProjectFormFields;
use crate::pages::admin::sections::projects::projects_params_form::ProjectParamsForm;

#[derive(Properties, PartialEq)]
pub struct CreateProjectModalProps {
    pub is_open: UseStateHandle<bool>,
    pub projects_state: UseStateHandle<Vec<Project>>,
}

/// Modal de alta de proyecto: nombre, descripción, GeoTIFF y parámetros iniciales.
#[function_component(CreateProjectModal)]
pub fn create_project_modal(props: &CreateProjectModalProps) -> Html {
    let name_input = use_state(String::new);
    let description_input = use_state(String::new);
    let selected_file = use_state(|| None::<web_sys::File>); 
    
    let form_fields_state = use_state(ProjectFormFields::new_empty);

    let error_msg = use_state(String::new);
    let modal_loading = use_state(|| false); 

    let on_submit = {
        let is_open = props.is_open.clone();
        let projects_state = props.projects_state.clone();
        let file_opt = (*selected_file).clone();
        let name = (*name_input).clone();
        let description = (*description_input).clone();
        
        let fields = form_fields_state.clone();
        let error_msg = error_msg.clone();
        let modal_loading = modal_loading.clone(); 

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let file = match &file_opt {
                Some(f) => f.clone(),
                None => { error_msg.set("Tenés que seleccionar un archivo GeoTIFF obligatorio".to_string()); return; }
            };
            if name.is_empty() { 
                error_msg.set("El nombre del proyecto es obligatorio".to_string()); 
                return; 
            }

            let attempts = match fields.attempts_limit.parse::<i64>() {
                Ok(n) => { 
                    if n <= 0 {
                        error_msg.set("El límite de intentos debe ser mayor a cero".to_string()); 
                        return;
                    }
                    n
                }
                Err(_) => { error_msg.set("El límite de intentos debe ser un número entero".to_string()); return; }
            };
            let b = match fields.budget.parse::<f64>() {
                Ok(n) => n,
                Err(_) => { error_msg.set("El presupuesto debe ser un número válido".to_string()); return; }
            };
            let mind = match fields.min_depth.parse::<f64>() {
                Ok(n) => n,
                Err(_) => { error_msg.set("La profundidad mínima debe ser un número válido".to_string()); return; }
            };
            let maxd = match fields.max_depth.parse::<f64>() {
                Ok(n) => n,
                Err(_) => { error_msg.set("La profundidad máxima debe ser un número válido".to_string()); return; }
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

    let file_name_preview = selected_file.as_ref().map(|f| f.name()).unwrap_or_default();
    let is_open_close = props.is_open.clone();
    let is_open_cancel = props.is_open.clone();

    html! {
        <Modal 
            title="Crear Nuevo Proyecto" 
            on_close={Callback::from(move |_| is_open_close.set(false))}
            max_width_class={Some("max-w-2xl".to_string())}
        >
            <p class="text-xs text-white/60 -mt-3 mb-4">{"Ingresá los detalles e inicializá el archivo geográfico del entorno."}</p>

            <form onsubmit={on_submit} class="space-y-4">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <FormInput label="Nombre del Proyecto" value={(*name_input).clone()}
                        oninput={Callback::from(move |e: InputEvent| name_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                    
                    <div class="flex flex-col space-y-1">
                        <label class="text-xs font-semibold text-white/80">{"Seleccionar Archivo GeoTIFF"}</label>
                        <input type="file" accept=".tif,.tiff" class="bg-slate-950 text-sm p-2 rounded-lg border border-white/10 focus:border-cyan-400 focus:outline-none file:mr-4 file:py-1 file:px-3 file:rounded-full file:border-0 file:text-xs file:font-semibold file:bg-cyan-200 file:text-black file:cursor-pointer" 
                            onchange={Callback::from(move |e: Event| {
                                if let Some(files) = e.target_unchecked_into::<web_sys::HtmlInputElement>().files() 
                                    && let Some(f) = files.get(0) { selected_file.set(Some(f)); }
                            })} />
                        if !file_name_preview.is_empty() { 
                            <p class="text-[10px] text-cyan-300 truncate mt-0.5">{"Seleccionado: "}{file_name_preview}</p> 
                        }
                    </div>
                </div>

                <div class="flex flex-col space-y-1">
                    <label class="text-xs font-semibold text-white/80">{"Descripción"}</label>
                    <textarea value={(*description_input).clone()} rows="2" class="bg-slate-950 text-sm p-2.5 rounded-lg border border-white/10 focus:border-cyan-400 focus:outline-none resize-none"
                        oninput={Callback::from(move |e: InputEvent| description_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                </div>

                <div class="border-t border-white/5 my-2" />
                <div class="text-xs font-bold text-cyan-300 uppercase tracking-wider">{"Parámetros Iniciales de Simulación"}</div>

                <ProjectParamsForm form_state={form_fields_state} />

                if !error_msg.is_empty() { 
                    <div class="mt-4 p-2.5 bg-red-500/10 border border-red-500/20 rounded-lg text-red-400 text-xs font-semibold flex items-center gap-1.5">
                        <span>{ &*error_msg }</span>
                    </div>
                }

                <div class="flex justify-end gap-3 pt-4 border-t border-white/5">
                    <button type="button" onclick={move |_| is_open_cancel.set(false)} class="px-4 py-2 text-sm font-medium bg-white/5 border border-white/10 rounded-xl hover:bg-white/10 transition-colors cursor-pointer">{"Cancelar"}</button>
                    <button type="submit" class="px-4 py-2 text-sm font-semibold bg-cyan-200 text-black/90 rounded-xl hover:bg-cyan-300 transition-colors cursor-pointer">{"Crear Proyecto"}</button>
                </div>
            </form>
        </Modal>
    }
}