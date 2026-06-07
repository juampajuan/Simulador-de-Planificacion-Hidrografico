use yew::prelude::*;
use crate::components::root::{Root};
use crate::components::subtitle::Subtitle;
use crate::components::title::Title;
use lucide_yew::{GraduationCap,University};


#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let input_cls = "rounded p-2 text-black text-sm dark:bg-zinc-700 dark:text-white";
     
    html! {
        <Root title={"Simulador Hidrográfico"}>
            <div class="
                flex-1
                flex
                items-center
                justify-center
                dot-grid
                relative
                dark:dot-grid-dark
            "> 
                <div class="bg-cyan-100
                        dark:bg-zinc-900  
                        border
                        border-white/20
                        rounded-md 
                ">
                    <div class="p-6 border-b border-white/20 space-y-2">
                        <Title text={"Acceso al simulador"}/>
                        <div class="dark:text-white/90 text-xs">
                            {"Complete con los datos correspondientes."}
                        </div>
                    </div>
                    <div class="
                        flex 
                        gap-2
                        divide-x divide-white/20
                    ">
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
                                    placeholder="ABC1"
                                    class={format!("{input_cls} text-xl")}
                                />
                            </div>  
    
                        </div> 
                        <div class="p-6">
                            <Subtitle
                                text={"Docente"}
                                icon={html! {
                                    <University size={24}/>
                                }}
                            />  

                            <div class="flex flex-col gap-1 pt-3">
                                <span class="text-xs font-semibold text-white/40 ml-1">
                                    {"Nombre de usuario"}
                                </span>

                                <input
                                    placeholder="gran docente"
                                    class={input_cls} 
                                />
                            </div>  

                            <div class="flex flex-col gap-1 pt-3">
                                <span class="text-xs font-semibold text-white/40 ml-1">
                                    {"Clave"}
                                </span>

                                <input
                                    type="password" 
                                    placeholder="●●●●●●●●●"
                                    class={input_cls} 
                                />
                            </div>  

                            
            
                        </div>
                    </div>

                    <div class="p-6 border-t border-white/20">
                        <button
                            class="text-center w-48 w-full disabled:opacity-30 bg-cyan-200 p-2 px-6 text-black text-sm font-bold hover:bg-cyan-300 transition-all rounded shadow-xl disabled:bg-cyan-100"
                        >
                            {"Acceder"}
                        </button>
                    </div>
                </div> 
               
            </div>
        </Root>
    }
}