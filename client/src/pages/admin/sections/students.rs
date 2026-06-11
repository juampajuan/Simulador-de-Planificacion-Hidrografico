use yew::prelude::*;
use crate::components::{subtitle::Subtitle};
use lucide_yew::{Plus, Users, Trash, Pencil};
 

#[function_component(AdminStudents)]
pub fn admin_students() -> Html {

    let usuarios = vec![
        Usuario {
            name: "Supremacia de juanes".into(),
            code: "AB811C".into(),
            project: 23
        },
        Usuario {
            name: "Pedro, Lopez, Miguel".into(),
            code: "AF53SB".into(),
            project: 11
        },
        Usuario {
            name: "Solo martin".into(),
            code: "7B23L1".into(),
            project: 32
        },
        Usuario {
            name: "Los Medidores".into(),
            code: "37GWSB".into(),
            project: 02
        },
        Usuario {
            name: "Ecosondistas".into(),
            code: "MMN15X".into(),
            project: 15
        },
    ];
     
    html! {<> 
        <div class="text-white flex justify-between p-2 pr-1">
            <div class="space-y-1">
                <Subtitle
                    text={"Todos los estudiantes y grupos"}
                    icon={html! {
                        <Users size={24}/>
                    }}
                />
                <p class="text-white/70 text-xs">{"Aca podes administrar los distintos grupos y asignarles proyectos a realizar."}</p> 
            </div>
            <button class="flex items-center px-3 py-0 gap-2 bg-cyan-200 text-black/90 rounded-full">
                <Plus size={18}/>
                <p class="text-sm font-semibold pt-0.5">{"Agregar"}</p>
            </button>
        </div>
 
        <div class="h-full overflow-y-auto pb-16">
            <TablaUsuarios usuarios={usuarios}/>
        </div>
    </>}
}

#[derive(Clone, PartialEq)]
pub struct Usuario {
    pub name: String,
    pub code: String,
    pub project: i32
}

#[derive(Properties, PartialEq)]
pub struct TablaUsuariosProps {
    pub usuarios: Vec<Usuario>,
}

#[function_component(TablaUsuarios)]
pub fn tabla_usuarios(props: &TablaUsuariosProps) -> Html {
    html! {
        <div class="overflow-x-auto text-white">
            <table class="min-w-full border-separate border-spacing-y-2">
                <thead class="text-xs bg-slate-950 shadow-[inset_0_0_0_1px_rgba(255,255,255,0.2)] rounded-lg">
                    <tr>
                        <th class="px-4 py-2 text-left rounded-l-lg">
                            {"Nombre Alumno/Grupo"}
                        </th>
                        <th class="px-4 py-2 text-left">
                            {"Codigo acceso"}
                        </th>
                        <th class="px-4 py-2 text-left">
                            {"Proyecto Asignado"}
                        </th>
                        <th class="px-4 py-2 text-end rounded-r-lg">
                            {"Acciones"}
                        </th>
                    </tr>
                </thead>

                <tbody>
                    {
                        props.usuarios.iter().map(|usuario| {
                            html! {
                                <tr class="text-sm bg-slate-800 rounded-lg shadow-[0_10px_15px_-3px_rgba(0,0,0,0.1),0_4px_6px_-4px_rgba(0,0,0,0.1),inset_0_0_0_1px_rgba(255,255,255,0.1)]">
                                    <td class="px-4 py-3 rounded-l-lg">
                                        { &usuario.name }
                                    </td>
                                    <td class="px-4 py-3">
                                        <div class="flex items-center">
                                            <div class="bg-cyan-800 font-mono tracking-wider border border-white/20 px-3 py-1 rounded-full">
                                                { &usuario.code }
                                            </div>
                                        </div>
                                    </td>
                                    <td class="px-4 py-3">
                                        { usuario.project }
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