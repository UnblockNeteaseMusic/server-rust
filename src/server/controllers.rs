use hyper::{Body, Request, Response};

use crate::request::reqwest::StatusCode;
use crate::server::error::ServerError;
use crate::server::hook::consts::HOOK_TARGET_HOST;
use crate::server::proxy_pac::gen_proxy_pac;
use crate::ErrorResult;

pub async fn proxy_pac_controller(req: Request<Body>) -> ErrorResult<Response<Body>> {
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
