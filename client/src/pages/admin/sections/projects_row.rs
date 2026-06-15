use crate::pages::admin::sections::projects_edit::ProjectEdit;
use yew::prelude::*;
use crate::structs::project::Project;
use lucide_yew::{X, Trash, Pencil};
use crate::services::requests::update_project;

#[derive(Properties, PartialEq)]
pub struct ProjectRowProps {
    pub project: Project,                             
    pub projects_state: UseStateHandle<Vec<Project>>, 
}

#[function_component(ProjectRow)]
pub fn project_row(props: &ProjectRowProps) -> Html {
    let is_editing = use_state(|| false);
    let row_mensaje = use_state(String::new);
    let row_loading = use_state(|| false);

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

    let desc_text = props.project.description.clone().unwrap_or_else(|| "Sin descripción".to_string());

    html! {
        <>
            <tr class="text-sm bg-slate-800 rounded-lg shadow-[0_10px_15px_-3px_rgba(0,0,0,0.1),0_4px_6px_-4px_rgba(0,0,0,0.1),inset_0_0_0_1px_rgba(255,255,255,0.1)]">
                <td class="px-4 py-3 font-semibold text-xl rounded-l-lg">
                    { props.project.id }
                </td>
                <td class="px-4 py-3">
                    <div class="space-y-1">
                        <div>{ &props.project.name }</div>
                        <div class="text-xs text-white/70">{ desc_text }</div>
                    </div>
                </td>
                <td class="px-4 py-3">
                    <div class="flex items-center">
                        <div class="bg-cyan-800 font-mono tracking-wider border border-white/20 px-3 py-1 rounded-full">
                            { &props.project.filename }
                        </div>
                    </div>
                </td>
                <td class="p-3 rounded-r-lg">
                    <div class="flex justify-end gap-2 items-center h-full">
                        <button 
                            onclick={toggle_edit} 
                            class={classes!(
                                "p-2", "rounded-full", "transition-colors",
                                if *is_editing { "bg-cyan-200"; "text-black" } else { "bg-white/15"; "hover:bg-white/25"; "text-white" }
                            )}
                        >
                            if *is_editing { <X size={18}/> } else { <Pencil size={18}/> }
                        </button>
                        <button class="bg-red-800 p-2 rounded-full hover:bg-red-700 transition-colors">
                            <Trash size={18}/>
                        </button>
                    </div>
                </td>
            </tr>

            if *is_editing {
                <tr class="text-sm bg-slate-800/20">
                    <td colspan="4" class="p-4 pt-1">
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
        </>
    }
}