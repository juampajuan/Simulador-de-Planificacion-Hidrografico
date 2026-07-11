use crate::components::subtitle::Subtitle;
use lucide_yew::GraduationCap;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Props del formulario de alumno: estado de carga, el estado compartido del código de
/// acceso y las clases CSS de los inputs.
#[derive(Properties, PartialEq)]
pub struct StudentFormProps {
    pub loading: bool,
    pub student_code: UseStateHandle<String>,
    pub input_cls: &'static str,
}

/// Formulario de acceso del alumno: un único campo con su código de acceso.
#[function_component(StudentForm)]
pub fn student_form(props: &StudentFormProps) -> Html {
    let on_student_input = {
        let student_code = props.student_code.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            student_code.set(value);
        })
    };

    html! {
        <div class="p-6">
            <Subtitle
                text={"Estudiante"}
                icon={html! { <GraduationCap size={24}/> }}
            />

            <div class="flex flex-col gap-1 pt-3">
                <span class="text-xs font-semibold text-white/40 ml-1">
                    {"Código de acceso"}
                </span>

                <input
                    type="text"
                    disabled={props.loading}
                    value={(*props.student_code).clone()}
                    oninput={on_student_input}
                    placeholder="ABC1J5"
                    class={format!("{} text-xl", props.input_cls)}
                />
            </div>
        </div>
    }
}
