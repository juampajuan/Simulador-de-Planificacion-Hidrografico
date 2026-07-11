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

    // Volvemos a fijar los valores dentro del rango real de la barra (0.0 a 1.0)
    let (pct_min, pct_max) = if range.abs() < f64::EPSILON {
        (0.0, 0.0)
    } else {
        (
            ((props.sim_min - props.start_m) / range).clamp(0.0, 1.0),
            ((props.sim_max - props.start_m) / range).clamp(0.0, 1.0),
        )
    };

    // Detectamos si están tan cerca que van a colisionar
    let overlap = (pct_max - pct_min).abs() < 0.05;

    let chip_1_top = if overlap {
        format!("calc({:.2}% - 14px)", pct_min * 100.0)
    } else {
        format!("{:.2}%", pct_min * 100.0)
    };

    let chip_2_top = if overlap {
        format!("calc({:.2}% + 14px)", pct_max * 100.0)
    } else {
        format!("{:.2}%", pct_max * 100.0)
    };

    html! {
        <div class="flex items-center gap-4 h-full pt-16 pb-6 opacity-100 select-none animate-fade-in pr-24">

            // LADO IZQUIERDO: Etiquetas Grises (Valores fijos extremos)
            <div class="flex h-full flex-col justify-between text-xs text-white py-1">
                <div>
                    <span class="px-3 py-1 border border-white/20 bg-slate-800 rounded-full shadow-lg font-mono whitespace-nowrap">
                        { format!("{:.1}m", props.start_m) }
                    </span>
                </div>
                <div>
                    <span class="px-3 py-1 border border-white/20 bg-slate-800 rounded-full shadow-lg font-mono whitespace-nowrap">
                        { format!("{:.1}m", props.end_m) }
                    </span>
                </div>
            </div>

            // CENTRO: Contenedor de la barra
            <div class="relative py-1.5 pb-2 h-full w-3 flex items-center justify-center">

                // La barra de color
                <div
                    class="w-3 h-full rounded outline outline-1 outline-white/60 -outline-offset-1 shadow-xl z-10"
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

                // LADO DERECHO: Ambas etiquetas azules a la derecha de la barra
                <span
                    class="absolute left-[calc(100%+12px)] z-20 px-3 py-1 border border-white/20 bg-cyan-800 rounded-full shadow-lg font-mono text-white text-xs whitespace-nowrap transition-all duration-200"
                    style={format!("top:{}; transform: translateY(-50%);", chip_1_top)}
                >
                    { format!("{:.1}m", props.sim_min) }
                </span>

                <span
                    class="absolute left-[calc(100%+12px)] z-20 px-3 py-1 border border-white/20 bg-cyan-800 rounded-full shadow-lg font-mono text-white text-xs whitespace-nowrap transition-all duration-200"
                    style={format!("top:{}; transform: translateY(-50%);", chip_2_top)}
                >
                    { format!("{:.1}m", props.sim_max) }
                </span>

            </div>
        </div>
    }
}