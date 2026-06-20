use yew::prelude::*; 

#[derive(Properties, PartialEq)]
pub struct ParamContProps {
    #[prop_or_default]
    pub header: Html,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(ParamCont)]
pub fn param_cont(props: &ParamContProps) -> Html {
    html! {
        <div>
            <div
                class="
                    w-[380px] 
                    max-h-full
                    overflow-hidden
                    flex
                    flex-col
                    dark:bg-slate-950
                    border
                    border-white/20
                    rounded-xl
                    shadow-xl
                "
            > 

                <div class="shrink-0 sticky top-0 z-10 dark:bg-slate-950/60 backdrop-blur border-b border-white/20 shadow p-2">
                    { props.header.clone() }
                </div>

                <div class="space-y-2 py-2 overflow-hidden overflow-y-auto">
                    { for props.children.iter() }
                </div>

            </div>
        </div>
    }
}