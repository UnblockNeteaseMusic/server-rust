pub(crate) mod executor;
pub(crate) mod config_reader;
pub(crate) mod controllers;

use axum::{
    routing::{get, post},
    Router, Extension,
};
use unm_types::{Context, ContextBuilder};
use std::{net::SocketAddr, sync::Arc};
use tracing::warn;

use crate::config_reader::ExternalConfigReader;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let default_context = Arc::new({
        Context::read_context_toml("./config.toml".into())
            .unwrap_or_else(|e| {
                warn!("Failed to read `config.toml` because of {e}");
                warn!("Use default context built in this API.");

                ContextBuilder::default().build().expect("Failed to build default context")
            })
    });

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/readme", get(readme))
        .nest("/api/v1", {
            Router::new()
                .route("/search", post(controllers::search::search_v1))
                .layer(Extension(default_context))
        });

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Welcome to UNM REST API!  Navigate to /readme to see the usage of this API."
}

async fn readme() -> &'static str {
    // The README.md file including the usage information of this API.
    include_str!("../README.md")
}
