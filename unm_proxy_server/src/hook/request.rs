use hyper::header::{HeaderValue, CONTENT_LENGTH};
use hyper::{Body, HeaderMap, Method, Request};
use log::debug;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;

use unm_common::StringError;
use unm_macro::is_host_wrapper;

use crate::error::ServerResult;
use crate::middleware::{Context, Decision, Middleware};
use crate::utils::extract_request_body;

const NETEASE_MOCK_IP: &str = "118.88.88.88";
const CONTENT_LENGTH_DEFAULTS: i32 = 0;

pub struct BeforeRequestHook;

enum NeteaseApiHookResult {
    SUCESSS,
    UNHOOKABLE,
}

async fn hook_netease_api(
    request: &mut Request<Body>,
    header: &mut HeaderMap,
) -> ServerResult<NeteaseApiHookResult> {
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
            return Ok(NeteaseApiHookResult::UNHOOKABLE);
        }
    }

    // Parse the Netease data.
    {
        // Return `UNHOOKABLE` if the body is empty.
        if let Some(body) = body {
            let mut data = String::new();
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

            if netease_forward {
                todo!();
            }
        } else {
            return Ok(NeteaseApiHookResult::UNHOOKABLE);
        }
    }
    todo!();
    Ok(NeteaseApiHookResult::SUCESSS)

    // return request
    //     .read(req)
    //     .then((body) => (req.body = body))
    //     .then((body) => {
    //         if ('x-napm-retry' in req.headers)
    //         delete req.headers['x-napm-retry'];
    //         req.headers['X-Real-IP'] = '118.88.88.88';
    //         if (req.url.includes('stream')) return; // look living eapi can not be decrypted
    //         if (body) {
    //             let data;
    //             const netease = {};
    //             netease.pad = (body.match(/%0+$/) || [''])[0];
    //             netease.forward = url.path === '/api/linux/forward';
    //             if (netease.forward) {
    //                 data = JSON.parse(
    //                     crypto.linuxapi
    //                         .decrypt(
    //                             Buffer.from(
    //                                 body.slice(
    //                                     8,
    //                                     body.length - netease.pad.length
    //                                 ),
    //                                 'hex'
    //                             )
    //                         )
    //                         .toString()
    //                 );
    //                 netease.path = parse(data.url).path;
    //                 netease.param = data.params;
    //             } else {
    //                 data = crypto.eapi
    //                     .decrypt(
    //                         Buffer.from(
    //                             body.slice(
    //                                 7,
    //                                 body.length - netease.pad.length
    //                             ),
    //                             'hex'
    //                         )
    //                     )
    //                     .toString()
    //                     .split('-36cd479b6b5-');
    //                 netease.path = data[0];
    //                 netease.param = JSON.parse(data[1]);
    //             }
    //             netease.path = netease.path.replace(/\/\d*$/, '');
    //             ctx.netease = netease;
    //             // console.log(netease.path, netease.param)
    //
    //             if (netease.path === '/api/song/enhance/download/url')
    //             return pretendPlay(ctx);
    //         }
    //     })
    //     .catch(
    //         (error) =>
    //         error &&
    //             logger.error(
    //                 error,
    //                 `A error occurred in hook.request.before when hooking ${req.url}.`
    //             )
    //     );
}

fn get_header_host(header: &HeaderMap) -> ServerResult<String> {
    let header_host_default_value = HeaderValue::from_static("");
    let header_host = header
        .get("Host")
        .unwrap_or(&header_host_default_value)
        .to_str()
        .map_err(StringError::StringConvertFailed)?
        .to_string();

    Ok(header_host)
}

impl Middleware for BeforeRequestHook {
    type Request = Request<Body>;

    fn execute(request: &mut Self::Request, context: &mut Context) -> ServerResult<()> {
        debug!("Processing in Middleware: BeforeRequestHook");

        let url = request.uri();
        let method = request.method();
        let header = request.headers().clone();

        let url_str = url.to_string();
        let path = url.path();
        let header_host = get_header_host(&header)?;

        // &*String => &(str) => &str
        let is_host = is_host_wrapper!(&*url_str, &*header_host);

        if is_host("music.163.com") {
            context.decision = Decision::Proxy;
        }

        if context.target_host.iter().any(|h| is_host(h))
            && *method == Method::POST
            && (path == "/api/linux/forward" || path.starts_with("/eapi/"))
        {}

        todo!()
    }
}
