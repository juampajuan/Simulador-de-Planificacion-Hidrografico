use yew::prelude::*;
use web_sys::{window, HtmlElement};
use wasm_bindgen::JsCast;

#[function_component(DarkModeButton)]
pub fn dark_mode_button() -> Html {
    let dark_mode = use_state(|| false);

    let toggle_dark_mode = {
        let dark_mode = dark_mode.clone();

        Callback::from(move |_| {
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

            if *dark_mode {
                class_list.remove_1("dark").unwrap();
                dark_mode.set(false);
            } else {
                class_list.add_1("dark").unwrap();
                dark_mode.set(true);
            }
        })
    };

    html! {
        <button
            class="
                p-1
                rounded-full
                hover:bg-black/10
                dark:hover:bg-white/10
                transition
            "
            onclick={toggle_dark_mode}
        >
            {
                if *dark_mode {
                    "☀️"
                } else {
                    "🌙"
                }
            }
        </button>
    }
}