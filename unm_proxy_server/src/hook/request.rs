mod netease_hook;
mod pretend_play;

use async_trait::async_trait;
use either::Either::{Left, Right};
use log::{debug, info};

use hyper::{Body, Method, Request};

use unm_utils::val_inside;

use crate::error::ServerResult;
use crate::middleware::{Context, Decision, Middleware};
use crate::utils::get_header_host;

use netease_hook::hook_netease_api;
use pretend_play::pretend_play;

const NETEASE_MOCK_IP: &str = "118.88.88.88";

pub struct BeforeRequestHook;

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
        let header_host = get_header_host(header)?;

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
                Right(mut netease_context) => {
                    if netease_context.path == "/api/song/enhance/download/url" {
                        netease_context = pretend_play(request, netease_context)?;
                    }
                    context.netease_context = Some(netease_context);
                }
            }
        }

        todo!()
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
