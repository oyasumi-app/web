use branding_text::name::GITHUB_ORG_URL;
use yew::prelude::*;

#[function_component]
pub fn ComingSoon() -> Html {
    html! {
        <div class="container">
            <div class="row">
                <div class="col">
                    <div class="h-100 p-5 bg-black text-center border rounded-3"> // like jumbotron 
                        <h2>{"Under development"}</h2>
                        <p>
                            {"The app is currently in development, and is not yet ready for use. "}
                            {"Make sure to follow the "}
                            <a href={GITHUB_ORG_URL}>{"GitHub organization"}</a>
                            {" for updates."}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}