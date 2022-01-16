//! UNM Resolver [Engine]: yt-dlp
//!
//! It can fetch audio from YouTube.

use serde::Deserialize;

use super::{Engine, Song, Proxy};

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
    async fn check(&self, info: &Song, _: Option<Proxy>) -> anyhow::Result<Option<String>> {
        Ok(fetch_from_youtube(&info.keyword()).await?.map(|r| r.url))
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
        log::error!("Failed to run `youtube-dl`.");
        log::error!("Code: {code:?}", code = child.status.code());
        log::error!("Stderr: {}", String::from_utf8_lossy(&child.stderr));

        Err(anyhow::anyhow!("Failed to run `youtube-dl`."))
    }
}
