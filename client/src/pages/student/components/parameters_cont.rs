use yew::prelude::*; 

#[derive(Properties, PartialEq)]
pub struct ParamContProps {
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
                    overflow-y-auto
                    flex
                    flex-col
                    dark:bg-slate-950
                    border
                    border-white/20
                    rounded-xl
                    py-2 
                    shadow-xl
                "
            > 
                <div class="  space-y-2">
                    { for props.children.iter() }
                </div>

            </div>
        </div>
    }
}