use crate::components::title::Title;
use crate::services::requests::trigger_login;
use web_sys::window;
use yew::prelude::*;

pub mod login_welcome;
pub mod student_form;
pub mod teacher_form;

use login_welcome::LoginWelcome;
use student_form::StudentForm;
use teacher_form::TeacherForm;

/// En qué modo está el formulario según lo que el usuario empezó a tipear:
/// ninguno (muestra ambos lados), alumno o docente.
#[derive(Clone, PartialEq)]
enum LoginMode {
    None,
    Student,
    Teacher,
}

/// Pantalla de login. Muestra los formularios de alumno y docente lado a lado y, en cuanto
/// el usuario empieza a escribir en uno, colapsa el otro con una animación. Maneja el estado
/// de los campos, el mensaje de feedback (éxito en cyan, error en rojo) y dispara el login.
#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let input_cls = "rounded p-2 text-sm bg-zinc-700 text-white w-full disabled:text-white/50 disabled:bg-zinc-600";

    let loading = use_state(|| false);
    let student_code = use_state(String::new);
    let teacher_user = use_state(String::new);
    let teacher_password = use_state(String::new);
    let login_mensaje = use_state(String::new);
    let counter = use_state(|| 0u8);

    {
        let counter = counter.clone();
        use_effect_with((), move |_| {
            counter.set(next_counter());
            if let Some(win) = window()
                && let Ok(Some(storage)) = win.local_storage()
            {
                let _ = storage.remove_item("group_or_user_name");
                let _ = storage.remove_item("user_role");
            }
            || ()
        });
    }

    // Decide qué formulario mostrar según cuál campo tiene contenido.
    let mode = if !student_code.is_empty() {
        LoginMode::Student
    } else if !teacher_user.is_empty() || !teacher_password.is_empty() {
        LoginMode::Teacher
    } else {
        LoginMode::None
    };
    let show_divider = matches!(mode, LoginMode::None);

    let student_cls = match mode {
        LoginMode::Teacher => {
            "flex-1 overflow-hidden transition-all duration-300 ease-in-out opacity-0 scale-95 max-w-0 p-0"
        }
        _ => {
            "flex-1 overflow-hidden transition-all duration-300 ease-in-out opacity-100 scale-100 max-w-md"
        }
    };

    let teacher_cls = match mode {
        LoginMode::Student => {
            "flex-1 overflow-hidden transition-all duration-300 ease-in-out opacity-0 scale-95 max-w-0 p-0"
        }
        _ => {
            "flex-1 overflow-hidden transition-all duration-300 ease-in-out opacity-100 scale-100 max-w-md flex-1"
        }
    };

    // Al enviar el form, dispara el login con los datos del modo correspondiente.
    let on_submit = {
        let student_code = student_code.clone();
        let teacher_user = teacher_user.clone();
        let teacher_password = teacher_password.clone();
        let loading = loading.clone();
        let mensaje = login_mensaje.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            trigger_login(
                &student_code,
                &teacher_user,
                &teacher_password,
                mensaje.clone(),
                loading.clone(),
            );
        })
    };

    // Color del cartel de feedback: cyan si el mensaje es de éxito/progreso, rojo si es error.
    let alert_cls = if !(*login_mensaje).is_empty() {
        let msg = (*login_mensaje).to_lowercase();
        if msg.contains("exitoso") || msg.contains("autenticando") || msg.contains("redirigiendo") {
            "bg-cyan-500/10 border border-cyan-500/20 text-cyan-400"
        } else {
            "bg-red-500/10 border border-red-500/20 text-red-400"
        }
    } else {
        ""
    };

    html! {
        <div class="h-screen w-screen grid grid-cols-2 bg-gradient-to-r from-slate-900 to-slate-900 relative">
            <div class="h-screen w-screen absolute top-0 left-0 overflow-hidden">
                <img
                    src={format!("static/login_wall_{}.jpg", *counter)}
                    class="w-full h-full object-cover opacity-60"
                />
            </div>

            <div class="h-screen w-screen grid grid-cols-2 bg-gradient-to-t from-slate-900/80 to-transparent relative">

                <LoginWelcome />

                <div class="flex-1 flex items-center justify-center relative dot-grid-dark">
                    <form onsubmit={on_submit} class="bg-slate-950/70 backdrop-blur w-[420px] border border-white/25 rounded-md shadow-xl">

                        <div class="p-6 border-b border-white/20 space-y-2">
                            <Title text={"Acceso al simulador"} />
                            <div class="text-white/90 text-xs">
                                {"Complete con los datos correspondientes."}
                            </div>
                        </div>

                        <div class="flex">
                            <div class={student_cls}>
                                <StudentForm
                                    loading={*loading}
                                    student_code={student_code}
                                    input_cls={input_cls}
                                />
                            </div>

                            <div class={classes!(
                                "w-px", "bg-white/20", "transition-opacity", "duration-300",
                                if show_divider { "opacity-100" } else { "opacity-0" }
                            )}/>

                            <div class={teacher_cls}>
                                <TeacherForm
                                    loading={*loading}
                                    teacher_user={teacher_user}
                                    teacher_password={teacher_password}
                                    input_cls={input_cls}
                                />
                            </div>
                        </div>

                        <div class="p-6 border-t border-white/20">
                            if !(*login_mensaje).is_empty() {
                                <div class={classes!(
                                    alert_cls,
                                    "text-xs", "p-3", "rounded", "mb-4", "text-center", "font-medium", "transition-all"
                                )}>
                                    { (*login_mensaje).clone() }
                                </div>
                            }
                            <button
                                type="submit"
                                disabled={*loading}
                                class="text-center w-full disabled:opacity-30 bg-cyan-200 p-2 px-6 text-black text-sm font-bold hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100 cursor-pointer"
                            >
                                {"Acceder"}
                            </button>
                        </div>

                    </form>
                </div>
            </div>
        </div>
    }
}

/// Rota el índice (0, 1, 2) de la imagen de fondo del login y lo persiste en localStorage,
/// para mostrar una imagen distinta en cada carga de la página.
fn next_counter() -> u8 {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let current = storage
        .get_item("page_counter")
        .unwrap()
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0);
    let next = (current + 1) % 3;
    let _ = storage.set_item("page_counter", &next.to_string());
    next
}
