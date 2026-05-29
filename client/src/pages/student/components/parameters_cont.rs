use yew::prelude::*; 

#[derive(Properties, PartialEq)]
pub struct ParamContProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ParamCont)]
pub fn param_cont(props: &ParamContProps) -> Html {
    html! {
       <div
            class="
                w-[420px]
                h-full
                overflow-hidden
                overflow-y-auto
                flex
                flex-col
            "
        > 
            <div class="min-h-full space-y-2">
                { for props.children.iter() }
            </div>

            // bg-green-900
            //     dark:bg-zinc-800
            //     text-white
            //     rounded-md  
            //     transition-colors

            // <div
            //     class="
            //         backdrop-blur-md
            //         bg-white/10
            //         border-b
            //         border-white/20
            //         px-3
            //         py-2
            //         sticky
            //         top-0
            //         z-50 
            //         font-semibold
            //     "
            // >
            //     {"Parámetros para simular"}
            // </div>
 
            // <div
            //     class="
            //         flex-1
            //         overflow-y-auto
            //         overflow-x-hidden
            //     "
            // >
                
            // </div>
        </div>
    }
}