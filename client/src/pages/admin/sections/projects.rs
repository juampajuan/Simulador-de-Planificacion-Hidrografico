use yew::prelude::*;
use crate::components::{subtitle::Subtitle};
use lucide_yew::{FolderOpenDot, Pencil, Plus, Trash};
 
#[function_component(AdminProjects)]
pub fn admin_projects() -> Html {

    let projects = vec![
        Project {
            name: "Zona problematica".into(),
            description: "Zona de gran conflicto belico activo...".into(),
            filename: "zona_iter_2_final.geotiff".into(),
            id: 23
        },
    ];
     
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
            <button class="flex items-center px-3 py-0 gap-2 bg-cyan-200 text-black/90 rounded-full">
                <Plus size={18}/>
                <p class="text-sm font-semibold pt-0.5">{"Agregar"}</p>
            </button>
        </div>
 
        <div class="h-full overflow-y-auto pb-16">
            <ProjectsTable projects={projects}/>
        </div>
    </>}
}

// TODO: A otra carpeta o archivo
#[derive(Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub filename: String,
    pub id: i32
}

#[derive(Properties, PartialEq)]
pub struct ProjectsTableProps {
    pub projects: Vec<Project>,
}

#[function_component(ProjectsTable)]
pub fn projects_table(props: &ProjectsTableProps) -> Html {
    html! {
        <div class="overflow-x-auto text-white">
            <table class="min-w-full border-separate border-spacing-y-2">
                <thead class="text-xs bg-slate-950 shadow-[inset_0_0_0_1px_rgba(255,255,255,0.2)] rounded-lg">
                    <tr>
                         <th class="px-4 py-2 text-left rounded-l-lg">
                            {"ID"}
                        </th>
                        <th class="px-4 py-2 text-left">
                            {"Nombres/descripcion"}
                        </th>
                        <th class="px-4 py-2 text-left">
                            {"Archivo"}
                        </th>
                        <th class="px-4 py-2 text-left">
                            {"Cantidad Intentos"}
                        </th>
                        <th class="px-4 py-2 text-left">
                            {"CLONAR ESTO, PARA MAS"}
                        </th>
                        <th class="px-4 py-2 text-end rounded-r-lg">
                            {"Acciones"}
                        </th>
                    </tr>
                </thead>

                <tbody>
                    {
                        props.projects.iter().map(|usuario| {
                            html! {
                                <tr class="text-sm bg-slate-800 rounded-lg shadow-[0_10px_15px_-3px_rgba(0,0,0,0.1),0_4px_6px_-4px_rgba(0,0,0,0.1),inset_0_0_0_1px_rgba(255,255,255,0.1)]">
                                    <td class="px-4 py-3 font-semibol text-xl rounded-l-lg">
                                        { &usuario.id }
                                    </td>
                                    <td class="px-4 py-3">
                                        <div class="space-y-1">
                                            <div>{ &usuario.name }</div>
                                            <div class="text-xs text-white/70">{ &usuario.description }</div>
                                        </div>
                                    </td>
                                    <td class="px-4 py-3">
                                        <div class="flex items-center">
                                            <div class="bg-cyan-800 font-mono tracking-wider border border-white/20 px-3 py-1 rounded-full">
                                                { &usuario.filename }
                                            </div>
                                        </div>
                                    </td>
                                    <td class="px-4 py-3">
                                        { "1" }
                                    </td>
                                    <td class="px-4 py-3">
                                        { "COSAS" }
                                    </td>
                                    <td class="p-3 rounded-r-lg">
                                        <div class="flex justify-end gap-2 items-center h-full">
                                            <button class="bg-white/15 p-2 rounded-full">
                                                <Pencil size={18}/>
                                            </button>
                                            <button class="bg-red-800 p-2 rounded-full">
                                                <Trash size={18}/>
                                            </button>
                                        </div>
                                    </td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}