use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SubtitleProps {
    pub text: AttrValue,

    #[prop_or_default]
    pub icon: Html,
}

/// Subtítulo de sección, en mayúsculas y color cyan, con ícono opcional.
#[function_component(Subtitle)]
pub fn subtitle(props: &SubtitleProps) -> Html {
    html! {
        <h1 class="
            font-semibold 
            uppercase
            text-sm
            text-cyan-300
            flex items-center gap-2
        ">
            { props.icon.clone() }
            <p class="pt-0.5">{ &props.text }</p>
        </h1>
    }
}