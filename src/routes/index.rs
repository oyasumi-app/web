use yew::prelude::*;

use crate::components::{main_hero, ComingSoon};

#[function_component]
pub fn Index() -> Html {
    html! {
        <div>
            <main_hero::MainHero />
            <ComingSoon />
        
        </div>
        
    }
}