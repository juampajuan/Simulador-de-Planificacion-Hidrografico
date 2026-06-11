use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::root::{Root};
use lucide_yew::{FolderOpen, Settings, Users};
 

#[function_component(AdminConfig)]
pub fn admin_config() -> Html {
     
    html! {
        <div class="text-white">
           {"Configutracion"}
        </div>
    }
}