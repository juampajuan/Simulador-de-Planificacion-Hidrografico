use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub text: String,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    html! {
        <h1 class="text-5xl font-bold mb-8">
            { &props.text }
        </h1>
    }
}