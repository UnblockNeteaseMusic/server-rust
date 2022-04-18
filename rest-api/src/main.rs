pub(crate) mod config_reader;
pub(crate) mod controllers;
pub(crate) mod executor;
pub(crate) mod retrieve;
pub(crate) mod schema;

use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    Extension, Json, Router,
};
use http::{HeaderMap, Method, StatusCode};
use serde_json::{json, Value};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, info, warn};
use unm_types::ContextBuilder;

use crate::config_reader::{ContextTomlStructure, ExternalConfigReader};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    info!("Reading the default context…");
    let default_context = Arc::new({
        ContextTomlStructure::read_toml("./config.toml".into())
            .map(|v| v.context)
            .unwrap_or_else(|e| {
                warn!("Failed to read `config.toml` because of {e}");
                warn!("Use default context built in this API.");

                ContextBuilder::default()
                    .build()
                    .expect("Failed to build default context")
            })
    });

    info!("Constructing app…");

    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(vec![http::header::CONTENT_TYPE])
        .allow_origin(Any);

    let rate_limit_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_| async {
            (
                StatusCode::TOO_MANY_REQUESTS,
                {
                    let mut hm = HeaderMap::new();
                    hm.insert(
                        http::header::CONTENT_TYPE,
                        http::HeaderValue::from_static("application/json"),
                    );
                    hm
                },
                r#"{"error": "You request too fast. Please wait 5 minutes."}"#.to_string(),
            )
        }))
        .buffer(1024) // Let RateLimit clone-able
        .load_shed()
        .rate_limit(30, Duration::from_secs(300)) // Allow only 30 requests per 5 minutes
        .into_inner();

    let limit_layer = ServiceBuilder::new()
        .layer(cors_layer)
        .layer(rate_limit_layer)
        .into_inner();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // Docs
        .nest(
            "/docs",
            Router::new()
                .route("/readme", get(|| async { include_str!("../README.md") }))
                .route("/api", get(|| async { include_str!("../docs/api.md") }))
                .route(
                    "/configure",
                    get(|| async { include_str!("../docs/configure.md") }),
                ),
        )
        // API [v1]
        .nest("/api/v1", {
            Router::new()
                .route("/search", post(controllers::search::search_v1))
                .route("/retrieve", post(controllers::retrieve::retrieve_v1))
                .layer(Extension(default_context))
        })
        // Schema [v1]
        .nest("/schema/v1", {
            Router::new()
                .route("/index", get(schema::schema_v1_index))
                .route("/search", get(schema::schema_v1_search))
                .route("/error", get(schema::schema_v1_error))
        })
        .layer(limit_layer);

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

async fn root() -> Json<Value> {
    Json(json!({
        "success": true,
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
