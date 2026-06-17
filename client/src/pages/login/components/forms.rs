use yew::prelude::*;
use crate::components::subtitle::Subtitle;
use lucide_yew::{GraduationCap, University};

pub fn render_student_form(cls: &'static str, value: String, on_input: Callback<InputEvent>, input_cls: &'static str, disabled: bool) -> Html {
    html! {
        <div class={cls}>
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
                        disabled={disabled} 
                        value={value} 
                        oninput={on_input} 
                        placeholder="ABC1J5" 
                        class={format!("{input_cls} text-xl")} 
                    />
                </div>
            </div>
        </div>
    }
}

pub fn render_teacher_form(cls: &'static str, user: String, password: String, on_user: Callback<InputEvent>, on_password: Callback<InputEvent>, input_cls: &'static str, disabled: bool) -> Html {
    html! {
        <div class={cls}>
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
                        disabled={disabled} 
                        value={user} 
                        oninput={on_user} 
                        placeholder="granDocente" 
                        class={input_cls} 
                    />
                </div>

                <div class="flex flex-col gap-1 pt-3">
                    <span class="text-xs font-semibold text-white/40 ml-1">
                        {"Clave"}
                    </span>

                    <input 
                        disabled={disabled} 
                        value={password} 
                        oninput={on_password} 
                        type="password" 
                        placeholder="●●●●●●●●●" 
                        class={input_cls} 
                    />
                </div>
            </div>
        </div>
    }
}