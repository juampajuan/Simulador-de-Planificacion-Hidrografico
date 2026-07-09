use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::root::{Root};
use lucide_yew::{FolderOpen, Users};
pub mod sections;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

/// Layout del panel de admin: sidebar de navegación + el contenido de la sección activa.
#[function_component(AdminLayout)]
pub fn admin_layout(props: &LayoutProps) -> Html {
     
    html! {
        <Root title={"Panel del simulador"}>
            <div class="
                flex-1
                flex
                items-center
                justify-center
                relative
                dot-grid-dark
                gap-2
            "> 
                <Sidebar/>

                <div class="w-full p-2 h-full space-y-2">
                    { for props.children.iter() }
                </div>               
            </div>
        </Root>
    }
}


/// Barra lateral con los links a las secciones (Proyectos, Estudiantes)
#[function_component(Sidebar)]
fn sidebar() -> Html {
    let location = use_location().unwrap();
    let current = location.path();

    html! {
        <aside class="
            h-full
            flex flex-col
            bg-slate-950
            border
            border-white/20
            rounded-xl
            p-2
            text-sm
            shadow-xl
        ">
            <nav class="flex-1 w-48">
                <ul class="space-y-2">
                    <li>
                        <Link<Route>
                            to={Route::AdminProjects}
                            classes={nav_class(current == "/admin")}
                        >
                            <FolderOpen size={18} />
                            <span>{ "Proyectos" }</span>
                        </Link<Route>>
                    </li>

                    <li>
                        <Link<Route>
                            to={Route::AdminStudents}
                            classes={nav_class(current == "/admin/students")}
                        >
                            <Users size={18} />
                            <span>{ "Estudiantes" }</span>
                        </Link<Route>>
                    </li>
                </ul>
            </nav>
            /*
                <div class="mt-auto pt-2 border-t border-white/20">
                    <Link<Route>
                        to={Route::AdminConfig}
                        classes={nav_class(current == "/admin/settings")}
                    >
                        <Settings size={18} />
                        <span>{ "Configuración" }</span>
                    </Link<Route>>
                </div>
            */
        </aside>
    }
}

fn nav_class(active: bool) -> &'static str {
    if active {
        "flex items-center gap-2 px-3 py-2 [&>span]:pt-0.5 rounded-md bg-cyan-200 text-black/80"
    } else {
        "flex items-center gap-2 px-3 py-2 [&>span]:pt-0.5 rounded-md hover:bg-white/10 text-white"
    }
}