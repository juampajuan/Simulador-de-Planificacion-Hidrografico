use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IMGviewerProps {
    pub image_url: Option<AttrValue>,
    pub mensaje: AttrValue,
}

#[function_component(IMGviewer)]
pub fn imgviewer(props: &IMGviewerProps) -> Html {
    html! {
        <div
            class="
                flex-1
                bg-cyan-100
                dark:bg-zinc-800
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
        </div>
    }
}