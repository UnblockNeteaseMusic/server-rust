use async_trait::async_trait;
use either::Either::{self, Left, Right};
use log::{debug, info};

use http::Uri;
use hyper::header::{HeaderValue, CONTENT_LENGTH};
use hyper::{Body, HeaderMap, Method, Request};

use regex::Regex;
use serde_json::Value;

use std::fmt::Formatter;
use std::str::FromStr;

use unm_utils::iter::Slice;
use unm_utils::val_inside;

use crate::error::ServerResult;
use crate::middleware::{Context, Decision, Middleware, NeteaseApiContext};
use crate::utils::extract_request_body;

const NETEASE_MOCK_IP: &str = "118.88.88.88";
const CONTENT_LENGTH_DEFAULTS: i32 = 0;

pub struct BeforeRequestHook;

#[derive(Debug)]
enum NeteaseApiHookStatus {
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

async fn hook_netease_api(
    request: &mut Request<Body>,
) -> ServerResult<Either<NeteaseApiHookStatus, NeteaseApiContext>> {
    let mut netease_ctx = NeteaseApiContext::new();
    let content_length = {
        // Get the value of Content-Length in request.
        let content_length_header = request.headers().get(CONTENT_LENGTH);

        if let Some(value) = content_length_header {
            value
                .to_str() // Turn the Content-Length to &str.
                .map(|v| {
                    v.parse::<i32>() // Turn the Content-Length(&str) to i32.
                        .unwrap_or(CONTENT_LENGTH_DEFAULTS)
                })
                .unwrap_or(CONTENT_LENGTH_DEFAULTS)
        } else {
            CONTENT_LENGTH_DEFAULTS
        }
    };

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
                .expect("not able to parse NETEASE_MOCK_IP"),
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

                netease_ctx.path = body_path;
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

                netease_ctx.path = body_path;
            }

            netease_ctx.path = Regex::new("/\\d*$")
                .unwrap()
                .replace(&*netease_ctx.path, "")
                .to_string();
            netease_ctx.pad = netease_pad;
            netease_ctx.forward = netease_forward;
        } else {
            return Ok(Left(NeteaseApiHookStatus::Unhookable));
        }
    }

    Ok(Right(netease_ctx))
}

fn get_header_host(header: &HeaderMap) -> ServerResult<String> {
    let header_host_default_value = HeaderValue::from_static("");
    let header_host = header
        .get("Host")
        .unwrap_or(&header_host_default_value)
        .to_str()?
        .to_string();

    Ok(header_host)
}

#[async_trait]
impl Middleware for BeforeRequestHook {
    type Request = Request<Body>;

    async fn execute(request: &mut Self::Request, context: &mut Context) -> ServerResult<()> {
        debug!("Processing in Middleware: BeforeRequestHook");

        let url = request.uri();
        let method = request.method();
        let header = request.headers();

        let url_str = url.to_string();
        let path = url.path();
        let header_host = get_header_host(&header)?;

        // &*String => &(str) => &str
        let is_host = val_inside!(&*url_str, &*header_host);

        if is_host("music.163.com") {
            context.decision = Decision::Proxy;
        }

        if context.target_host.iter().any(|h| is_host(h))
            && *method == Method::POST
            && (path == "/api/linux/forward" || path.starts_with("/eapi/"))
        {
            let netease_ctx = hook_netease_api(request).await?;

            match netease_ctx {
                Left(status) => {
                    info!("Unable to hook the Netease API: {}", status);
                }
                Right(netease_context) => {
                    context.netease_context = Some(netease_context);
                }
            }
        }

        todo!()
        // const pretendPlay = (ctx) => {
        // 	const { req, netease } = ctx;
        // 	const turn = 'http://music.163.com/api/song/enhance/player/url';
        // 	let query;
        // 	if (netease.forward) {
        // 		const { id, br } = netease.param;
        // 		netease.param = { ids: `["${id}"]`, br };
        // 		query = crypto.linuxapi.encryptRequest(turn, netease.param);
        // 	} else {
        // 		const { id, br, e_r, header } = netease.param;
        // 		netease.param = { ids: `["${id}"]`, br, e_r, header };
        // 		query = crypto.eapi.encryptRequest(turn, netease.param);
        // 	}
        // 	req.url = query.url;
        // 	req.body = query.body + netease.pad;
        // };
        // 					// console.log(netease.path, netease.param)
        //
        // 					if (netease.path === '/api/song/enhance/download/url')
        // 						return pretendPlay(ctx);
        // 				}
        // 			})
        // 			.catch(
        // 				(error) =>
        // 					error &&
        // 					logger.error(
        // 						error,
        // 						`A error occurred in hook.request.before when hooking ${req.url}.`
        // 					)
        // 			);
        // 	} else if (
        // 		hook.target.host.has(url.hostname) &&
        // 		(url.path.startsWith('/weapi/') || url.path.startsWith('/api/'))
        // 	) {
        // 		req.headers['X-Real-IP'] = '118.88.88.88';
        // 		ctx.netease = {
        // 			web: true,
        // 			path: url.path
        // 				.replace(/^\/weapi\//, '/api/')
        // 				.split('?')
        // 				.shift() // remove the query parameters
        // 				.replace(/\/\d*$/, ''),
        // 		};
        // 	} else if (req.url.includes('package')) {
        // 		try {
        // 			const data = req.url.split('package/').pop().split('/');
        // 			const url = parse(crypto.base64.decode(data[0]));
        // 			const id = data[1].replace(/\.\w+/, '');
        // 			req.url = url.href;
        // 			req.headers['host'] = url.hostname;
        // 			req.headers['cookie'] = null;
        // 			ctx.package = { id };
        // 			ctx.decision = 'proxy';
        // 			// if (url.href.includes('google'))
        // 			// 	return request('GET', req.url, req.headers, null, parse('http://127.0.0.1:1080'))
        // 			// 	.then(response => (ctx.res.writeHead(response.statusCode, response.headers), response.pipe(ctx.res)))
        // 		} catch (error) {
        // 			ctx.error = error;
        // 			ctx.decision = 'close';
        // 		}
        // 	}
    }
}
