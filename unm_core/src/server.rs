pub use self::error::*;
use crate::server::route::{root_handler, shutdown_signal};
use hyper::service::{make_service_fn, service_fn};
use log::info;
use std::convert::Infallible;
use std::net::SocketAddr;

macro_rules! instantiate_server {
    ($addr:expr) => {{
        let service =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(root_handler)) });
        let server = hyper::Server::bind(&($addr))
            .serve(service)
            .with_graceful_shutdown(shutdown_signal());

        server
    }};
}

macro_rules! serve_http {
    ($addr:expr) => {{
        tokio::spawn(async move {
            let server = instantiate_server!($addr);

            info!(
                "[HTTP] Welcome! You can access UNM service on: \x1b[1m{}\x1b[0m",
                $addr.to_string()
            );

            server.await
        })
    }};
}

macro_rules! serve_https {
    ($addr:expr) => {{
        tokio::spawn(async move {
            let server = instantiate_server!($addr);

            info!(
                "[HTTPS] Welcome! You can access UNM service on: \x1b[1m{}\x1b[0m",
                $addr.to_string()
            );

            server.await
        })
    }};
}

/// The HTTPS server configuration of UnblockNeteaseMusic.
pub struct HttpsServerConfig {
    /// The address of this server.
    pub address: SocketAddr,
}

/// The HTTP server configuration of UnblockNeteaseMusic.
pub struct HttpServerConfig {
    /// The address of this server.
    pub address: SocketAddr,
}

/// The common server configuration of UnblockNeteaseMusic.
pub struct Server {
    pub http: HttpServerConfig,
    pub https: Option<HttpsServerConfig>,
}

impl Server {
    pub async fn serve(&self) -> ServerServeResult<()> {
        let http_addr = self.http.address;

        if let Some(https_conf) = &self.https {
            let https_addr = https_conf.address;
            let (http_result, https_result) =
                tokio::join![serve_http!(http_addr), serve_https!(https_addr)];

            if let Err(e) = http_result {
                Err(ServerServeError::HttpError(e))
            } else if let Err(e) = https_result {
                Err(ServerServeError::HttpsError(e))
            } else {
                Ok(())
            }
        } else if let Err(e) = serve_http!(http_addr).await {
            Err(ServerServeError::HttpError(e))
        } else {
            Ok(())
        }
    }
}

mod connect_handler;
pub mod controllers;
mod error;
mod hook;
mod middleware;
mod proxy_pac;
pub mod route;
mod utils;
