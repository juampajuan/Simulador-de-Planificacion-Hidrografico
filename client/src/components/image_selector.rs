use yew::prelude::*;
use lucide_yew::{Image, Layers, SlidersHorizontal};
use crate::services::requests::StudentSimulation;
use crate::structs::state::SimulationUiState;

#[derive(Properties, PartialEq)]
pub struct ImageSelectorProps {
    pub ui_state: SimulationUiState,
    pub active_sim: StudentSimulation,
}

#[function_component(ImageSelector)]
pub fn image_selector(props: &ImageSelectorProps) -> Html {
    let sim = &props.active_sim;
    let min_depth_val = sim.result_min_depth;
    let max_depth_val = sim.result_max_depth;

    let change_view_map = {
        let ui = props.ui_state.clone();
        move |path_opt: Option<String>, err_msg: &str| {
            if let Some(path) = path_opt {
                if !path.is_empty() {
                    ui.map_base64.set(None);
                    ui.scale_base64.set(None);
                    ui.image_url.set(Some(format!("/images/{}", path)));
                    ui.mensaje.set(String::new());
                    ui.min_depth.set(min_depth_val);
                    ui.max_depth.set(max_depth_val);
                    ui.loading.set(false);
                } else {
                    ui.image_url.set(None);
                    ui.mensaje.set(err_msg.to_string());
                }
            } else {
                ui.image_url.set(None);
                ui.mensaje.set(err_msg.to_string());
            }
        }
    };

    let on_click_sim = {
        let change = change_view_map.clone();
        let path = sim.simulation_image_path.clone();
        Callback::from(move |_| change(path.clone(), "No hay mapa de simulación disponible."))
    };

    let on_click_cov = {
        let change = change_view_map.clone();
        let path = sim.coverage_image_path.clone();
        Callback::from(move |_| change(path.clone(), "No hay mapa de cobertura disponible."))
    };

    let on_click_diff = {
        let change = change_view_map.clone();
        let path = sim.difference_image_path.clone();
        Callback::from(move |_| change(path.clone(), "No hay mapa de diferencias disponible."))
    };

    html! {
        <div class="flex flex-col gap-1 w-[125px] p-2 bg-slate-950/60 backdrop-blur border border-white/20 rounded-lg shadow-xl select-none animate-fade-in">
            
            <div class="text-[10px] font-bold text-cyan-400 uppercase tracking-wider px-1.5 py-0.5 select-none">
                {"Resultados"}
            </div>
            
            <button 
                onclick={on_click_sim} 
                class="w-full text-xs font-semibold rounded-sm transition-all cursor-pointer text-left flex items-center gap-2 h-7 px-1.5 text-white/40 hover:text-white/70 hover:bg-zinc-700/30"
            >
                <Image size={14} class="shrink-0" /> 
                <span class="truncate">{"Simulación"}</span>
            </button>
            
            <button 
                onclick={on_click_cov} 
                class="w-full text-xs font-semibold rounded-sm transition-all cursor-pointer text-left flex items-center gap-2 h-7 px-1.5 text-white/40 hover:text-white/70 hover:bg-zinc-700/30"
            >
                <Layers size={14} class="shrink-0" /> 
                <span class="truncate">{"Cobertura"}</span>
            </button>
            
            <button 
                onclick={on_click_diff} 
                class="w-full text-xs font-semibold rounded-sm transition-all cursor-pointer text-left flex items-center gap-2 h-7 px-1.5 text-white/40 hover:text-white/70 hover:bg-zinc-700/30"
            >
                <SlidersHorizontal size={14} class="shrink-0" /> 
                <span class="truncate">{"Diferencias"}</span>
            </button>

        </div>
    }
}