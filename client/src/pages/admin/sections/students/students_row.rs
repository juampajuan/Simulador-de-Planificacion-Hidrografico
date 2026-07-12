use crate::services::requests::update_student;
use crate::structs::project::Project;
use common::Student;
use lucide_yew::{Eye, Pencil, Save, Trash, X};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StudentRowProps {
    pub usuario: Student,
    pub proyectos: Vec<Project>,
    pub delete_target: UseStateHandle<Option<Student>>,
    pub students_state: UseStateHandle<Vec<Student>>,
    pub on_view_attempts: Callback<(Student, Project)>,
}

/// Fila de un alumno en la tabla.
#[function_component(StudentRow)]
pub fn student_row(props: &StudentRowProps) -> Html {
    let is_editing = use_state(|| false);
    let row_mensaje = use_state(String::new);
    let row_loading = use_state(|| false);

    let edit_name = use_state({
        let n = props.usuario.name.clone();
        move || n
    });

    let edit_project_id = use_state({
        let p = props.usuario.project_id;
        move || p
    });

    let current_project_id = *edit_project_id;
    let name_original = props.usuario.name.clone();
    let project_original = props.usuario.project_id;

    let toggle_edit = {
        let is_editing = is_editing.clone();
        let edit_name = edit_name.clone();
        let edit_project_id = edit_project_id.clone();
        let row_mensaje = row_mensaje.clone();

        Callback::from(move |_| {
            if *is_editing {
                edit_name.set(name_original.clone());
                edit_project_id.set(project_original);
                row_mensaje.set(String::new());
            }
            is_editing.set(!*is_editing);
        })
    };

    let usuario_delete = props.usuario.clone();
    let on_delete_click = {
        let delete_target = props.delete_target.clone();
        Callback::from(move |_| {
            delete_target.set(Some(usuario_delete.clone()));
        })
    };

    let id_save = props.usuario.id;
    let students_state_save = props.students_state.clone();

    let on_save_student = {
        let is_editing = is_editing.clone();
        let edit_name = edit_name.clone();
        let edit_project_id = edit_project_id.clone();
        let row_mensaje = row_mensaje.clone();
        let row_loading = row_loading.clone();

        Callback::from(move |_| {
            let name = (*edit_name).trim().to_string();
            let p_id = *edit_project_id;

            if name.is_empty() {
                row_mensaje.set("El nombre del grupo no puede estar vacío".to_string());
            } else {
                row_mensaje.set("Guardando...".to_string());
                update_student(
                    id_save,
                    name,
                    p_id,
                    students_state_save.clone(),
                    row_mensaje.clone(),
                    row_loading.clone(),
                );
                is_editing.set(false);
            }
        })
    };

    let edit_name_input = edit_name.clone();
    let edit_project_select = edit_project_id.clone();
    let row_mensaje_clear = row_mensaje.clone();

    let proyecto_encontrado = props
        .proyectos
        .iter()
        .find(|p| p.id == props.usuario.project_id);

    let (proy_name, proy_desc) = match proyecto_encontrado {
        Some(p) => (
            p.name.clone(),
            p.description
                .clone()
                .unwrap_or_else(|| "Sin descripción".to_string()),
        ),
        None => (String::new(), String::new()),
    };

    let on_view_click = {
        let on_view_attempts = props.on_view_attempts.clone();
        let usuario = props.usuario.clone();
        let proyecto = proyecto_encontrado.cloned();
        Callback::from(move |_| {
            if let Some(proy) = &proyecto {
                on_view_attempts.emit((usuario.clone(), proy.clone()));
            }
        })
    };

    let alert_cls = if !(*row_mensaje).is_empty() {
        let msg = (*row_mensaje).to_lowercase();
        if msg.contains("guardando") || msg.contains("exito") {
            "text-cyan-400 mt-1 block text-xs"
        } else {
            "text-red-400 mt-1 block text-xs font-medium"
        }
    } else {
        ""
    };

    html! {
        <tr class={classes!(
            "text-sm", "rounded-lg", "shadow-[inset_0_0_0_1px_rgba(255,255,255,0.1)]",
            if *is_editing { "bg-slate-600" } else { "bg-slate-800" }
        )}>
            <td class="px-4 py-3 rounded-l-lg font-medium">
                if *is_editing {
                    <div class="w-full">
                        <input
                            type="text"
                            disabled={*row_loading}
                            value={(*edit_name).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                edit_name_input.set(input.value());
                                row_mensaje_clear.set(String::new());
                            })}
                            class="bg-slate-900 border border-white/20 rounded px-2 py-1 text-sm focus:outline-none focus:border-cyan-400 text-white w-auto box-border"
                        />
                        if !(*row_mensaje).is_empty() {
                            <span class={alert_cls}>{ (*row_mensaje).clone() }</span>
                        }
                    </div>
                } else {
                    <div class="truncate">{ &props.usuario.name }</div>
                }
            </td>

            <td class="px-4 py-3">
                <div class="flex items-center">
                    <div class="bg-cyan-800 font-mono tracking-wider border border-white/20 px-3 py-1 rounded-full text-xs">
                        { &props.usuario.code }
                    </div>
                </div>
            </td>

            <td class="px-4 py-3 text-white">
                if *is_editing {
                    <select
                        disabled={*row_loading}
                        value={current_project_id.to_string()}
                        onchange={Callback::from(move |e: Event| {
                            let select: HtmlSelectElement = e.target_unchecked_into();
                            if let Ok(val) = select.value().parse::<i64>() {
                                edit_project_select.set(val);
                            }
                        })}
                        class="bg-slate-900 border border-white/20 rounded px-2 py-1 text-sm text-white focus:outline-none focus:border-cyan-400 cursor-pointer w-auto box-border"
                    >
                        <option value="0" selected={current_project_id == 0}>{"-- Sin Asignar --"}</option>
                        {
                            for props.proyectos.iter().map(|proy| {
                                let is_selected = current_project_id == proy.id;
                                html! {
                                    <option value={proy.id.to_string()} selected={is_selected}>
                                        { &proy.name }
                                    </option>
                                }
                            })
                        }
                    </select>
                } else if props.usuario.project_id == 0 {
                    <span class="text-white/40">{"Sin asignar"}</span>
                } else if proyecto_encontrado.is_some() {
                    <div class="space-y-1 truncate">
                        <div class="truncate">{ proy_name }</div>
                        <div class="text-xs text-white/70 truncate">{ proy_desc }</div>
                    </div>
                } else {
                    <span class="text-red-400">{"Proyecto no encontrado"}</span>
                }
            </td>

            <td class="p-3 rounded-r-lg">
                <div class="flex justify-end gap-2 items-center w-full">
                    if !*is_editing && proyecto_encontrado.is_some() {
                        <button
                            type="button"
                            disabled={*row_loading}
                            onclick={on_view_click}
                            title="Ver intentos de simulación"
                            class="bg-white/15 hover:bg-white/25 p-2 flex gap-1 rounded-full cursor-pointer transition-colors shrink-0 items-center"
                        >
                            <Eye size={18}/>
                            <p class="text-xs">{"Intentos"}</p>
                        </button>
                    }

                    if *is_editing {
                        <button
                            disabled={*row_loading}
                            onclick={on_save_student}
                            class="flex items-center gap-2 bg-cyan-200 hover:bg-cyan-300 text-black font-semibold text-xs px-3 py-2 rounded transition-colors cursor-pointer shrink-0"
                        >
                            <Save size={18}/>
                            {"Guardar Cambios"}
                        </button>
                    }

                    <button
                        type="button"
                        disabled={*row_loading}
                        onclick={toggle_edit}
                        class={classes!(
                            "p-2", "rounded-full", "transition-colors", "cursor-pointer", "shrink-0",
                            if *is_editing { "bg-cyan-200 text-slate-900" } else { "bg-white/15 hover:bg-white/25 text-white" }
                        )}
                    >
                        if *is_editing { <X size={18}/> } else { <Pencil size={18}/> }
                    </button>

                    <button
                        type="button"
                        disabled={*row_loading}
                        onclick={on_delete_click}
                        class="bg-red-800 p-2 rounded-full cursor-pointer hover:bg-red-700 transition-colors shrink-0"
                    >
                        <Trash size={18}/>
                    </button>
                </div>
            </td>
        </tr>
    }
}
