use crate::pages::admin::sections::projects::projects_edit::ProjectEdit;
use yew::prelude::*;
use crate::structs::project::Project;
use lucide_yew::{X, Trash, Pencil};
use crate::services::requests::{update_project, delete_project};
use crate::components::confirm_modal::ConfirmModal;

#[derive(Properties, PartialEq)]
pub struct ProjectRowProps {
    pub project: Project, 
    pub row_number: usize,                            
    pub projects_state: UseStateHandle<Vec<Project>>, 
}

/// Fila de un proyecto
#[function_component(ProjectRow)]
pub fn project_row(props: &ProjectRowProps) -> Html {
    let is_editing = use_state(|| false);
    let row_mensaje = use_state(String::new);
    let row_loading = use_state(|| false);
    let is_delete_modal_open = use_state(|| false);

    let toggle_edit = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| is_editing.set(!*is_editing))
    };

    let on_save_project = {
        let is_editing = is_editing.clone();
        let projects_state = props.projects_state.clone();
        let row_mensaje = row_mensaje.clone();
        let row_loading = row_loading.clone();
        let project_id = props.project.id;

        Callback::from(move |updated_data: Project| {
            update_project(
                project_id,
                updated_data,
                projects_state.clone(),
                row_mensaje.clone(),
                row_loading.clone()
            );

            is_editing.set(false);
        })
    };

    let on_click_trash = {
        let is_delete_modal_open = is_delete_modal_open.clone();
        Callback::from(move |_| is_delete_modal_open.set(true))
    };

    let on_confirm_delete = {
        let is_delete_modal_open = is_delete_modal_open.clone();
        let projects_state = props.projects_state.clone();
        let row_mensaje = row_mensaje.clone();
        let row_loading = row_loading.clone();
        let project_id = props.project.id;

        Callback::from(move |_| {
            delete_project(
                project_id,
                projects_state.clone(),
                row_mensaje.clone(),
                row_loading.clone()
            );
            is_delete_modal_open.set(false);
        })
    };

    let on_cancel_delete = {
        let is_delete_modal_open = is_delete_modal_open.clone();
        Callback::from(move |_| is_delete_modal_open.set(false))
    };

    let desc_text = props.project.description.clone().unwrap_or_else(|| "Sin descripción".to_string());
    let is_exam_project = props.project.exam_mode;

    html! {
        <>
            <tr 
                class={classes!(
                    "text-sm",
                    if *is_editing { "bg-slate-600" } else { "bg-slate-800" },
                    "rounded-lg",
                    "shadow-[0_10px_15px_-3px_rgba(0,0,0,0.1),0_4px_6px_-4px_rgba(0,0,0,0.1),inset_0_0_0_1px_rgba(255,255,255,0.1)]"
                )}
            >
                <td class="px-4 py-2 text-left font-medium">{ props.row_number }</td>   
                <td class="px-4 py-3">
                    <div class="space-y-1">
                        <div class="font-medium text-white">{ &props.project.name }</div>
                        <div class="text-xs text-white/70">{ desc_text }</div>
                    </div>
                </td>

                <td class="px-4 py-3 whitespace-nowrap align-middle">
                    <div class="font-medium text-white select-none">
                        { if is_exam_project { "Entrega" } else { "Libre" } }
                    </div>
                </td>

                <td class="px-4 py-3 whitespace-nowrap align-middle">
                    <div class="font-medium text-white/90">
                        { format!("{}", props.project.attempts_limit) }
                    </div>
                </td>

                <td class="px-4 py-3 align-middle">
                    <div class="flex items-center">
                        <div class="bg-cyan-800 font-mono tracking-wider border border-white/20 px-3 py-1 rounded-full">
                            { &props.project.filename }
                        </div>
                    </div>
                </td>

                <td class="p-3 rounded-r-lg align-middle">
                    <div class="flex justify-end gap-2 items-center h-full">
                        <button 
                            onclick={toggle_edit}
                            class={classes!(
                                "p-2", "rounded-full", "transition-colors", "cursor-pointer",
                                if *is_editing { "bg-cyan-200 text-slate-900" } else { "bg-white/15 hover:bg-white/25 text-white" }
                            )}
                        >
                            if *is_editing { <X size={18}/> } else { <Pencil size={18}/> }
                        </button>
                        <button 
                            onclick={on_click_trash}
                            class="bg-red-800 p-2 rounded-full hover:bg-red-700 transition-colors cursor-pointer"
                        >
                            <Trash size={18}/>
                        </button>
                    </div>
                </td>
            </tr>

            if *is_editing {
                <tr class="text-sm">
                    <td colspan="6" class="p-4 bg-slate-600 rounded-lg border border-white/20">
                        <ProjectEdit
                            project_state={props.project.clone()} 
                            projects_state={props.projects_state.clone()}
                            on_save={on_save_project}
                        />
                        if !row_mensaje.is_empty() {
                            <p class="text-red-400 text-xs mt-2">{ &*row_mensaje }</p>
                        }
                    </td>
                </tr>
            }
            <ConfirmModal 
                is_open={*is_delete_modal_open}
                title="¿Estás completamente seguro?"
                message={format!("Esta acción eliminará permanentemente el proyecto '{}' \n junto a todas sus asignaciones.", props.project.name)}
                on_confirm={on_confirm_delete}
                on_cancel={on_cancel_delete}
            />
        </>
    }
}