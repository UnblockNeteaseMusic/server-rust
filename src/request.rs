use reqwest::{self, Method, Response};
use serde_json::{json, Value as Json};
pub use tokio::sync::oneshot::Receiver;
use url::Url;

use crate::error::*;

use self::{header::default_headers, proxy::ProxyManager};

pub mod header;
pub mod proxy;

pub async fn request(
    method: Method,
    received_url: Url,
    received_headers: Option<Json>,
    body: Option<String>,
    proxy: Option<&ProxyManager>,
) -> Result<Response> {
    let mut _headers = received_headers.clone();
    let headers = _headers.get_or_insert(json!({})).as_object_mut();
    if headers.is_none() {
        return Err(Error::HeadersDataInvalid);
    }

    let mut client_builder = reqwest::Client::builder()
	.user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.181 Safari/537.36")
	.gzip(true).deflate(true)
	.default_headers(default_headers());
    client_builder = match proxy {
        None => client_builder.no_proxy(),
        Some(p) => match &p.as_ref() {
            Some(p) => client_builder.proxy(p.clone()),
            None => client_builder.no_proxy(),
        },
    };
    let client = client_builder.build()?;
    let mut client = client.request(method, received_url);

    for (key, val) in headers.unwrap() {
        match val.as_str() {
            None => {}
            Some(v) => client = client.header(key, v),
        };
    }

    if body.is_some() {
        client = client.body(body.unwrap());
    }
    let ans = client.send().await;
    ans.map_err(|e: reqwest::Error| Error::RequestFail(e))
}
