use hyper::header::HeaderValue;
use hyper::{Body, HeaderMap, Method, Request};
use log::debug;

use unm_common::StringError;
use unm_macro::is_host_wrapper;

use crate::error::ServerResult;
use crate::middleware::{Context, Decision, Middleware};
use crate::utils::extract_request_body;

const NETEASE_MOCK_IP: &str = "118.88.88.88";

pub struct BeforeRequestHook;

async fn hook_netease_api(request: &mut Request<Body>, _header: &mut HeaderMap) {
    let _body = extract_request_body(request).await;

    todo!()
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
