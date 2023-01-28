use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use branding_text::name::NAME;

#[function_component]
pub fn MainNavbar() -> Html {
    html!{
        <nav class="navbar navbar-expand-lg bg-body-tertiary">
            <div class="container-fluid">
                <Link<Route> classes="navbar-brand" to={Route::Index}>{NAME}</Link<Route>>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#mainNavbarCollapsible" aria-controls="mainNavbarCollapsible" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="mainNavbarCollapsible">
                    <div class="navbar-nav">
                        <Link<Route> classes="nav-link" to={Route::Index}>{"Home"}</Link<Route>>
                        <a class="nav-link disabled">{"Login"}</a>
                    </div>
                </div>
            </div>
        </nav>
    }
}