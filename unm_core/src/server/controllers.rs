use hyper::{Body, Request, Response, StatusCode};

use unm_common::StringError;

use crate::server::error::{ServerError, ServerResult};
use crate::server::hook::consts::HOOK_TARGET_HOST;
use crate::server::proxy_pac::gen_proxy_pac;

pub async fn proxy_pac_controller(req: Request<Body>) -> ServerResult<Response<Body>> {
    let host = req
        .headers()
        .get("Host")
        .ok_or(ServerError::ExtractHostFailed)?
        .to_str()
        .map_err(StringError::StringConvertFailed)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/x-ns-proxy-autoconfig")
        .body(Body::from(gen_proxy_pac(host, &HOOK_TARGET_HOST)))
        .unwrap())
}
