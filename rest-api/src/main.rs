pub(crate) mod config_reader;
pub(crate) mod controllers;
pub(crate) mod executor;
pub(crate) mod retrieve;
pub(crate) mod schema;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::{net::SocketAddr, sync::Arc};
use tracing::{debug, info, warn};
use unm_types::{Context, ContextBuilder};

use crate::config_reader::ExternalConfigReader;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    info!("Reading the default context…");
    let default_context = Arc::new({
        Context::read_toml("./config.toml".into()).unwrap_or_else(|e| {
            warn!("Failed to read `config.toml` because of {e}");
            warn!("Use default context built in this API.");

            ContextBuilder::default()
                .build()
                .expect("Failed to build default context")
        })
    });

    info!("Constructing app…");
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/readme", get(readme))
        .nest("/api/v1", {
            Router::new()
                .route("/search", post(controllers::search::search_v1))
                .route("/retrieve", post(controllers::retrieve::retrieve_v1))
                .layer(Extension(default_context))
        })
        .nest("/schema/v1", {
            Router::new()
                .route("/search", get(schema::schema_v1_search))
                .route("/error", get(schema::schema_v1_error))
        });

    let serve_address =
        std::env::var("SERVE_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    debug!("Will listen on: {serve_address}");

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr: SocketAddr = serve_address.parse().expect("failed to parse address");
    info!("listening on {}", addr);
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
