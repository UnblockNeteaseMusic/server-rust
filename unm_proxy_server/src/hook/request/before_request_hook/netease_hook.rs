use crate::hook::request::NETEASE_MOCK_IP;
use crate::middleware::NeteaseApiContext;

use crate::error::ServerResult;
use either::Either::{self, Left, Right};

use crate::utils::{extract_request_body, get_content_length};
use http::request::Request;
use http::uri::Uri;

use hyper::Body;
use regex::Regex;
use serde_json::Value;

use std::fmt::Formatter;
use std::str::FromStr;

use unm_utils::iter::Slice;

#[derive(Debug)]
pub(in crate::hook::request) enum NeteaseApiHookStatus {
    Unhookable,
}

impl std::fmt::Display for NeteaseApiHookStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NeteaseApiHookStatus::Unhookable => {
                write!(f, "This request is not hook-able.")
            }
        }
    }
}

pub(super) async fn hook_netease_api(
    request: &mut Request<Body>,
) -> ServerResult<Either<NeteaseApiHookStatus, NeteaseApiContext>> {
    let mut netease_ctx = NeteaseApiContext::new();
    let content_length = get_content_length(request.headers());

    // Decompress the body.
    let body = {
        if content_length > 0 {
            let body = extract_request_body(request).await?;
            let body_clone = body.clone();
            let mut_body = request.body_mut();
            *mut_body = Body::from(body);

            Some(body_clone)
        } else {
            None
        }
    };

    // Clean up headers.
    {
        let headers = request.headers_mut();
        headers.remove("x-napm-retry");
        headers.insert(
            "X-Real-IP",
            NETEASE_MOCK_IP
                .parse()
                .expect("unable to parse NETEASE_MOCK_IP"),
        );
    }

    // Check if the API is hook-able.
    {
        let url = request.uri();
        if url.path().contains("stream") {
            return Ok(Left(NeteaseApiHookStatus::Unhookable));
        }
    }

    // Parse the Netease data.
    // https://github.com/UnblockNeteaseMusic/server/blob/v0.27.0-beta.9/src/hook.js#L128
    {
        if let Some(body) = body {
            let netease_pad = {
                let netease_pad_matcher = Regex::new("%0+$").unwrap();
                let find_result = netease_pad_matcher.find(&body);

                if let Some(matches) = find_result {
                    matches.as_str().to_string()
                } else {
                    "".to_string()
                }
            };
            let netease_forward = request.uri() == "/api/linux/forward";
            let take_body_hex = |pre_skip| -> ServerResult<Vec<u8>> {
                let body_len = body.len();
                let pad_len = netease_pad.len();

                let body_part = body
                    .chars()
                    .slice(pre_skip, body_len - pad_len)
                    .collect::<String>();
                Ok(hex::decode(body_part)?)
            };

            if netease_forward {
                let body_hex = take_body_hex(8);
                let body_decrypted = unm_crypto::linux::decrypt(&body_hex?)?;
                let body_object: serde_json::Value = serde_json::from_slice(&body_decrypted[..])?;
                let body_path = {
                    let body_url = body_object.get("url").and_then(|v| v.as_str());

                    if let Some(url) = body_url {
                        let url = Uri::from_str(url)?;
                        url.path().to_string()
                    } else {
                        "".to_string()
                    }
                };
                let body_params = body_object.get("params").cloned();

                netease_ctx.path = Some(body_path);
                netease_ctx.param = body_params;
            } else {
                let body_hex = take_body_hex(7)?;
                let body_decrypted = unm_crypto::eapi::decrypt(&body_hex)?;
                let body_str = String::from_utf8(body_decrypted)?;
                let mut body_split_iter = body_str.split("-36cd479b6b5-");

                let body_path = body_split_iter.next().unwrap_or("").to_string();
                let body_param_raw = body_split_iter.next();

                if let Some(body_param_raw) = body_param_raw {
                    let body_param_json: Value = serde_json::from_str(body_param_raw)?;
                    netease_ctx.param = Some(body_param_json);
                }

                netease_ctx.path = Some(body_path);
            }

            netease_ctx.path = Some(
                Regex::new("/\\d*$")
                    .unwrap()
                    .replace(
                        &*netease_ctx
                            .path
                            .expect("netease_ctx.path should never be null"),
                        "",
                    )
                    .to_string(),
            );
            netease_ctx.pad = Some(netease_pad);
            netease_ctx.forward = netease_forward;
        } else {
            return Ok(Left(NeteaseApiHookStatus::Unhookable));
        }
    }

    Ok(Right(netease_ctx))
}
