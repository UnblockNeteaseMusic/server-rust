use std::convert::Infallible;
use std::error::Error;
use std::fmt::Debug;

use hyper::{Body, Method, Request, Response, StatusCode};
use log::error;

use crate::server::controllers::proxy_pac_controller;

mod controllers;
pub mod error;
pub mod proxy_pac;

pub async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

fn internal_server_error_response(message: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(message.to_string()))
        .unwrap()
}

fn unimplemented_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Unimplemented :("))
        .unwrap()
}

fn error_handler(route: &str, error: impl Error + Debug) -> Response<Body> {
    error!("Server [{}]: {:?}", route, error);
    internal_server_error_response(&format!(
        "// Failed to process your request: {:?} \n\
        // Please check if your request is valid.",
        error
    ))
}

pub async fn root_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let route = (req.method(), req.uri().path());

    Ok(if let (&Method::GET, "/proxy.pac") = route {
        proxy_pac_controller(req)
            .await
            .unwrap_or_else(|error| error_handler("/proxy.pac", error))
    } else {
        unimplemented_response()
    })
}
