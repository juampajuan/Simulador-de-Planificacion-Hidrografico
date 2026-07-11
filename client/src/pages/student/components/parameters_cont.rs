use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ParamContProps {
    #[prop_or_default]
    pub header: Html,

    #[prop_or_default]
    pub children: Children,
}

/// Contenedor del panel de parámetros
#[function_component(ParamCont)]
pub fn param_cont(props: &ParamContProps) -> Html {
    html! {
        <div>
            <div
                class="
                    w-[375px] 
                    max-h-full
                    overflow-hidden
                    flex
                    flex-col
                    border
                    border-white/20
                    rounded-lg
                    shadow-xl
                "
            >

                <div class="shrink-0 sticky top-0 z-1 bg-slate-950/60 rounded-t-lg overflow-hidden backdrop-blur border-b border-white/20 shadow p-2">
                    { props.header.clone() }
                </div>

                <div class="space-y-2 py-2 pb-0 overflow-hidden overflow-y-auto rounded-b-lg bg-slate-950/60 backdrop-blur">
                    { for props.children.iter() }
                </div>

            </div>
        </div>
    }
}
