use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    student::StudentPage,
    not_found::NotFound,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Student,

    // #[at("/auth/student")]
    // StudentLogin,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={|route| match route {
                Route::Student => html! { <StudentPage /> },
                Route::NotFound => html! { <NotFound /> },
                // Route::StudentLogin => html! { .... },
            }} />
        </BrowserRouter>
    }
}