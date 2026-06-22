use yew::prelude::*;

// Input de texto o numero
#[derive(Properties, PartialEq)]
pub struct FormInputProps {
    pub label: &'static str,
    pub value: String,
    pub oninput: Callback<InputEvent>,
    #[prop_or("text")] pub input_type: &'static str,
    #[prop_or_default] pub step: &'static str,
}

/// Input genérico de texto o número con su label.
#[function_component(FormInput)]
pub fn form_input(props: &FormInputProps) -> Html {
    html! {
        <div class="flex flex-col space-y-1 w-full">
            <label class="text-xs font-semibold text-white/80">{props.label}</label>
            <input 
                type={props.input_type} 
                step={props.step}
                value={props.value.clone()} 
                oninput={props.oninput.clone()} 
                class="bg-slate-950 text-sm p-2.5 rounded-lg border border-white/10 focus:border-cyan-400 focus:outline-none text-white w-full" 
            />
        </div>
    }
}

// Input de opciones
#[derive(Properties, PartialEq)]
pub struct FormSelectProps {
    pub label: &'static str,
    pub value: String,
    pub onchange: Callback<Event>,
    pub options: Vec<&'static str>,
}

/// Select genérico con su label y lista de opciones.
#[function_component(FormSelect)]
pub fn form_select(props: &FormSelectProps) -> Html {
    html! {
        <div class="flex flex-col gap-1 w-full">
            <label class="text-xs text-white/50">{props.label}</label>
            <select onchange={props.onchange.clone()} class="bg-slate-950 border border-white/10 rounded p-2 text-sm focus:outline-none focus:border-cyan-400 text-white cursor-pointer w-full">
                {
                    props.options.iter().map(|opt| html! {
                        <option value={*opt} selected={props.value == *opt}>{opt}</option>
                    }).collect::<Html>()
                }
            </select>
        </div>
    }
}