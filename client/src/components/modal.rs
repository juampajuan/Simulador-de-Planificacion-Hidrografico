use yew::prelude::*;
use lucide_yew::X;

use crate::components::subtitle::Subtitle;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub subtitle: String,
    pub on_close: Callback<()>,
    pub children: Html, 
    #[prop_or_default]
    pub max_width_class: Option<String>, 
}

/// Modal genérico reutilizable: título, botón de cerrar y contenido variable (children).
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let on_close = props.on_close.clone();
    let max_width = props.max_width_class.clone().unwrap_or_else(|| "max-w-md".to_string());

    html! {
        <div class="fixed inset-0 top-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 overflow-y-auto">
            <div class={classes!(
                "bg-slate-900", "border", "border-white/10", "rounded-2xl", "p-3", 
                "w-full", "shadow-2xl", "relative", "text-white", "my-8", 
                "animate-in", "fade-in", "duration-150",
                max_width
            )}>

                <div class="flex justify-between mb-1">
            
                    <Subtitle
                        text={props.title.clone()}
                        icon={html! {}}
                    />
        
                    <button 
                        type="button" 
                        onclick={move |_| on_close.emit(())} 
                        class="p-0.5 text-white/40 hover:text-white transition-colors cursor-pointer"
                    >
                        <X size={20} />
                    </button>

                </div>

                <div class="pb-5">
                    <p class="text-xs text-white/70">{&props.subtitle}</p>
                </div>

                { props.children.clone() }
            </div>
        </div>
    }
}