use crate::components::attempts_modal::AttemptsModal;
use crate::pages::admin::sections::students::students_row::StudentRow;
use crate::structs::project::Project;
use crate::structs::student::Student;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TablaUsuariosProps {
    pub usuarios: Vec<Student>,
    pub proyectos: Vec<Project>,
    pub delete_target: UseStateHandle<Option<Student>>,
    pub students_state: UseStateHandle<Vec<Student>>,
    pub filter: String,
}

/// Tabla de alumnos/grupos: arma el encabezado y renderiza una fila (`StudentRow`) por alumno.
#[function_component(TablaUsuarios)]
pub fn tabla_usuarios(props: &TablaUsuariosProps) -> Html {
    let selected_target = use_state(|| Option::<(Student, Project)>::None);

    let on_view_attempts = {
        let selected_target = selected_target.clone();
        Callback::from(move |(student, project)| {
            selected_target.set(Some((student, project)));
        })
    };

    let attempts_modal_html = if let Some((student, project)) = &*selected_target {
        let selected_target_close = selected_target.clone();
        html! {
            <AttemptsModal
                is_open={true}
                project={project.clone()}
                student_id={student.id}
                student_code={student.name.clone()}
                on_close={Callback::from(move |_| selected_target_close.set(None))}
            />
        }
    } else {
        html! {}
    };

    let search_query = props.filter.to_lowercase().trim().to_string();

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
                        props.usuarios.iter()
                        .filter(|usuario| {
                            if search_query.is_empty() {
                                true
                            } else {
                                props.proyectos.iter()
                                    .find(|p| p.id == usuario.project_id)
                                    .map(|p| p.name.to_lowercase().contains(&search_query))
                                    .unwrap_or(false)
                            }
                        })
                        .map(|usuario| {
                            html! {
                                <StudentRow
                                    key={usuario.id}
                                    usuario={usuario.clone()}
                                    proyectos={props.proyectos.clone()}
                                    delete_target={props.delete_target.clone()}
                                    students_state={props.students_state.clone()}
                                    on_view_attempts={on_view_attempts.clone()}
                                />
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>

            { attempts_modal_html }
        </div>
    }
}
