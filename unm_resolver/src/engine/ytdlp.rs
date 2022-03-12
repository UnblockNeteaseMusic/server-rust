//! UNM Resolver [Engine]: yt-dlp
//!
//! It can fetch audio from YouTube.

use std::borrow::Cow;

use serde::Deserialize;

use super::{Context, Engine, Song, SongSearchInformation, SerializedIdentifier, RetrievedSongInfo};

const ENGINE_NAME: &str = "ytdlp";

/// The response that `yt-dlp` will return.
#[derive(Deserialize)]
struct YtDlpResponse {
    /// The audio URL.
    url: String,
}

/// The search and track engine powered by `yt-dlp`.
pub struct YtDlpEngine;

#[async_trait::async_trait]
impl Engine for YtDlpEngine {
    // TODO: allow specifying proxy
    async fn check<'a>(&self, info: &'a Song, _: &'a Context) -> anyhow::Result<Option<String>> {
        Ok(fetch_from_youtube(&info.keyword()).await?.map(|r| r.url))
    }

    // TODO: allow specifying proxy
    async fn search<'a>(&self, info: &'a Song, _: &'a Context) -> anyhow::Result<Option<SongSearchInformation<'static>>> {
        let response = fetch_from_youtube(&info.keyword()).await?.map(|r| r.url);
        Ok(response.map(|url| SongSearchInformation {
            source: Cow::Borrowed(ENGINE_NAME),
            identifier: url,
        }))
    }

    async fn retrieve<'a>(&self, identifier: &'a SerializedIdentifier, _: &'a Context) -> anyhow::Result<RetrievedSongInfo<'static>> {
        Ok(RetrievedSongInfo {
            source: Cow::Borrowed(ENGINE_NAME),
            url: identifier.to_string(),
        })
    }
}

/// Get the response from `yt-dlp`.
///
/// ```plain
/// yt-dlp -f bestaudio --dump-json ytsearch1:{keyword}
///     -f bestaudio    choose the best quality of the audio
///     --dump-json     dump the information as JSON without downloading it
/// ```
async fn fetch_from_youtube(keyword: &str) -> anyhow::Result<Option<YtDlpResponse>> {
    let mut cmd = tokio::process::Command::new("yt-dlp");

    let child = cmd
        .args(&["-f", "bestaudio", "--dump-json"])
        .arg(format!("ytsearch1:{keyword}"))
        .kill_on_drop(true)
        .output()
        .await?;

    if child.status.success() {
        let response = String::from_utf8_lossy(&child.stdout);

        Ok(if response.is_empty() {
            None
        } else {
            let json = serde_json::from_str::<'_, YtDlpResponse>(&response)?;
            Some(json)
        })
    } else {
        log::error!("Failed to run `yt-dlp`.");
        log::error!("Code: {code:?}", code = child.status.code());
        log::error!("Stderr: {}", String::from_utf8_lossy(&child.stderr));

        Err(anyhow::anyhow!("Failed to run `yt-dlp`."))
    }
}
