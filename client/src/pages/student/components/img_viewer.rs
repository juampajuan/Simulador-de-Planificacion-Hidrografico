use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IMGviewerProps {
    pub image_url: Option<AttrValue>,
    pub mensaje: AttrValue,
    pub loading: UseStateHandle<bool>
}

#[function_component(IMGviewer)]
pub fn imgviewer(props: &IMGviewerProps) -> Html {
    html! {
        <div
            class="
                flex-1
                bg-cyan-100
                dark:bg-zinc-900
                flex
                items-center
                justify-center
                overflow-hidden
                transition-colors
                p-2
                border
                border-white/20
                rounded-md 
                dot-grid
                relative
                dark:dot-grid-dark
            "
        >

            {
                if let Some(url) = &props.image_url {
                    html! {
                        <img
                            key={url.to_string()} // <--- CLAVE: Fuerza a Yew a recrear el elemento
                            src={url}
                            class="h-full object-contain rounded-lg"
                        />
                    }
                } else {
                    html! {
                        <h2 class="text-2xl font-bold dark:text-white text-center p-8">
                            { &props.mensaje }
                        </h2>
                    }
                }
            }
            
            { 
                if *props.loading {
                    html! {
                        <div class="flex flex-col absolute top-0 backdrop-blur left-0 dark:bg-black/50 w-full h-full justify-center items-center">
                            <div class="loader2"/>
                            <h2 class="dark:text-cyan-200 font-bold text-center p-5">
                                    { &props.mensaje }
                            </h2>
                        </div>
                    }
                }  else {
                    html! {}
                }
            }

        </div>
    }
}