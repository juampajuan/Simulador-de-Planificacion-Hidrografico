use yew::prelude::*;
use lucide_yew::University;
use web_sys::HtmlInputElement;
use crate::components::subtitle::Subtitle;

#[derive(Properties, PartialEq)]
pub struct TeacherFormProps {
    pub loading: bool,
    pub teacher_user: UseStateHandle<String>,
    pub teacher_password: UseStateHandle<String>,
    pub input_cls: &'static str,
}

#[function_component(TeacherForm)]
pub fn teacher_form(props: &TeacherFormProps) -> Html {
    let on_user_input = {
        let teacher_user = props.teacher_user.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            teacher_user.set(value);
        })
    };

    let on_password_input = {
        let teacher_password = props.teacher_password.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            teacher_password.set(value);
        })
    };

    html! {
        <div class="p-6">
            <Subtitle
                text={"Docente"}
                icon={html! { <University size={24}/> }}
            />

            <div class="flex flex-col gap-1 pt-3">
                <span class="text-xs font-semibold text-white/40 ml-1 truncate">
                    {"Nombre de usuario"}
                </span>
                <input
                    type="text"
                    disabled={props.loading}
                    value={(*props.teacher_user).clone()}
                    oninput={on_user_input}
                    placeholder="granDocente"
                    class={props.input_cls}
                />
            </div>

            <div class="flex flex-col gap-1 pt-3">
                <span class="text-xs font-semibold text-white/40 ml-1">
                    {"Clave"}
                </span>
                <input
                    type="password"
                    disabled={props.loading}
                    value={(*props.teacher_password).clone()}
                    oninput={on_password_input}
                    placeholder="●●●●●●●●●"
                    class={props.input_cls}
                />
            </div>
        </div>
    }
}