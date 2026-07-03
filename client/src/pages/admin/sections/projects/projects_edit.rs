use yew::prelude::*;
use crate::structs::project::Project;
use lucide_yew::Save;
use crate::pages::admin::sections::projects::projects_fields::ProjectFormFields;
use crate::pages::admin::sections::projects::projects_params_form::ProjectParamsForm;


#[derive(Properties, PartialEq)]
pub struct ProjectEditProps {
    pub project_state: Project,
    pub projects_state: UseStateHandle<Vec<Project>>, 
    pub on_save: Callback<Project>,
}

/// Formulario de edición de los parámetros de un proyecto existente.
#[function_component(ProjectEdit)]
pub fn project_edit(props: &ProjectEditProps) -> Html {
    let form_state = use_state(|| ProjectFormFields::from_project(&props.project_state));
    let error_msg = use_state(String::new);

    let is_exam = props.project_state.exam_mode; 

    let on_submit = {
        let on_save = props.on_save.clone();
        let project_base = props.project_state.clone();
        let fields = form_state.clone();
        let error_msg = error_msg.clone();

        Callback::from(move |_| {
            let b = match fields.budget.parse::<f64>() { Ok(n) => n, Err(_) => { error_msg.set("Presupuesto inválido".into()); return; }};
            let mind = match fields.min_depth.parse::<f64>() { Ok(n) => n, Err(_) => { error_msg.set("Prof. mínima inválida".into()); return; }};
            let maxd = match fields.max_depth.parse::<f64>() { Ok(n) => n, Err(_) => { error_msg.set("Prof. máxima inválida".into()); return; }};
            if mind >= maxd { error_msg.set("La profundidad máxima debe ser mayor a la mínima".to_string()); return; }
            
            let attempts = match fields.attempts_limit.parse::<i64>() { 
                Ok(n) => {
                    if n <= 0 {
                        error_msg.set("El límite de intentos debe ser mayor a cero".to_string());
                        return;
                    }
                    n
                } 
                Err(_) => { error_msg.set("Intentos inválidos".into()); return; }
            };

            error_msg.set(String::new());

            let mut updated_project = Project {
                attempts_limit: attempts,
                weather: fields.weather.clone(),
                seabed_hardness: fields.seabed_hardness.clone(),
                budget: b,
                geotiff_min_depth: mind,
                geotiff_max_depth: maxd,
                ..project_base.clone()
            };
            updated_project.exam_mode = is_exam; 
            on_save.emit(updated_project);
        })
    };

    html! {
        <div class="space-y-4">
            <div class="flex justify-between items-center">
                <div class="text-xs font-bold text-cyan-300 uppercase tracking-wider">
                    { "Editar Parámetros de Proyecto" }
                </div>
            </div>

            <ProjectParamsForm form_state={form_state} exam_mode={is_exam} />

            if !error_msg.is_empty() {
                <p class="text-red-400 text-xs font-medium">{ &*error_msg }</p>
            }

            <div class="flex justify-end pt-2">
                <button onclick={on_submit} class="flex items-center gap-2 bg-cyan-200 hover:bg-cyan-300 text-black font-semibold text-xs px-3 py-2 rounded transition-colors cursor-pointer">
                    <Save size={18}/>
                    {"Guardar Cambios"}
                </button>
            </div>
        </div>
    }
}