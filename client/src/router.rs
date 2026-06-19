use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    admin::sections::{
        projects::projects::AdminProjects,
        students::students::AdminStudents,
        config::AdminConfig
    },
    admin::AdminLayout,
    login::LoginPage,
    not_found::NotFound,
    student::StudentPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Student,

    #[at("/login")]
    Login,

    #[at("/admin")]
    AdminProjects,

    #[at("/admin/students")]
    AdminStudents,

    #[at("/admin/settings")]
    AdminConfig,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Student => html! {
            <StudentPage />
        },

        Route::Login => html! {
            <LoginPage />
        },

        Route::AdminStudents => html! {
            <AdminLayout>
                <AdminStudents />
            </AdminLayout>
        },

        Route::AdminProjects => html! {
            <AdminLayout>
                <AdminProjects />
            </AdminLayout>
        },

        Route::AdminConfig => html! {
            <AdminLayout>
                <AdminConfig />
            </AdminLayout>
        },

        Route::NotFound => html! {
            <NotFound />
        },
    }
}

#[function_component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}