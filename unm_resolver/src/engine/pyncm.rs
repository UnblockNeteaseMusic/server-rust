//! UNM Resolver [Engine]: PyNCM
//!
//! It can fetch audio from the unofficial
//! Netease Cloud Music API.

use rayon::prelude::*;
use std::{str::FromStr, borrow::Cow};

use http::Method;
use serde::Deserialize;

use crate::request::request;

use super::{Context, Engine, Song, SongSearchInformation, RetrievedSongInfo, SerializedIdentifier};

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

const ENGINE_NAME: &str = "pyncm";

/// The `pyncm` engine that can fetch audio from
/// the unofficial Netease Cloud Music API.
pub struct PyNCMEngine;

#[async_trait::async_trait]
impl Engine for PyNCMEngine {
    async fn search<'a>(&self, info: &'a Song, ctx: &'a Context) -> anyhow::Result<Option<SongSearchInformation<'static>>> {
        let response = fetch_song_info(&info.id, ctx).await?;

        if response.code == 200 {
            let match_result = find_match(&response.data, &info.id)?
                .map(|url| SongSearchInformation {
                    source: Cow::Borrowed(ENGINE_NAME),
                    identifier: url, // FIXME: hacky way to search with PyNCM
                });

            Ok(match_result)
        } else {
            Err(anyhow::anyhow!(
                "failed to request. code: {}",
                response.code
            ))
        }
    }

    async fn retrieve<'a>(&self, identifier: &'a SerializedIdentifier, _: &'a Context) -> anyhow::Result<RetrievedSongInfo<'static>> {
        Ok(RetrievedSongInfo {
            source: Cow::Borrowed(ENGINE_NAME),
            url: identifier.to_string(),
        })
    }
}

/// Fetch the song info in [`PyNCMResponse`].
async fn fetch_song_info(
    id: &str,
    ctx: &Context<'_>,
) -> anyhow::Result<PyNCMResponse> {
    let url_str = format!(
        "http://mos9527.tooo.top/ncm/pyncm/track/GetTrackAudio?song_ids={id}&bitrate={bitrate}",
        id = id,
        bitrate = if ctx.enable_flac { 999000 } else { 320000 }
    );
    let url = url::Url::from_str(&url_str)?;

    let response = request(Method::GET, &url, None, None, ctx.proxy.cloned()).await?;
    Ok(response.json::<PyNCMResponse>().await?)
}

/// Find the matched song from an array of [`PyNCMResponseEntry`].
fn find_match(data: &[PyNCMResponseEntry], song_id: &str) -> anyhow::Result<Option<String>> {
    data.par_iter()
        .find_any(|entry| {
            // Test if the ID of this entry matched what we want to fetch,
            // and there is content in its URL.
            entry.id == song_id && entry.url.is_some()
        })
        .map(|v| v.url.clone())
        .ok_or_else(|| anyhow::anyhow!("no matched song"))
}
