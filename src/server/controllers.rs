use hyper::{Body, Request, Response};

use crate::hook::target::HOOK_TARGET_HOST;
use crate::request::reqwest::StatusCode;
use crate::server::error::ServerError;
use crate::server::proxy_pac::gen_proxy_pac;
use crate::Error;

pub async fn proxy_pac_controller(req: Request<Body>) -> Result<Response<Body>, Error> {
    let host = req
        .headers()
        .get("Host")
        .ok_or(ServerError::ExtractHostFailed)?
        .to_str()?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/x-ns-proxy-autoconfig")
        .body(Body::from(gen_proxy_pac(host, &HOOK_TARGET_HOST)))
        .unwrap())
}
