use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub text: AttrValue,

    #[prop_or_default]
    pub icon: Html,
}

/// Título reutilizable, con ícono opcional.
#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    html! {
        <h1 class="
            font-semibold text-xl
            text-cyan-200
            flex items-center gap-2
        ">
            { props.icon.clone() }
            { &props.text }
        </h1>
    }
}
