use yew::prelude::*;
use crate::structs::student::Student;
use crate::structs::project::Project;
use crate::pages::admin::sections::students::students_row::StudentRow;

#[derive(Properties, PartialEq)]
pub struct TablaUsuariosProps {
    pub usuarios: Vec<Student>,
    pub proyectos: Vec<Project>, 
    pub delete_target: UseStateHandle<Option<Student>>,
    pub students_state: UseStateHandle<Vec<Student>>,
}

/// Tabla de alumnos/grupos: arma el encabezado y renderiza una fila (`StudentRow`) por alumno.
#[function_component(TablaUsuarios)]
pub fn tabla_usuarios(props: &TablaUsuariosProps) -> Html {
    html! {
        <div class="overflow-x-auto text-white w-full">
            <table class="w-full table-fixed border-separate border-spacing-y-2">
                <thead class="text-xs bg-slate-950 shadow-[inset_0_0_0_1px_rgba(255,255,255,0.2)] rounded-lg">
                    <tr>
                        <th class="w-[30%] px-4 py-2 text-left rounded-l-lg">{"Nombre Alumno/Grupo"}</th>
                        <th class="w-[20%] px-4 py-2 text-left">{"Codigo acceso"}</th>
                        <th class="w-[35%] px-4 py-2 text-left">{"Proyecto Asignado"}</th>
                        <th class="w-[15%] px-4 py-2 text-end rounded-r-lg">{"Acciones"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        props.usuarios.iter().map(|usuario| {
                            html! {
                                <StudentRow 
                                    key={usuario.id}
                                    usuario={usuario.clone()}
                                    proyectos={props.proyectos.clone()}
                                    delete_target={props.delete_target.clone()}
                                    students_state={props.students_state.clone()}
                                />
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}