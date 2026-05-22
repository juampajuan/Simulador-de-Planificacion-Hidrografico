use yew::prelude::*;
use super::title::Title;
use super::darkmode_btn::DarkModeButton;

#[derive(Properties, PartialEq)]
pub struct RootProps {
    pub title: AttrValue,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Root)]
pub fn root(props: &RootProps) -> Html {
    html! {
        <main class=" 
            h-screen
            w-full
            bg-cyan-200
            dark:bg-cyan-800
            flex
            flex-col
            transition-colors
            duration-300
        ">
            // Esto es la navbar, luego va a estar el nombre del grupo y un logout.
            <div class="pt-2 px-3 flex justify-between items-center">
                <div>
                    <Title text={&props.title} />
                </div>
                <div>
                    <DarkModeButton />
                </div>
            </div>

            // Contenido principal
            <section class="h-full flex flex-1 gap-2 p-2 overflow-hidden">
                { for props.children.iter() }
            </section>
        </main>
    }
}