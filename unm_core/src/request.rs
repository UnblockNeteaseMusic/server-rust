pub use self::error::*;
use self::proxy_manager::ProxyManager;
use crate::request::header::default_headers;
pub use reqwest::{self, Method, Response};
pub use serde_json::{json, Value as Json};
pub use url::Url;

pub async fn request(
    method: Method,
    received_url: Url,
    received_headers: Option<Json>,
    body: Option<String>,
    proxy: Option<&ProxyManager>,
) -> RequestResult<Response> {
    let mut _headers = received_headers.clone();
    let headers = _headers.get_or_insert(json!({})).as_object_mut();
    if headers.is_none() {
        return Err(RequestError::HeadersDataInvalid);
    }

    let mut client_builder = reqwest::Client::builder()
        .gzip(true)
        .deflate(true)
        .default_headers(default_headers());
    client_builder = match proxy {
        None => client_builder.no_proxy(),
        Some(p) => match &p.as_ref() {
            Some(p) => client_builder.proxy(p.clone()),
            None => client_builder.no_proxy(),
        },
    };
    let client = client_builder.build().map_err(RequestError::RequestFail)?;
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
    ans.map_err(RequestError::RequestFail)
}

pub async fn request_str(
    method: Method,
    received_url: &str,
    received_headers: Option<Json>,
    body: Option<String>,
    proxy: Option<&ProxyManager>,
) -> RequestResult<Response> {
    let url = Url::parse(received_url).map_err(RequestError::UrlParseFail)?;
    return request(method, url, received_headers, body, proxy).await;
}

mod error;
mod header;
pub mod proxy_manager;
