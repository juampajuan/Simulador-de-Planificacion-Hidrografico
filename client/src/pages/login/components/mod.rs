pub mod background;
pub mod title;
pub mod forms;
pub mod login_card;

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::components::darkmode_btn::DarkModeButton;
use crate::services::requests::trigger_login;
use background::{render_background, next_counter};
use title::render_title;
use login_card::{calculate_mode, get_animation_classes, render_login_card};

#[derive(Clone, PartialEq)]
pub enum LoginMode {
    None,
    Student,
    Teacher,
}

pub struct LoginCallbacks {
    pub on_student_input:          Callback<InputEvent>,
    pub on_teacher_user_input:     Callback<InputEvent>,
    pub on_teacher_password_input: Callback<InputEvent>,
    pub on_submit:                 Callback<MouseEvent>,
}

fn build_callbacks(
    loading:          &UseStateHandle<bool>,
    student_code:     &UseStateHandle<String>,
    teacher_user:     &UseStateHandle<String>,
    teacher_password: &UseStateHandle<String>,
    login_mensaje:    &UseStateHandle<String>,
) -> LoginCallbacks {
    let on_student_input = {
        let student_code = student_code.clone();
        Callback::from(move |e: InputEvent| {
            student_code.set(e.target_unchecked_into::<HtmlInputElement>().value());
        })
    };

    let on_teacher_user_input = {
        let teacher_user = teacher_user.clone();
        Callback::from(move |e: InputEvent| {
            teacher_user.set(e.target_unchecked_into::<HtmlInputElement>().value());
        })
    };

    let on_teacher_password_input = {
        let teacher_password = teacher_password.clone();
        Callback::from(move |e: InputEvent| {
            teacher_password.set(e.target_unchecked_into::<HtmlInputElement>().value());
        })
    };

    let on_submit = {
        let student_code     = student_code.clone();
        let teacher_user     = teacher_user.clone();
        let teacher_password = teacher_password.clone();
        let loading          = loading.clone();
        let mensaje          = login_mensaje.clone();
        Callback::from(move |_| {
            trigger_login(&student_code, &teacher_user, &teacher_password, mensaje.clone(), loading.clone());
        })
    };

    LoginCallbacks { on_student_input, on_teacher_user_input, on_teacher_password_input, on_submit }
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let input_cls = "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white w-full disabled:dark:text-white/50 disabled:dark:bg-zinc-600";

    let loading          = use_state(|| false);
    let student_code     = use_state(String::new);
    let teacher_user     = use_state(String::new);
    let teacher_password = use_state(String::new);
    let login_mensaje    = use_state(String::new);
    let counter          = use_state(|| 0u8);

    { let counter = counter.clone(); use_effect_with((), move |_| { counter.set(next_counter()); || () }); }

    let callbacks    = build_callbacks(&loading, &student_code, &teacher_user, &teacher_password, &login_mensaje);
    let mode         = calculate_mode(&student_code, &teacher_user, &teacher_password);
    let (student_cls, teacher_cls) = get_animation_classes(&mode);
    let show_divider = matches!(mode, LoginMode::None);
    let disabled     = *loading;

    html! {
        <div class="h-screen w-screen grid grid-cols-2 bg-gradient-to-r from-slate-900 to-slate-900 relative">
            { render_background(*counter) }
            <div class="absolute right-5 top-5 z-30">
                <DarkModeButton/></div>
            <div class="h-screen w-screen grid grid-cols-2 bg-gradient-to-t from-slate-900/80 to-transparent relative">
            
                { render_title() }
                { render_login_card(student_cls, teacher_cls, show_divider, (*student_code).clone(), (*teacher_user).clone(), (*teacher_password).clone(), callbacks, input_cls, disabled) }
            </div>
        </div>
    }
}