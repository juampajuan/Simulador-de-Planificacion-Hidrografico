use yew::prelude::*;
use crate::components::title::Title;
use super::{LoginMode, LoginCallbacks};
use super::forms::{render_student_form, render_teacher_form};

pub fn calculate_mode(
    student_code:     &UseStateHandle<String>,
    teacher_user:     &UseStateHandle<String>,
    teacher_password: &UseStateHandle<String>,
) -> LoginMode {
    if !student_code.is_empty() {
        LoginMode::Student
    } else if !teacher_user.is_empty() || !teacher_password.is_empty() {
        LoginMode::Teacher
    } else {
        LoginMode::None
    }
}

pub fn get_animation_classes(mode: &LoginMode) -> (&'static str, &'static str) {
    let student_cls = match mode {
        LoginMode::Teacher => "flex-1 overflow-hidden transition-all duration-300 ease-in-out opacity-0 scale-95 max-w-0 p-0",
        _                  => "flex-1 overflow-hidden transition-all duration-300 ease-in-out opacity-100 scale-100 max-w-md",
    };

    let teacher_cls = match mode {
        LoginMode::Student => "flex-1 overflow-hidden transition-all duration-300 ease-in-out opacity-0 scale-95 max-w-0 p-0",
        _                  => "flex-1 overflow-hidden transition-all duration-300 ease-in-out opacity-100 scale-100 max-w-md flex-1",
    };

    (student_cls, teacher_cls)
}

fn render_divider(show: bool) -> Html {
    html! {
        <div class={classes!("w-px", "bg-white/20", "transition-opacity", "duration-300", if show { "opacity-100" } else { "opacity-0" })} />
    }
}

fn render_submit_button(on_submit: Callback<MouseEvent>, disabled: bool) -> Html {
    html! {
        <div class="p-6 border-t border-white/20">
            <button 
                disabled={disabled} 
                onclick={on_submit}
                class="
                    text-center 
                    w-full 
                    disabled:opacity-30 
                    bg-cyan-200 
                    p-2 
                    px-6 
                    text-black 
                    text-sm 
                    font-bold 
                    hover:bg-cyan-300 
                    transition-all 
                    rounded 
                    shadow-xl 
                    disabled:bg-cyan-100
                "
            >
                {"Acceder"}
            </button>
        </div>
    }
}

pub fn render_login_card(
    student_cls:      &'static str,
    teacher_cls:      &'static str,
    show_divider:     bool,
    student_code:     String,
    teacher_user:     String,
    teacher_password: String,
    callbacks:        LoginCallbacks,
    input_cls:        &'static str,
    disabled:         bool,
) -> Html {
    html! {
        <div class="
            flex-1 
            flex 
            items-center 
            justify-center 
            dot-grid 
            relative 
            dark:dot-grid-dark
        ">
            <div class="
                bg-cyan-100 
                dark:bg-slate-950/70 
                backdrop-blur 
                w-[420px] 
                border 
                border-white/25 
                rounded-md 
                shadow-xl
            ">
                <div class="p-6 border-b border-white/20 space-y-2">

                    <Title text={"Acceso al simulador"} />

                    <div class="dark:text-white/90 text-xs">
                        {"Complete con los datos correspondientes."}
                    </div>
                </div>

                <div class="
                    flex
                ">
                    { render_student_form(student_cls, student_code, callbacks.on_student_input, input_cls, disabled) }
                    { render_divider(show_divider) }
                    { render_teacher_form(teacher_cls, teacher_user, teacher_password, callbacks.on_teacher_user_input, callbacks.on_teacher_password_input, input_cls, disabled) }
                </div>
                { render_submit_button(callbacks.on_submit, disabled) }
            </div>
        </div>
    }
}