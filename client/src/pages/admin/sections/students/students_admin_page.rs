use crate::components::confirm_modal::ConfirmModal;
use crate::components::modal::Modal;
use crate::components::subtitle::Subtitle;
use crate::services::requests::{
    create_student, delete_student, get_all_projects, get_all_students,
};
use crate::structs::project::Project;
use common::Student;
use lucide_yew::{Check, ClipboardCopy, Plus, Search, Users};
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

use crate::pages::admin::sections::students::students_table::TablaUsuarios;

/// Sección de administración de alumnos/grupos: lista a todos, permite crear uno nuevo
/// (modal con nombre + proyecto asignado) y borrar con confirmación.
#[function_component(AdminStudents)]
pub fn admin_students() -> Html {
    let students_state = use_state(Vec::<Student>::new);
    let projects_state = use_state(Vec::<Project>::new);

    let ui_mensaje = use_state(String::new);
    let ui_loading = use_state(|| false);

    let show_modal = use_state(|| false);
    let input_name = use_state(String::new);
    let input_project_id = use_state(|| 0i64);

    let form_error = use_state(String::new);

    let delete_target = use_state(|| None::<Student>);
    let search_filter = use_state(String::new);

    {
        let students_state = students_state.clone();
        let projects_state = projects_state.clone();
        let ui_mensaje = ui_mensaje.clone();
        let ui_loading = ui_loading.clone();

        use_effect_with((), move |_| {
            get_all_students(students_state, ui_mensaje.clone(), ui_loading.clone());
            get_all_projects(projects_state, ui_mensaje, ui_loading);
            || ()
        });
    }

    let is_copied = use_state(|| false);

    let on_export = {
        let students = students_state.clone();
        let ui_mensaje = ui_mensaje.clone();
        let is_copied = is_copied.clone(); // Clonamos el estado

        Callback::from(move |_| {
            if students.is_empty() {
                ui_mensaje.set("No hay grupos cargados para exportar.".to_string());
                return;
            }

            let mut export_text = String::from("");
            for student in &*students {
                export_text.push_str(&format!(
                    "Grupo: {}  |  Código: `{}`\n",
                    student.name, student.code
                ));
            }

            if let Some(win) = window() {
                let navigator = win.navigator();
                let clipboard = navigator.clipboard();
                let _ = clipboard.write_text(&export_text);

                is_copied.set(true);

                // Corregido: Usamos Closure::once con una clausura FnOnce normal de Rust
                let is_copied_reset = is_copied.clone();
                let closure = wasm_bindgen::closure::Closure::once(move || {
                    is_copied_reset.set(false);
                });

                // Le pasamos la referencia al setTimeout del navegador
                let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    3000,
                );

                // Dejamos que el runtime de JS controle su ciclo de vida
                closure.forget();
            } else {
                ui_mensaje
                    .set("Error: Tu navegador bloqueó el acceso al portapapeles.".to_string());
            }
        })
    };

    let on_open_add_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(true))
    };

    let on_close_add_modal = {
        let show_modal = show_modal.clone();
        let input_name = input_name.clone();
        let input_project_id = input_project_id.clone();
        let form_error = form_error.clone();
        Callback::from(move |_| {
            show_modal.set(false);
            input_name.set(String::new());
            input_project_id.set(0);
            form_error.set(String::new());
        })
    };

    let on_submit_add = {
        let name = (*input_name).clone();
        let project_id = *input_project_id;
        let students_state = students_state.clone();
        let ui_mensaje = ui_mensaje.clone();
        let ui_loading = ui_loading.clone();
        let show_modal = show_modal.clone();
        let input_name = input_name.clone();
        let input_project_id = input_project_id.clone();
        let form_error = form_error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if project_id == 0 {
                form_error
                    .set("Debes seleccionar un proyecto disponible para el grupo.".to_string());
                return;
            }

            if !name.is_empty() {
                form_error.set(String::new());
                create_student(
                    name.clone(),
                    project_id,
                    students_state.clone(),
                    ui_mensaje.clone(),
                    ui_loading.clone(),
                );
                show_modal.set(false);
                input_name.set(String::new());
                input_project_id.set(0);
            } else {
                form_error.set("Asigna a un alumno/grupo válido.".to_string());
            }
        })
    };

    let on_confirm_delete = {
        let delete_target = delete_target.clone();
        let students_state = students_state.clone();
        let ui_mensaje = ui_mensaje.clone();
        let ui_loading = ui_loading.clone();

        Callback::from(move |_| {
            if let Some(student) = &*delete_target {
                delete_student(
                    student.id,
                    students_state.clone(),
                    ui_mensaje.clone(),
                    ui_loading.clone(),
                );
            }
            delete_target.set(None);
        })
    };

    let on_cancel_delete = {
        let delete_target = delete_target.clone();
        Callback::from(move |_| delete_target.set(None))
    };

    let on_search_input = {
        let search_filter = search_filter.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_filter.set(input.value());
        })
    };

    let add_student_modal_html = if *show_modal {
        let on_close = on_close_add_modal.clone();
        let input_name_setter = input_name.clone();
        let input_project_setter = input_project_id.clone();
        let proyectos_dropdown = (*projects_state).clone();

        html! {
            <Modal title="Agrega el nuevo alumno o grupo" subtitle="" on_close={Callback::from(move |_| on_close.emit(()))}>
                <form onsubmit={on_submit_add}>
                    <div class="space-y-4">

                        if !form_error.is_empty() {
                            <div class="p-2 bg-red-500/15 border border-red-500/20 rounded-lg text-red-400 text-xs font-semibold flex items-center gap-1.5">
                                <span>{ &*form_error }</span>
                            </div>
                        }

                        <div>
                            <label class="block text-xs font-semibold text-white/70 mb-1">{"Nombre del Alumno / Grupo"}</label>
                            <input
                                type="text"
                                required=true
                                value={(*input_name).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    input_name_setter.set(input.value());
                                })}
                                placeholder="Grupo 4"
                                class="w-full bg-slate-950 border border-white/10 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-cyan-400 text-white"
                            />
                        </div>

                        <div>
                            <label class="block text-xs font-semibold text-white/70 mb-1">{"Seleccionar Proyecto Asignado"}</label>
                            <select
                                value={(*input_project_id).to_string()}
                                onchange={Callback::from(move |e: Event| {
                                    let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                    if let Ok(val) = select.value().parse::<i64>() {
                                        input_project_setter.set(val);
                                    }
                                })}
                                class="w-full bg-slate-950 border border-white/10 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-cyan-400 text-white cursor-pointer"
                            >
                                <option value="0" selected={*input_project_id == 0} disabled=true hidden=true>
                                    {"-- Elegir Proyecto Disponible --"}
                                </option>
                                {
                                    proyectos_dropdown.iter().map(|proy| {
                                        html! {
                                            <option value={proy.id.to_string()} selected={*input_project_id == proy.id}>
                                                { &proy.name }
                                            </option>
                                        }
                                    }).collect::<Html>()
                                }
                            </select>
                        </div>
                    </div>

                    <div class="flex justify-end gap-3 mt-6">
                        <button type="button" onclick={move |_| on_close_add_modal.emit(())} class="px-4 py-2 text-sm font-medium border border-white/10 rounded-lg bg-white/10 hover:bg-white/20 cursor-pointer transition-colors">
                            {"Cancelar"}
                        </button>
                        <button type="submit" class="px-4 py-2 text-sm font-semibold rounded-lg bg-cyan-200 text-black/90 hover:bg-cyan-300 cursor-pointer transition-colors">
                            {"Crear Grupo"}
                        </button>
                    </div>
                </form>
            </Modal>
        }
    } else {
        html! {}
    };

    html! {
        <>
            <div class="text-white flex justify-between p-2 pr-1 items-end">
                <div class="space-y-1">
                    <Subtitle text={"Todos los estudiantes y grupos"} icon={html! { <Users size={24}/> }} />
                    <p class="text-white/70 text-xs">{"Aca podes administrar los distintos grupos y asignarles proyectos a realizar."}</p>
                    if !ui_mensaje.is_empty() {
                        <p class="text-cyan-300 text-xs font-mono">{ &*ui_mensaje }</p>
                    }
                </div>
                <div class="flex items-center gap-2">
                    <div class="relative flex items-center">
                        <span class="absolute left-3 text-white pointer-events-none">
                            <Search size={14} />
                        </span>
                        <input
                            type="text"
                            placeholder="Busca por proyecto."
                            value={(*search_filter).clone()}
                            oninput={on_search_input}
                            class="w-[160px] h-9 pl-8 pr-3 rounded-lg text-xs bg-slate-900 hover:bg-slate-850 text-white placeholder-white/40 border border-white/20 focus:border-cyan-400 focus:ring-1 focus:ring-cyan-400/30 focus:outline-none transition-all font-medium shadow-inner"
                        />
                    </div>

                    <button
                        onclick={on_export}
                        disabled={*is_copied}
                        class={
                            if *is_copied {
                                "flex items-center px-3 h-9 gap-1.5 border border-emerald-200 bg-emerald-900 text-emerald-200 rounded-lg transition-colors shadow-md opacity-90"
                            } else {
                                "flex items-center px-3 h-9 gap-1.5 border border-white/10 bg-slate-800 hover:bg-slate-750 text-white/90 rounded-lg cursor-pointer transition-colors shadow-md"
                            }
                        }
                        title="Copia los nombres y códigos de los alumnos en portapapeles"
                    >
                        if *is_copied {
                            <Check size={16}/>
                            <span class="text-xs font-semibold">{"¡Copiado!"}</span>
                        } else {
                            <ClipboardCopy size={16}/>
                            <span class="text-xs font-semibold">{"Copiar códigos"}</span>
                        }
                    </button>

                    <div/>

                    <button onclick={on_open_add_modal} class="flex items-center px-4 h-9 gap-1.5 bg-cyan-200 text-black/90 rounded-lg cursor-pointer hover:bg-cyan-300 transition-colors shadow-lg shadow-cyan-500/5">
                        <Plus size={16}/>
                        <span class="text-xs font-bold">{"Agregar"}</span>
                    </button>
                </div>
            </div>

            <div class="h-full overflow-y-auto pb-16 mt-2">
                if *ui_loading {
                    <div class="text-white text-center p-4 font-mono text-sm">{"Procesando operacion..."}</div>
                } else {
                    <TablaUsuarios
                        usuarios={(*students_state).clone()}
                        proyectos={(*projects_state).clone()}
                        delete_target={delete_target.clone()}
                        students_state={students_state.clone()}
                        filter={(*search_filter).clone()}
                    />
                }
            </div>

            { add_student_modal_html }

            <ConfirmModal
                is_open={delete_target.is_some()}
                title="¿Eliminar Alumno / Grupo?"
                message={
                    if let Some(s) = &*delete_target {
                        format!("¿Estás seguro de que querés borrar a '{}'? Esta acción no se puede deshacer.", s.name)
                    } else { String::new() }
                }
                on_confirm={on_confirm_delete}
                on_cancel={on_cancel_delete}
            />
        </>
    }
}
