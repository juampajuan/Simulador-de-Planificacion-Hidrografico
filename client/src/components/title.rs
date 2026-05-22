use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub text: AttrValue,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    html! {
        <h1 class="
            font-bold text-xl 
            dark:text-cyan-50
            text-cyan-800
        ">
            { &props.text }
        </h1>
    }
}