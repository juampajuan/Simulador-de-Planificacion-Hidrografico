use yew::prelude::*;
use web_sys::{window, HtmlElement};
use wasm_bindgen::JsCast;
use lucide_yew::{Sun, Moon};

#[function_component(DarkModeButton)]
pub fn dark_mode_button() -> Html {
    let dark_mode = use_state(|| true);

    let set_dark_mode = {
        let dark_mode = dark_mode.clone();

        Callback::from(move |enabled: bool| {
            let document = window()
                .unwrap()
                .document()
                .unwrap();

            let html = document
                .document_element()
                .unwrap();

            let html: HtmlElement =
                html.dyn_into().unwrap();

            let class_list = html.class_list();

            if enabled {
                class_list.add_1("dark").unwrap();
            } else {
                class_list.remove_1("dark").unwrap();
            }

            dark_mode.set(enabled);
        })
    };

    let toggle_dark_mode = {
        let dark_mode = dark_mode.clone();
        let set_dark_mode = set_dark_mode.clone();

        Callback::from(move |_| {
            set_dark_mode.emit(!*dark_mode);
        })
    };

    use_effect_with((), {
        let set_dark_mode = set_dark_mode.clone();

        move |_| {
            set_dark_mode.emit(true);

            || ()
        }
    });

    html! {
        <button
            class="
                p-1.5
                rounded-full
                hover:bg-black/10
                dark:hover:bg-white/10
                transition
                hidden
            "
            onclick={toggle_dark_mode}
        >
            {
                if *dark_mode {
                    html! {<Sun color="white" size=18 />}
                } else {
                    html! {<Moon color="black" size=18 />}
                }
            }
        </button>
    }
}