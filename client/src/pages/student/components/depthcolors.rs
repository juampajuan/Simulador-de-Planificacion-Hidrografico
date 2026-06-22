use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DepthLegendProps {
    pub start_m: f64,
    pub end_m: f64,
}

/// Leyenda de la escala de colores de profundidad (de mínima a máxima).
#[function_component(DepthLegend)]
pub fn depth_legend(props: &DepthLegendProps) -> Html {
    html! {
        <div class="flex items-center gap-3 h-full opacity-100 hover:opacity-100">
            <div class="flex h-full flex-col justify-between text-xs text-white">
                <div>
                    <span class="px-3 py-1 border border-white/20 bg-slate-800 rounded-full shadow-lg">{ format!("{:.1}m", props.start_m) }</span>
                </div>
                <div>
                    <span class="px-3 py-1 border border-white/20 bg-slate-800 rounded-full shadow-lg">{ format!("{:.1}m", props.end_m) }</span>
                </div>
            </div>
            <div class="py-1.5 h-full">
                <div
                    class="w-3 rounded h-full outline outline-1 outline-white/60 -outline-offset-1 shadow-xl"
                    style="
                        background: linear-gradient(
                            to bottom,
                            rgb(180,15,0) 0%,
                            rgb(240,100,20) 25%,
                            rgb(255,225,155) 50%,
                            rgb(30,175,135) 75%,
                            rgb(0,70,190) 100%
                        );
                    "
                />
            </div>
        </div>
    }
}