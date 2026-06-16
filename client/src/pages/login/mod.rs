use yew::prelude::*;
use crate::components::darkmode_btn::DarkModeButton;
use crate::components::root::{Root};
use crate::components::subtitle::Subtitle;
use crate::components::title::Title;
use lucide_yew::{GraduationCap,University};
use web_sys::{HtmlInputElement, window};
use crate::services::requests::trigger_login;


#[derive(Clone, PartialEq)]
enum LoginMode {
    None,
    Student,
    Teacher,
}

// TODO: Separar en componentes

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let input_cls =
        "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white w-full disabled:dark:text-white/50 disabled:dark:bg-zinc-600";

    let loading = use_state(|| false);
    let student_code = use_state(String::new);
    let teacher_user = use_state(String::new);
    let teacher_password = use_state(String::new);

    let mode = if !student_code.is_empty() {
        LoginMode::Student
    } else if !teacher_user.is_empty() || !teacher_password.is_empty() {
        LoginMode::Teacher
    } else {
        LoginMode::None
    };
    let show_divider = matches!(mode, LoginMode::None);

    let on_student_input = {
        let student_code = student_code.clone();

        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<HtmlInputElement>()
                .value();

            student_code.set(value);
        })
    };

    let on_teacher_user_input = {
        let teacher_user = teacher_user.clone();

        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<HtmlInputElement>()
                .value();

            teacher_user.set(value);
        })
    };

    let on_teacher_password_input = {
        let teacher_password = teacher_password.clone();

        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<HtmlInputElement>()
                .value();

            teacher_password.set(value);
        })
    };

    let student_cls = match mode {
        LoginMode::Teacher => {
            "
            flex-1
            overflow-hidden
            transition-all
            duration-300
            ease-in-out
            opacity-0
            scale-95
            max-w-0
            p-0
            
            "
        }
        _ => {
            "
            flex-1
            overflow-hidden
            transition-all
            duration-300
            ease-in-out
            opacity-100
            scale-100
            max-w-md
            "
        }
    };

    let teacher_cls = match mode {
        LoginMode::Student => {
            "
            flex-1
            overflow-hidden
            transition-all
            duration-300
            ease-in-out
            opacity-0
            scale-95
            max-w-0 
            p-0 
            "
        }
        _ => {
            "
            flex-1
            overflow-hidden
            transition-all
            duration-300
            ease-in-out
            opacity-100
            scale-100
            max-w-md 
            flex-1
            "
        }
    };

    let login_mensaje = use_state(|| String::new());

    let on_submit = {
        let student_code = student_code.clone();
        let teacher_user = teacher_user.clone();
        let teacher_password = teacher_password.clone();
        let loading = loading.clone();
        let mensaje = login_mensaje.clone();

        Callback::from(move |_| {
            trigger_login(
                &student_code,
                &teacher_user,
                &teacher_password,
                mensaje.clone(),
                loading.clone()
            );
        })
    };

    let counter = use_state(|| 0u8);

    {
        let counter = counter.clone();

        use_effect_with((), move |_| {
            counter.set(next_counter());
            || ()
        });
    }

    html! {
        // <Root title={"Bienvenido al Simulador de Planificación Hidrográfico"}>
        <div class="h-screen w-screen grid grid-cols-2 bg-gradient-to-r from-slate-900 to-slate-900 relative">
            <div class="h-screen w-screen absolute top-0 left-0 overflow-hidden">
                <img
                    src={format!("static/login_wall_{}.jpg", *counter)}
                    class="w-full h-full object-cover opacity-60"
                />
            </div>
            <div class="absolute right-5 top-5 z-30">
                <DarkModeButton/>
            </div>
        <div class="h-screen w-screen grid grid-cols-2 bg-gradient-to-t from-slate-900/80 to-transparent relative">
            
            <div class="relative flex justify-center flex-col p-8 pt-0 pb-12 items-center">
                <div class="space-y-2">
                    <h1 class="
                        font-semibold text-4xl
                        font-shadow
                        dark:text-cyan-100
                        text-cyan-800 flex items-center gap-2
                    ">
                        { "Bienvenido al" }
                    </h1>
                    <h1 class="
                        font-semibold text-6xl
                        font-shadow
                        dark:text-cyan-100
                        text-cyan-800 flex items-center gap-2
                    ">
                        { "Simulador de Planificación" }
                    </h1>
                    <h1 class="
                        font-semibold text-6xl
                        font-shadow
                        dark:text-cyan-100
                        text-cyan-800 flex items-center gap-2
                    ">
                        { "Hidrográfico" }
                    </h1>
                </div>
            </div>
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
                        // <img width="60px" src="/static/icon.png"/>

                        <Title text={"Acceso al simulador"} />

                        <div class="dark:text-white/90 text-xs">
                            {"Complete con los datos correspondientes."}
                        </div>
                    </div>

                    <div class="
                        flex
                    ">
                        <div class={student_cls}>
                            <div class="p-6">
                                <Subtitle
                                    text={"Estudiante"}
                                    icon={html! {
                                        <GraduationCap size={24}/>
                                    }}
                                />

                                <div class="flex flex-col gap-1 pt-3">
                                    <span class="text-xs font-semibold text-white/40 ml-1">
                                        {"Codigo de acceso"}
                                    </span>

                                    <input
                                        disabled={*loading}
                                        value={(*student_code).clone()}
                                        oninput={on_student_input}
                                        placeholder="ABC1J5"
                                        class={format!("{input_cls} text-xl")}
                                    />
                                </div>
                            </div>
                        </div>

                        <div
                            class={classes!(
                                "w-px",
                                "bg-white/20",
                                "transition-opacity",
                                "duration-300",
                                if show_divider {
                                    "opacity-100"
                                } else {
                                    "opacity-0"
                                }
                            )}
                        />
                        
                        <div class={teacher_cls}>
                            <div class="p-6">
                                <Subtitle
                                    text={"Docente"}
                                    icon={html! {
                                        <University size={24}/>
                                    }}
                                />

                                <div class="flex flex-col gap-1 pt-3">
                                    <span class="text-xs font-semibold text-white/40 ml-1 truncate">
                                        {"Nombre de usuario"}
                                    </span>

                                    <input
                                        disabled={*loading}
                                        value={(*teacher_user).clone()}
                                        oninput={on_teacher_user_input}
                                        placeholder="granDocente"
                                        class={input_cls}
                                    />
                                </div>

                                <div class="flex flex-col gap-1 pt-3">
                                    <span class="text-xs font-semibold text-white/40 ml-1">
                                        {"Clave"}
                                    </span>

                                    <input
                                        disabled={*loading}
                                        value={(*teacher_password).clone()}
                                        oninput={on_teacher_password_input}
                                        type="password"
                                        placeholder="●●●●●●●●●"
                                        class={input_cls}
                                    />
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="p-6 border-t border-white/20">
                        <button
                            disabled={*loading}        
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
                </div>
            </div>
        </div>
        </div>
    }
}

fn next_counter() -> u8 {
    let storage = window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    let current = storage
        .get_item("page_counter")
        .unwrap()
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0);

    let next = (current + 1) % 3; // 0..4

    storage
        .set_item("page_counter", &next.to_string())
        .unwrap();

    web_sys::console::log_1(
        &format!("Counter: {}", next).into(),
    );

    next
}