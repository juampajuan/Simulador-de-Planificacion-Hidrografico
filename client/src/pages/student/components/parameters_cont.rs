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
                bg-green-900
                dark:bg-zinc-800
                text-white
                rounded-md  
                transition-colors
                h-full
                overflow-hidden
                flex
                flex-col
            "
        > 
            <div
                class="
                    backdrop-blur-md
                    bg-white/10
                    border-b
                    border-white/20
                    px-3
                    py-2
                    sticky
                    top-0
                    z-50 
                    font-semibold
                "
            >
                {"Parámetros para simular"}
            </div>
 
            <div
                class="
                    flex-1
                    overflow-y-auto
                    overflow-x-hidden
                "
            >
                <div class="min-h-full">
                    { for props.children.iter() }
                </div>
            </div>

            // Cuando haya completado todo lo necesario, se cambia a false y lo podra clickear
            <div class="w-full">
                <button disabled={true} class="text-center disabled:opacity-30 bg-cyan-200 p-2 text-black font-semibold w-full">
                    {"Realizar Medicion"}
                </button>
            </div>
        </div>
    }
}