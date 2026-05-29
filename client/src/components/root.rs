use yew::prelude::*;
use super::title::Title;
use super::darkmode_btn::DarkModeButton;
use lucide_yew::{LogOut, Waves};

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
            dark:bg-slate-900
            flex
            flex-col
            transition-colors
            duration-300
        ">
            // Esto es la navbar, luego va a estar el nombre del grupo y un logout.
            <div class="pt-2 px-3 flex justify-between items-center">
                <div>
                    <Title text={&props.title} 
                        icon={html! {
                            <Waves size={24} />
                        }}
                    />
                </div>
                <div class="flex gap-2 items-center">
                    <DarkModeButton />
                    <div class="flex pl-3 gap-3 p-1 items-center rounded-full dark:text-white bg-black/10 dark:bg-white/10">
                        <p class="text-sm">{"Grupo 21"}</p>
                        <button class="hover:text-white rounded-full p-2 hover:bg-red-400">
                            <LogOut size=18/>
                        </button>
                    </div>
                </div>
            </div>

            // Contenido principal
            <section class="h-full flex flex-1 gap-2 p-2 overflow-hidden">
                { for props.children.iter() }
            </section>
        </main>
    }
}