use yew::prelude::*;
use lucide_yew::{EqualNot, Image, Layers};
use crate::services::requests::StudentSimulation;
use crate::structs::state::SimulationUiState;
use gloo_timers::callback::Timeout;

#[derive(Properties, PartialEq)]
pub struct ImageSelectorProps {
    pub ui_state: SimulationUiState,
    pub active_sim: StudentSimulation,
}

#[function_component(ImageSelector)]
pub fn image_selector(props: &ImageSelectorProps) -> Html {
    let sim = props.active_sim.clone();
    let min_depth_val = sim.result_min_depth;
    let max_depth_val = sim.result_max_depth;
    
    let selected = use_state(|| 1);

    let change_view_map = {
        let ui = props.ui_state.clone();
        move |path_opt: Option<String>, err_msg: &str, is_diff_view: bool| {
            if let Some(path) = path_opt {
                if !path.is_empty() {
                    ui.image_url.set(Some(format!("/images/{}", path)));
                    ui.mensaje.set(String::new());
                    ui.min_depth.set(min_depth_val);
                    ui.max_depth.set(max_depth_val);
                    
                    if is_diff_view {
                        ui.show_legend.set(false);
                    } else {
                        ui.show_legend.set(true);
                    }
                    
                    ui.loading.set(false);
                } else {
                    ui.image_url.set(None);
                    ui.show_legend.set(false);
                    ui.mensaje.set(err_msg.to_string());
                }
            } else {
                ui.image_url.set(None);
                ui.show_legend.set(false);
                ui.mensaje.set(err_msg.to_string());
            }
        }
    };

    let apply_selection = {
        let selected = selected.clone();
        let change = change_view_map.clone();
        let sim = sim.clone();
        let ui = props.ui_state.clone();

        move |index: usize| {
            selected.set(index);

            match index {
                0 => {
                    ui.image_url.set(None);
                    ui.show_legend.set(false);
                    ui.mensaje.set(String::new());
                }
                1 => change(
                    sim.simulation_image_path.clone(),
                    "No hay mapa de simulación disponible.",
                    false,
                ),
                2 => change(
                    sim.coverage_image_path.clone(),
                    "No hay mapa de cobertura disponible.",
                    false,
                ),
                3 => change(
                    sim.difference_image_path.clone(),
                    "No hay mapa de diferencias disponible.",
                    true,
                ),
                _ => {}
            }
        }
    };

    // Esto preselecciona la foto y la aplica.
    {
        let apply = apply_selection.clone();
        let sim = props.active_sim.clone();

        // Usamos un pequenio timeout, ya que si no, el evento en el modal (del profe) NO logra mostrar la imagen.
        use_effect_with(sim, move |_| {
            let timeout = Timeout::new(10, move || {
                apply(1);
            });

            move || {
                drop(timeout);
            }
        });
    }

    let on_click_sim = {
        let apply = apply_selection.clone();
        Callback::from(move |_| apply(1))
    };

    let on_click_cov = {
        let apply = apply_selection.clone();
        Callback::from(move |_| apply(2))
    };

    let on_click_diff = {
        let apply = apply_selection.clone();
        Callback::from(move |_| apply(3))
    };

    html! {
        <div class="flex gap-2 p-2 bg-slate-950/60 backdrop-blur border border-white/20 rounded-lg shadow-xl select-none animate-fade-in z-10">
                        
            <button 
                onclick={on_click_sim} 
                class={classes!(
                    "w-full", "text-xs", "font-semibold", "transition-all", "rounded",
                    "cursor-pointer", "text-left", "flex", "items-center", "gap-2", "py-2", "px-2",
                    if *selected == 1 {
                        Some("text-white bg-zinc-700/50")
                    } else {
                        Some("text-white/40 hover:text-white/70 hover:bg-zinc-700/30")
                    }
                )}
            >
                <Image size={18}/> 
                <span class="truncate">{"Simulación"}</span>
            </button>
            
            <button 
                onclick={on_click_cov} 
                class={classes!(
                    "w-full", "text-xs", "font-semibold", "transition-all", "rounded",
                    "cursor-pointer", "text-left", "flex", "items-center", "gap-2", "py-2", "px-2",
                    if *selected == 2 {
                        Some("text-white bg-zinc-700/50")
                    } else {
                        Some("text-white/40 hover:text-white/70 hover:bg-zinc-700/30")
                    }
                )}
            >
                <Layers size={18}/> 
                <span class="truncate">{"Cobertura"}</span>
            </button>
            
            <button 
                onclick={on_click_diff} 
                class={classes!(
                    "w-full", "text-xs", "font-semibold", "transition-all", "rounded",
                    "cursor-pointer", "text-left", "flex", "items-center", "gap-2", "py-2", "px-2",
                    if *selected == 3 {
                        Some("text-white bg-zinc-700/50")
                    } else {
                        Some("text-white/40 hover:text-white/70 hover:bg-zinc-700/30")
                    }
                )}
            >
                <EqualNot size={18}/> 
                <span class="truncate">{"Diferencias"}</span>
            </button>

        </div>
    }
}