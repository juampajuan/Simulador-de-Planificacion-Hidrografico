use yew::prelude::*;
use lucide_yew::X;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub on_close: Callback<()>,
    pub children: Html, 
    #[prop_or_default]
    pub max_width_class: Option<String>, 
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let on_close = props.on_close.clone();
    let max_width = props.max_width_class.clone().unwrap_or_else(|| "max-w-md".to_string());

    html! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 overflow-y-auto">
            <div class={classes!(
                "bg-slate-900", "border", "border-white/10", "rounded-2xl", "p-6", 
                "w-full", "shadow-2xl", "relative", "text-white", "my-8", 
                "animate-in", "fade-in", "zoom-in-95", "duration-150",
                max_width
            )}>
                <button 
                    type="button" 
                    onclick={move |_| on_close.emit(())} 
                    class="absolute top-4 right-4 text-white/40 hover:text-white transition-colors cursor-pointer"
                >
                    <X size={20} />
                </button>

                <div class="space-y-1 mb-4">
                    <h3 class="text-lg font-bold text-cyan-200">{ &props.title }</h3>
                </div>

                { props.children.clone() }
            </div>
        </div>
    }
}