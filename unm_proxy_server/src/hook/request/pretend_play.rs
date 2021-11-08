use log::warn;
use serde_json::{json, value::Value};
use std::str::FromStr;

use crate::middleware::NeteaseApiContext;
use crate::ServerResult;
use hyper::{Body, Request, Uri};

const API_TO_TURN_TO: &str = "http://music.163.com/api/song/enhance/player/url";

// https://github.com/UnblockNeteaseMusic/server/blob/v0.27.0-beta.9/src/hook.js#L378
pub(super) fn pretend_play(
    request: &mut Request<Body>,
    mut netease_context: NeteaseApiContext,
) -> ServerResult<NeteaseApiContext> {
    if let Some(param) = &netease_context.param {
        let get_param_or_null = |key| param.get(key).unwrap_or(&Value::Null);
        let id = get_param_or_null("id");
        let br = get_param_or_null("br").clone();
        let mut e_r = &Value::Null;
        let mut header = &Value::Null;

        if !netease_context.forward {
            e_r = get_param_or_null("e_r");
            header = get_param_or_null("header");
        }

        let pretend = json!({
            "ids": format!(r#"["{id}"]"#, id = id),
            "br": br,
            "e_r": e_r,
            "header": header,
        });
        netease_context.param = Some(pretend.clone());

        let (url, body) = if netease_context.forward {
            let query = unm_crypto::linux::encrypt_request(API_TO_TURN_TO, &pretend)?;
            (query.url, query.body)
        } else {
            let query = unm_crypto::eapi::encrypt_request(API_TO_TURN_TO, &pretend)?;
            (query.url, query.body)
        };

        *request.uri_mut() = Uri::from_str(&url)?;
        *request.body_mut() = Body::from(format!("{}{}", body, netease_context.pad));
    } else {
        warn!("Unable to pretend play: there is no 'param' in netease_context.");
    }

    Ok(netease_context)
}
