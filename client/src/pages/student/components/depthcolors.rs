use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DepthLegendProps {
    pub start_m: f64,
    pub end_m: f64,
    pub sim_min: f64,
    pub sim_max: f64,
}

/// Leyenda de la escala de colores de profundidad (de mínima a máxima).
#[function_component(DepthLegend)]
pub fn depth_legend(props: &DepthLegendProps) -> Html {
    let range = props.end_m - props.start_m;

    let (pct_min, pct_max) = if range.abs() < f64::EPSILON {
        (0.0, 0.0)
    } else {
        (
            ((props.sim_min - props.start_m) / range).clamp(0.0, 1.0),
            ((props.sim_max - props.start_m) / range).clamp(0.0, 1.0),
        )
    };

    let chip_1_top = format!("{:.2}%", pct_min * 100.0);
    let chip_2_top = format!("{:.2}%", pct_max * 100.0);

    html! {
        <div class="flex flex-col items-end h-full pt-12 pb-2 opacity-100 select-none animate-fade-in">

            <div>
                <span class="px-3 py-2 border relative top-[-5px] border-white/20 bg-slate-700 text-white text-xs rounded-full rounded-br-none shadow-lg font-mono whitespace-nowrap">
                    { format!("{:.1}m", props.start_m) }
                </span>
            </div>

            <div class="relative h-full w-3 flex flex-col items-end justify-center gap-1 bg-red-200">

                <div
                    class="w-3 h-full outline outline-1 outline-white/30 -outline-offset-1 shadow-xl z-10"
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

                <span
                    class="absolute right-[calc(100%+12px)] z-20 px-3 py-1 border border-white/20 bg-cyan-700 rounded-full rounded-tr-none shadow-lg font-mono text-white text-xs whitespace-nowrap transition-all duration-200"
                    style={format!("top:{}; transform: translateY(0px);", chip_1_top)}
                >
                    { format!("{:.1}m", props.sim_min) }
                </span>

                <span
                    class="absolute right-[calc(100%+12px)] z-20 px-3 py-1 border border-white/20 bg-cyan-700 rounded-full rounded-br-none shadow-lg font-mono text-white text-xs whitespace-nowrap transition-all duration-200"
                    style={format!("top:{}; transform: translateY(-100%);", chip_2_top)}
                >
                    { format!("{:.1}m", props.sim_max) }
                </span>

            </div>

            <div>
                <span class="px-3 py-2 relative border border-white/20 top-[2px] bg-slate-700 text-white text-xs rounded-full rounded-tr-none shadow-lg font-mono whitespace-nowrap">
                    { format!("{:.1}m", props.end_m) }
                </span>
            </div>

        </div>
    }
}
