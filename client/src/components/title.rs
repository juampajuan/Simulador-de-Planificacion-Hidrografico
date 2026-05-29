use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub text: AttrValue,

    #[prop_or_default]
    pub icon: Html,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    html! {
        <h1 class="
            font-semibold text-xl
            dark:text-cyan-50
            text-cyan-800 flex items-center gap-2
        ">
            { props.icon.clone() }
            { &props.text }
        </h1>
    }
}