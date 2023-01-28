use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::path::PathBuf;

use axum::body::{Body, StreamBody};
use axum::error_handling::HandleError;
use axum::extract::Query;
use axum::handler::Handler;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use clap::Parser;
use futures::stream::{self, StreamExt};
use hyper::server::Server;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use yew::platform::Runtime;
use web::{ServerApp, ServerAppProps};


/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long, parse(from_os_str))]
    dir: PathBuf,
}

async fn render(
    Extension((index_html_before, index_html_after)): Extension<(String, String)>,
    url: Request<Body>,
    Query(queries): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let url = url.uri().to_string();

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries,
    });

    StreamBody::new(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
}

// An executor to process requests on the Yew runtime.
//
// By spawning requests on the Yew runtime,
// it processes request on the same thread as the rendering task.
//
// This increases performance in some environments (e.g.: in VM).
#[derive(Clone, Default)]
struct Executor {
    inner: Runtime,
}

impl<F> hyper::rt::Executor<F> for Executor
where
    F: Future + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.inner.spawn_pinned(move || async move {
            fut.await;
        });
    }
}

#[tokio::main]
async fn main() {
    let exec = Executor::default();

    env_logger::init();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    // Split the index.html into two parts: before and after the <body> tag.
    // Note that the <body> tag has some attributes associated with it.
    // The <body> tag needs to be in the "before" part.
    let (index_html_before, index_html_after) = {
        // Find the <body ...> tag.
        let body_start = index_html_s
            .find("<body")
            .expect("failed to find <body> tag in index.html");
        // Find the end of the <body ...> tag.
        // This is the position of the '>' character relative to the start of the <body ...> tag --
        // aka the length of the <body ...> tag.
        let body_tag_len = index_html_s
            .split_at(body_start).1
            .find('>')
            .expect("failed to find <body> tag in index.html");
        // The "before" part is the part until the end of the <body ...> tag,
        // including the '>' character.
        // For this, we split at (the position of the "<body") + (the length of the "<body ...> tag").
        // The "after" part is the rest of the string.
        let (before, after) = index_html_s.split_at(body_start + body_tag_len + 1);

        (before.to_string(), after.to_string())
    };

    let handle_error = |e| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error occurred: {}", e),
        )
    };

    let app = Router::new()
        .route("/api/test", get(|| async move { "Hello World" }))
        .fallback(HandleError::new(
            ServeDir::new(opts.dir)
                .append_index_html_on_directories(false)
                .fallback(
                    render
                        .layer(Extension((
                            index_html_before.clone(),
                            index_html_after.clone(),
                        )))
                        .into_service()
                        .map_err(|err| -> std::io::Error { match err {} }),
                ),
            handle_error,
        ));

    println!("You can view the website at: http://localhost:8080/");

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .executor(exec)
        .serve(app.into_make_service())
        .await
        .unwrap();
}