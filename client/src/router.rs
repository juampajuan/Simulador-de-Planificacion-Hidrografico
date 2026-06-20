use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    admin::sections::{
        projects::projects_admin_page::AdminProjects,
        students::students_admin_page::AdminStudents,
        config::AdminConfig
    },
    admin::AdminLayout,
    login::LoginPage,
    not_found::NotFound,
    student::StudentPage,
};
use crate::protected_route::{ProtectedRoute, Role}; 

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
            <ProtectedRoute required_role={Role::Student}>
                <StudentPage />
            </ProtectedRoute>
        },

        Route::Login => html! {
            <LoginPage />
        },

        Route::AdminStudents => html! {
            <ProtectedRoute required_role={Role::Admin}>
                <AdminLayout>
                    <AdminStudents />
                </AdminLayout>
            </ProtectedRoute>
        },

        Route::AdminProjects => html! {
            <ProtectedRoute required_role={Role::Admin}>
                <AdminLayout>
                    <AdminProjects />
                </AdminLayout>
            </ProtectedRoute>
        },

        Route::AdminConfig => html! {
            <ProtectedRoute required_role={Role::Admin}>
                <AdminLayout>
                    <AdminConfig />
                </AdminLayout>
            </ProtectedRoute>
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