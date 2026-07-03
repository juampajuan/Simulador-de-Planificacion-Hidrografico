use yew::prelude::*;
use crate::components::form_inputs::{FormInput, FormSelect};
use crate::pages::admin::sections::projects::projects_fields::ProjectFormFields;


#[derive(Properties, PartialEq)]
pub struct ProjectParamsFormProps {
    pub form_state: UseStateHandle<ProjectFormFields>,
    pub exam_mode: bool,
}

/// Form reutilizable con los parámetros del proyecto (intentos, clima, fondo, presupuesto, profundidades).
#[function_component(ProjectParamsForm)]
pub fn project_params_form(props: &ProjectParamsFormProps) -> Html {
    let state = props.form_state.clone();

    let state_attempts = state.clone();
    let on_attempts_input = Callback::from(move |e: InputEvent| {
        let mut f = (*state_attempts).clone();
        f.attempts_limit = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
        state_attempts.set(f);
    });

    let state_weather = state.clone();
    let on_weather_change = Callback::from(move |e: Event| {
        let mut f = (*state_weather).clone();
        f.weather = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
        state_weather.set(f);
    });

    let state_seabed = state.clone();
    let on_seabed_change = Callback::from(move |e: Event| {
        let mut f = (*state_seabed).clone();
        f.seabed_hardness = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
        state_seabed.set(f);
    });

    let state_budget = state.clone();
    let on_budget_input = Callback::from(move |e: InputEvent| {
        let mut f = (*state_budget).clone();
        f.budget = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
        state_budget.set(f);
    });

    let state_min = state.clone();
    let on_min_input = Callback::from(move |e: InputEvent| {
        let mut f = (*state_min).clone();
        f.min_depth = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
        state_min.set(f);
    });

    let state_max = state.clone();
    let on_max_input = Callback::from(move |e: InputEvent| {
        let mut f = (*state_max).clone();
        f.max_depth = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
        state_max.set(f);
    });

    html! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <FormInput label="Límite de Intentos" input_type="number" value={state.attempts_limit.clone()} oninput={on_attempts_input} />

            <FormSelect label="Condición Climática" value={state.weather.clone()} options={vec!["Favorable", "Poco favorable", "Desfavorable"]} onchange={on_weather_change} />

            <FormSelect label="Dureza del Fondo" value={state.seabed_hardness.clone()} options={vec!["Duro", "Blando"]} onchange={on_seabed_change} />

            <FormInput label="Presupuesto Asignado ($)" input_type="number" step="0.01" value={state.budget.clone()} oninput={on_budget_input} />

            <FormInput label="Prof. Mínima GeoTIFF (mts)" input_type="number" step="0.1" value={state.min_depth.clone()} oninput={on_min_input} />

            <FormInput label="Prof. Máxima GeoTIFF (mts)" input_type="number" step="0.1" value={state.max_depth.clone()} oninput={on_max_input} />
        </div>
    }
}