//! UNM Engine: PyNCM
//!
//! It can fetch audio from the unofficial
//! Netease Cloud Music API.

use log::{debug, info};
use serde::Deserialize;
use unm_engine::interface::Engine;
use unm_request::build_client;
use unm_types::{Context, RetrievedSongInfo, SerializedIdentifier, Song, SongSearchInformation};
use url::Url;

#[derive(Deserialize)]
#[non_exhaustive]
struct PyNCMResponse {
    /// The URL of this song.
    pub url: Option<String>,
}

pub const ENGINE_ID: &str = "pyncm";

/// The `pyncm` engine that can fetch audio from
/// the unofficial Netease Cloud Music API.
pub struct PyNCMEngine;

#[async_trait::async_trait]
impl Engine for PyNCMEngine {
    async fn search<'a>(
        &self,
        info: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation>> {
        info!("Searching with PyNCM engine…");

        let response = fetch_song_info(&info.id, ctx).await?;
        let match_result = response.url.map(|url| {
            SongSearchInformation::builder()
                .source(ENGINE_ID.into())
                .identifier(url)
                .build()
        });
        Ok(match_result)
    }

    async fn retrieve<'a>(
        &self,
        identifier: &'a SerializedIdentifier,
        _: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo> {
        info!("Retrieving with PyNCM engine…");

        // We just return the identifier as the URL of song.
        Ok(RetrievedSongInfo::builder()
            .source(ENGINE_ID.into())
            .url(identifier.to_string())
            .build())
    }
}

/// Fetch the song info in [`PyNCMResponse`].
async fn fetch_song_info(id: &str, ctx: &Context) -> anyhow::Result<PyNCMResponse> {
    debug!("Fetching the song information…");

    let bitrate = if ctx.enable_flac { 999 } else { 320 };
    let url = Url::parse_with_params(
        "https://music.gdstudio.xyz/api.php?types=url&source=netease",
        &[("id", id), ("br", &bitrate.to_string())],
    )?;
    let client = build_client(ctx.proxy_uri.as_deref())?;
    let response = client.get(url).send().await?;
    Ok(response.json::<PyNCMResponse>().await?)
}
#[cfg(test)]
mod tests {
    use unm_types::ContextBuilder;

    #[tokio::test]
    async fn test_fetch_song_info() {
        use super::fetch_song_info;

        let song_id = "1939601619"; // Madeon – Love You Back
        let result = fetch_song_info(song_id, &ContextBuilder::default().build().unwrap()).await;

        if let Ok(response) = result {
            assert!(response.url.is_some());
        } else {
            panic!("failed to fetch song info");
        }
    }
}
