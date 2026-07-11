use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DepthLegendProps {
    pub start_m: f64,
    pub end_m: f64,
}

/// Leyenda de la escala de colores de profundidad (de mínima a máxima).
#[function_component(DepthLegend)]
pub fn depth_legend(props: &DepthLegendProps) -> Html {
    // TODO: Valores de ejemplo. Luego reemplazalos por props.
    // Agregar los al struct de arriba y recibirlos en la firma.
    let chip_1 = 6.8;
    let chip_2 = 8.9;

    let range = (props.end_m - props.start_m).max(f64::EPSILON);

    let chip_1_top = format!(
        "calc({:.2}% - 14px)",
        ((chip_1 - props.start_m) / range).clamp(0.0, 1.0) * 100.0
    );

    let chip_2_top = format!(
        "calc({:.2}% - 14px)",
        ((chip_2 - props.start_m) / range).clamp(0.0, 1.0) * 100.0
    );

    html! {
        <div class="flex items-center gap-2 h-full pt-16 opacity-100 hover:opacity-100 select-none animate-fade-in">
            <div class="flex h-full flex-col justify-between text-xs text-white">
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

           <div class="relative py-1.5 pb-2 h-full">
                <div
                    class="w-3 h-full rounded outline outline-1 outline-white/60 -outline-offset-1 shadow-xl"
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
                    class="absolute right-5 -translate-y-1/2 z-20 px-3 py-1 border border-white/20 bg-cyan-800 rounded-full shadow-lg font-mono text-white text-xs whitespace-nowrap"
                    style={format!("top:{};", chip_1_top)}
                >
                    { format!("{:.1}m", chip_1) }
                </span>

                <span
                    class="absolute right-5 -translate-y-1/2 z-30 px-3 py-1 border border-white/20 bg-cyan-800 rounded-full shadow-lg font-mono text-white text-xs whitespace-nowrap"
                    style={format!("top:{};", chip_2_top)}
                >
                    { format!("{:.1}m", chip_2) }
                </span>
            </div>
        </div>
    }
}