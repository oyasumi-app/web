use branding_text::{tagline::TAGLINE, copy::landing_hero::LANDING_HERO_COPY, name::GITHUB_ORG_URL};
use yew::prelude::*;

#[function_component]
pub fn MainHero() -> Html {
    html! {
        <div class="px-5 py-5 my-5 text-center">
            <h1 class="display-5 fw-bold">{TAGLINE}</h1>
            <div class="col-lg-6 mx-auto">
                <p class="lead mb-4">
                    {LANDING_HERO_COPY}
                </p>
            </div>

            <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
                <a href={GITHUB_ORG_URL} class="btn btn-secondary btn-lg px-4 gap-3">
                    <i class="bi bi-github"></i>
                    { " GitHub" }
                </a>
            </div>
        </div>
    }
}
