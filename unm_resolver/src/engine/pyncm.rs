//! UNM Resolver [Engine]: PyNCM
//!
//! It can fetch audio from the unofficial
//! Netease Cloud Music API.

use std::str::FromStr;

use http::Method;
use reqwest::Proxy;
use serde::Deserialize;

use crate::request::request;

use super::{Song, Provider};

#[derive(Deserialize)]
struct PyNCMResponse {
    /// The status code of this response.
    pub code: i32,
    pub data: Vec<PyNCMResponseEntry>,
}

#[derive(Deserialize)]
struct PyNCMResponseEntry {
    /// The NCM ID of this song.
    pub id: String,
    /// The URL of this song.
    pub url: Option<String>,
}

/// The `bilibili` provider that can fetch audio from
/// the unofficial Netease Cloud Music API.
pub struct PyNCMProvider;

impl PyNCMProvider {
    async fn track(&self, song: &Song, enable_flac: bool, proxy: Option<Proxy>) -> anyhow::Result<Option<String>> {
        let url_str = format!(
            "http://mos9527.tooo.top/ncm/pyncm/track/GetTrackAudio?song_ids={id}&bitrate={bitrate}",
            id = song.id,
            bitrate = if enable_flac { 999000 } else { 320000 }
        );
        let url = url::Url::from_str(&url_str)?;

        let response = request(Method::GET, &url, None, None, proxy).await?;
        let res_json = response.json::<PyNCMResponse>().await?;

		if res_json.code == 200 {
			let matched = res_json
            .data
            .into_iter()
            .find(|entry| entry.id == song.id && entry.url.is_some())
            .ok_or(anyhow::anyhow!("no matched song"))?
            .url;

        	Ok(matched)
		} else {
			Err(anyhow::anyhow!("failed to request. code: {}", res_json.code))
		}
    }
}

#[async_trait::async_trait]
impl Provider for PyNCMProvider {
    async fn check(&self, info: &Song, proxy: Option<Proxy>) -> anyhow::Result<Option<String>> {
		// FIXME: enable_flac should be configuable by users.
		self.track(info, cfg!(ENABLE_FLAC), proxy).await
	}
}
