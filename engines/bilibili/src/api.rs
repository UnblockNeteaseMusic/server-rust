use http::Method;
use unm_request::request;
use unm_types::Context;
use url::Url;

use self::typing::{SearchResult, TrackResult};

pub mod typing;

pub async fn search(keyword: &str, context: &Context) -> anyhow::Result<SearchResult> {
    let url = Url::parse_with_params(
        "https://api.bilibili.com/audio/music-service-c/s",
        &[
            ("search_type", "music"),
            ("page", "1"),
            ("pagesize", "30"),
            ("keyword", keyword),
        ],
    )?;

    let response = request(Method::GET, &url, None, None, context.try_get_proxy()?).await?;
    Ok(response.json::<SearchResult>().await?)
}

pub async fn track(id: &str, context: &Context) -> anyhow::Result<TrackResult> {
    let url = Url::parse_with_params(
        "https://www.bilibili.com/audio/music-service-c/web/url",
        &[("rivilege", "2"), ("quality", "2"), ("sid", id)],
    )?;

    let response = request(Method::GET, &url, None, None, context.try_get_proxy()?).await?;
    Ok(response.json::<TrackResult>().await?)
}
