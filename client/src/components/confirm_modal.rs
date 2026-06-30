use yew::prelude::*;
use lucide_yew::TriangleAlert;

#[derive(Properties, PartialEq)]
pub struct ConfirmModalProps {
    pub is_open: bool,
    pub title: &'static str,
    pub message: String,
    pub on_confirm: Callback<()>,
    pub on_cancel: Callback<()>,
}

/// Modal de confirmación para acciones destructivas (ej: borrar), con botones Cancelar y Borrar.
#[function_component(ConfirmModal)]
pub fn confirm_modal(props: &ConfirmModalProps) -> Html {
    if !props.is_open { return html! {}; }

    let on_confirm = props.on_confirm.clone();
    let on_cancel = props.on_cancel.clone();

    html! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-3">
            <div class="bg-slate-900 w-full max-w-md rounded-2xl p-3 border border-white/10 shadow-2xl text-white space-y-4 animate-in fade-in zoom-in-95 duration-150">
                
                <div class="flex items-center gap-3 text-amber-400">
                    <TriangleAlert size={24} />
                    <h3 class="text-lg font-bold text-slate-100">{props.title}</h3>
                </div>

                <p class="text-sm text-white/70">
                    { props.message.clone() }
                </p>

                <div class="flex justify-end gap-3 pt-3">
                    <button 
                        type="button" 
                        onclick={move |_| on_cancel.emit(())} 
                        class="px-4 py-2 text-sm font-medium bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 transition-colors cursor-pointer"
                    >
                        {"Cancelar"}
                    </button>
                    <button 
                        type="button" 
                        onclick={move |_| on_confirm.emit(())} 
                        class="px-4 py-2 text-sm font-semibold bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors cursor-pointer"
                    >
                        {"Borrar"}
                    </button>
                </div>
            </div>
        </div>
    }
}