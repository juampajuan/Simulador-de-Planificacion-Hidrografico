use yew::prelude::*;
use crate::structs::project::Project;
use lucide_yew::Save;

#[derive(Properties, PartialEq)]
pub struct ProjectEditProps {
    pub project_state: Project,
    pub projects_state: UseStateHandle<Vec<Project>>, 
    pub on_save: Callback<Project>,
}

#[function_component(ProjectEdit)]
pub fn project_edit(props: &ProjectEditProps) -> Html {
    let attempts_limit = use_state(|| props.project_state.attempts_limit);
    let weather = use_state(|| props.project_state.weather.clone());
    let seabed_hardness = use_state(|| props.project_state.seabed_hardness.clone());
    let budget = use_state(|| props.project_state.budget);
    let min_depth = use_state(|| props.project_state.geotiff_min_depth);
    let max_depth = use_state(|| props.project_state.geotiff_max_depth);

    let weather_clone = weather.clone();
    let on_weather_change = Callback::from(move |e: Event| {
        if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() { 
            weather_clone.set(target.value()); 
        }
    });

    let seabed_clone = seabed_hardness.clone();
    let on_seabed_change = Callback::from(move |e: Event| {
        if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() { 
            seabed_clone.set(target.value()); 
        }
    });

    let min_depth_clone = min_depth.clone();
    let on_min_depth_input = Callback::from(move |e: InputEvent| {
        if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
            if let Ok(val) = target.value().parse::<f64>() { min_depth_clone.set(val); }
        }
    });

    let max_depth_clone = max_depth.clone();
    let on_max_depth_input = Callback::from(move |e: InputEvent| {
        if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
            if let Ok(val) = target.value().parse::<f64>() { max_depth_clone.set(val); }
        }
    });

    let on_submit = {
        let on_save = props.on_save.clone();
        let project_base = props.project_state.clone();
        let attempts = attempts_limit.clone();
        let w = weather.clone();
        let s = seabed_hardness.clone();
        let b = budget.clone();
        let mind = min_depth.clone();
        let maxd = max_depth.clone();

        Callback::from(move |_| {
            let updated_project = Project {
                attempts_limit: *attempts,
                weather: (*w).clone(),
                seabed_hardness: (*s).clone(),
                budget: *b,
                geotiff_min_depth: *mind,
                geotiff_max_depth: *maxd,
                ..project_base.clone()
            };
            on_save.emit(updated_project);
        })
    };

    html! {
        <div class="space-y-4 border-t border-white/5 pt-4">
            <div class="text-xs font-bold text-cyan-400 uppercase tracking-wider">
                { "Editar Parámetros de Proyecto" }
            </div>

            <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                <div class="flex flex-col gap-1">
                    <label class="text-xs text-white/50">{"Límite de Intentos"}</label>
                    <input 
                        type="number" 
                        value={(*attempts_limit).to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                if let Ok(val) = target.value().parse::<i64>() { attempts_limit.set(val); }
                            }
                        })}
                        class="bg-slate-900 border border-white/10 rounded px-3 py-1.5 text-sm focus:outline-none focus:border-cyan-400 text-white"
                    />
                </div>

                <div class="flex flex-col gap-1">
                    <label class="text-xs text-white/50">{"Condición Climática"}</label>
                    <select value={(*weather).clone()} onchange={on_weather_change} class="bg-slate-900 border border-white/10 rounded px-3 py-1.5 text-sm focus:outline-none focus:border-cyan-400 text-white appearance-none cursor-pointer">
                        <option value="Favorable" selected={*weather == "Favorable"}>{"Favorable"}</option>
                        <option value="Poco favorable" selected={*weather == "Poco favorable"}>{"Poco favorable"}</option>
                        <option value="Desfavorable" selected={*weather == "Desfavorable"}>{"Desfavorable"}</option>
                    </select>
                </div>

                <div class="flex flex-col gap-1">
                    <label class="text-xs text-white/50">{"Dureza del Fondo"}</label>
                    <select value={(*seabed_hardness).clone()} onchange={on_seabed_change} class="bg-slate-900 border border-white/10 rounded px-3 py-1.5 text-sm focus:outline-none focus:border-cyan-400 text-white appearance-none cursor-pointer">
                        <option value="Duro" selected={*seabed_hardness == "Duro"}>{"Duro"}</option>
                        <option value="Blando" selected={*seabed_hardness == "Blando"}>{"Blando"}</option>
                    </select>
                </div>

                <div class="flex flex-col gap-1">
                    <label class="text-xs text-white/50">{"Presupuesto Asignado ($)"}</label>
                    <input 
                        type="number" step="0.01" value={(*budget).to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                if let Ok(val) = target.value().parse::<f64>() { budget.set(val); }
                            }
                        })}
                        class="bg-slate-900 border border-white/10 rounded px-3 py-1.5 text-sm focus:outline-none focus:border-cyan-400 text-white"
                    />
                </div>

                <div class="flex flex-col gap-1">
                    <label class="text-xs text-white/50">{"Profundidad Mínima del GeoTIFF (mts)"}</label>
                    <input type="number" step="0.1" value={(*min_depth).to_string()} oninput={on_min_depth_input} class="bg-slate-900 border border-white/10 rounded px-3 py-1.5 text-sm focus:outline-none focus:border-cyan-400 text-white" />
                </div>

                <div class="flex flex-col gap-1">
                    <label class="text-xs text-white/50">{"Profundidad Máxima del GeoTIFF (mts)"}</label>
                    <input type="number" step="0.1" value={(*max_depth).to_string()} oninput={on_max_depth_input} class="bg-slate-900 border border-white/10 rounded px-3 py-1.5 text-sm focus:outline-none focus:border-cyan-400 text-white" />
                </div>
            </div>

            <div class="flex justify-end pt-2">
                <button onclick={on_submit} class="flex items-center gap-2 bg-cyan-200 hover:bg-cyan-300 text-black font-semibold text-xs px-4 py-2 rounded-full transition-colors">
                    <Save size={14}/>
                    {"Guardar Cambios"}
                </button>
            </div>
        </div>
    }
}