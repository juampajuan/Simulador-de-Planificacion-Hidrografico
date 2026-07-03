use yew::prelude::*;
use crate::components::subtitle::Subtitle;
use crate::structs::project::Project;
use lucide_yew::{FolderOpenDot, Plus};
use crate::services::requests::get_all_projects;
use crate::pages::admin::sections::projects::projects_row::ProjectRow;
use crate::pages::admin::sections::projects::projects_create::CreateProjectModal;

/// Sección de proyectos: lista los del profesor y permite crear uno nuevo.
#[function_component(AdminProjects)]
pub fn admin_projects() -> Html {
    let projects = use_state(Vec::<Project>::new);
    let ui_mensaje = use_state(String::new);
    let ui_loading = use_state(|| false);
    let is_modal_open = use_state(|| false);

    {
        let projects = projects.clone();
        let ui_mensaje = ui_mensaje.clone();
        let ui_loading = ui_loading.clone();
        
        use_effect_with((), move |_| {
            get_all_projects(projects, ui_mensaje, ui_loading);
            || ()
        });
    }

    let on_click_add = {
        let is_modal_open = is_modal_open.clone();
        Callback::from(move |_| is_modal_open.set(true))
    };

    html! {<> 
        <div class="text-white flex justify-between p-2 pr-1">
            <div class="space-y-1">
                <Subtitle
                    text={"Todos los proyectos"}
                    icon={html! {
                        <FolderOpenDot size={24}/>
                    }}
                />
                <p class="text-white/70 text-xs">{"Aca podes administrar los proyectos que luego se podran asignar a los alumnos o grupos."}</p> 
            </div>
            <div>
                <button 
                    onclick={on_click_add}
                    class="flex items-center px-3 py-3 gap-2 bg-cyan-200 text-black/90 rounded-full hover:bg-cyan-300 transition-colors cursor-pointer"
                >
                    <Plus size={18}/>
                    <p class="text-xs font-semibold pt-0.5">{"Agregar"}</p>
                </button>
            </div>
        </div>
 
        <div class="h-full overflow-y-auto pb-16 mt-4">
            if *ui_loading {
                <p class="text-cyan-300 text-xs animate-pulse">{"Cargando proyectos..."}</p>
            } else if !ui_mensaje.is_empty() {
                <p class="text-red-400 text-xs">{ &*ui_mensaje }</p>
            }
            <ProjectsTable projects_state={projects.clone()}/>
        </div>

        if *is_modal_open {
            <CreateProjectModal 
                is_open={is_modal_open.clone()} 
                projects_state={projects.clone()} 
            />
        }
    </>}
}

#[derive(Properties, PartialEq)]
pub struct ProjectsTableProps {
    pub projects_state: UseStateHandle<Vec<Project>>,
}

/// Tabla de proyectos: arma el encabezado y renderiza una fila (`ProjectRow`) por proyecto.
#[function_component(ProjectsTable)]
pub fn projects_table(props: &ProjectsTableProps) -> Html {
    html! {
        <div class="overflow-x-auto text-white w-full">
            <table class="w-full table-fixed border-separate border-spacing-y-2">
                <thead class="text-xs bg-slate-950 shadow-[inset_0_0_0_1px_rgba(255,255,255,0.2)] rounded-lg">
                    <tr>
                        <th class="w-[5%] px-4 py-2 text-left rounded-l-lg">{"#"}</th>
                        <th class="w-[30%] px-4 py-2 text-left">{"Nombre / Descripción"}</th>
                        <th class="w-[15%] px-4 py-2 text-left">{"Modo"}</th>
                        <th class="w-[15%] px-4 py-2 text-left">{"Límite de intentos"}</th>
                        <th class="w-[25%] px-4 py-2 text-left">{"Archivo GeoTIFF"}</th>
                        <th class="w-[10%] px-4 py-2 text-end rounded-r-lg">{"Acciones"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        props.projects_state.iter().enumerate().map(|(index, item)| {
                            html! {
                                <ProjectRow 
                                    key={item.id}
                                    row_number={index + 1}
                                    project={item.clone()} 
                                    projects_state={props.projects_state.clone()} 
                                />
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}