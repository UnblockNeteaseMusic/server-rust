use async_trait::async_trait;
use either::{Left, Right};
use http::{Method, Uri};
use std::str::FromStr;

use self::{netease_hook::hook_netease_api, pretend_play::pretend_play};
use crate::hook::request::NETEASE_MOCK_IP;

use hyper::{Body, Request};
use log::{debug, info};
use regex::Regex;
use unm_utils::option::UnwrapOrWithLog;
use unm_utils::val_inside;

use crate::middleware::{Context, Decision, Middleware, NeteaseApiContext, Package};
use crate::utils::get_header_host;
use crate::{BeforeRequestHookError, ServerResult};

mod netease_hook;
mod pretend_play;

pub struct BeforeRequestHook;

#[async_trait]
impl Middleware for BeforeRequestHook {
    type Request = Request<Body>;

    fn name(&self) -> String {
        String::from("BeforeRequestHook")
    }

    async fn execute(
        &self,
        request: &mut Self::Request,
        context: &mut Context,
    ) -> ServerResult<()> {
        debug!("Processing in Middleware: BeforeRequestHook");

        let (hostname, url_str, path) = {
            let url = request.uri();
            let hostname = url.host().unwrap_or("").to_string();
            let url_str = url.to_string();
            let path = url.path().to_string();

            (hostname, url_str, path)
        };
        let header_host = get_header_host(request.headers())?;

        // &*String => &(str) => &str
        let is_host = val_inside!(&*url_str, &*header_host);

        if is_host("music.163.com") {
            context.decision = Decision::Proxy;
        }

        // https://github.com/UnblockNeteaseMusic/server/blob/v0.27.0-beta.9/src/hook.js#L113

        if context.target_host.iter().any(|h| is_host(h))
            && *request.method() == Method::POST
            && (path == "/api/linux/forward" || path.starts_with("/eapi/"))
        {
            let netease_ctx = hook_netease_api(request).await?;

            match netease_ctx {
                Left(status) => {
                    info!("Unable to hook the Netease API: {}", status);
                }
                Right(mut netease_context) => {
                    if matches!(&netease_context.path, Some(v) if v == "/api/song/enhance/download/url")
                    {
                        netease_context = pretend_play(request, netease_context)?;
                    }
                    context.netease_context = Some(netease_context);
                }
            }
        } else if is_host(&hostname) && (path.starts_with("/weapi/") || path.starts_with("/api/")) {
            request
                .headers_mut()
                .insert("X-Real-IP", NETEASE_MOCK_IP.parse().unwrap());

            let mut ctx = NeteaseApiContext::new();
            ctx.web = true;
            ctx.path = Some({
                let api_replace_regex = Regex::new("^/webapi/").unwrap();
                let number_clear_regex = Regex::new("/\\d*$").unwrap();
                let replaced_path = api_replace_regex.replace(&path, "/api/");
                // The "clean" step removes the query parameters in the replaced_path.
                let cleaned_path = replaced_path
                    .into_owned()
                    .split('?')
                    .skip(1)
                    .collect::<String>();
                number_clear_regex
                    .replace(cleaned_path.as_str(), "")
                    .into_owned()
            });

            context.netease_context = Some(ctx);
        } else if url_str.contains("package/") {
            let mut data = url_str
                .split("package/")
                .last()
                .unwrap_or_log(
                    "BeforeRequestHook",
                    "package/ should be in url_str; use empty string instead.",
                    "",
                )
                .split('/');
            let url = {
                let data = data
                    .next()
                    .ok_or(BeforeRequestHookError::FailedToExtractUrl)?;
                let decoded = unm_crypto::base64::decode_crypto_base64(data)?;
                let decoded_str = String::from_utf8(decoded)?;

                Uri::from_str(&*decoded_str)?
            };
            let id = {
                let data = data
                    .next()
                    .ok_or(BeforeRequestHookError::FailedToExtractId)?;

                Regex::new(r#"\.\w+/"#).unwrap().replace(data, "")
            };

            *request.uri_mut() = url.clone();

            {
                let headers = request.headers_mut();
                headers.insert(
                    http::header::HOST,
                    url.host()
                        .ok_or(BeforeRequestHookError::FailedToExtractId)?
                        .parse()?,
                );
                headers.insert(http::header::COOKIE, "".parse()?);
            }

            context.package = Some(Package { id: id.to_string() });
            context.decision = Decision::Proxy;
        };

        Ok(())
    }
}
