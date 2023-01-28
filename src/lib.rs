use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::{Router, Switch, history::{AnyHistory, MemoryHistory, History}, Routable, BrowserRouter};

mod components;

#[derive(Serialize, Deserialize)]
struct UuidResponse {
    uuid: Uuid,
}

#[cfg(feature = "ssr")]
async fn fetch_uuid() -> Uuid {
    // reqwest works for both non-wasm and wasm targets.
    let resp = reqwest::get("https://httpbin.org/uuid").await.unwrap();
    let uuid_resp = resp.json::<UuidResponse>().await.unwrap();

    uuid_resp.uuid
}

#[function_component]
fn Content() -> HtmlResult {
    let uuid = use_prepared_state!(async move |_| -> Uuid { fetch_uuid().await }, ())?.unwrap();

    Ok(html! {
        <div>{"Random UUID: "}{uuid}</div>
    })
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: String,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </Router>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}


#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(route: Route) -> Html {
    match route {
        Route::Index => html! {
            <components::index::Index />
        },
        Route::NotFound => html! {
            <div>
                <h1>{"404"}</h1>
            </div>
        },
    }
}