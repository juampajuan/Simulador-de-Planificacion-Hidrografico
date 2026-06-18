use yew::prelude::*;
use web_sys::window;

pub fn render_background(counter: u8) -> Html {
    html! {
        <div class="h-screen w-screen absolute top-0 left-0 overflow-hidden">
            <img
                src={format!("static/login_wall_{}.jpg", counter)}
                class="w-full h-full object-cover opacity-60"
            />
        </div>
    }
}

// Rota entre 3 imágenes de fondo usando localStorage
pub fn next_counter() -> u8 {
    let Some(storage) = window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    else {
        return 0;
    };

    let current = storage
        .get_item("page_counter")
        .ok()
        .flatten()
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0);

    let next = (current + 1) % 3;
    let _ = storage.set_item("page_counter", &next.to_string());
    next
}