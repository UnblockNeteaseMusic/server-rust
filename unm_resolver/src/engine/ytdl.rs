//! UNM Resolver [Engine]: youtube-dl
//!
//! Better to use `yt-dlp` instead of `youtube-dl`
//! since the latter do not maintain actively now.

use serde::Deserialize;

use super::{Context, Engine, Song};

/// The response that `youtube-dl` will return.
#[derive(Deserialize)]
struct YtDlResponse {
    /// The audio URL.
    url: String,
}

/// The search and track engine powered by `youtube-dl`.
pub struct YtDlEngine;

#[async_trait::async_trait]
impl Engine for YtDlEngine {
    // TODO: allow specifying proxy
    async fn check<'a>(&self, info: &'a Song, _: &'a Context) -> anyhow::Result<Option<String>> {
        Ok(fetch_from_youtube(&info.keyword()).await?.map(|r| r.url))
    }
}

/// Get the response from `youtube-dl`.
///
/// ```plain
/// youtube-dl -f bestaudio --dump-json ytsearch1:{keyword}
///     -f bestaudio    choose the best quality of the audio
///     --dump-json     dump the information as JSON without downloading it
/// ```
async fn fetch_from_youtube(keyword: &str) -> anyhow::Result<Option<YtDlResponse>> {
    let mut cmd = tokio::process::Command::new("yt-dl");

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
            let json = serde_json::from_str::<'_, YtDlResponse>(&response)?;
            Some(json)
        })
    } else {
        log::error!("Failed to run `youtube-dl`.");
        log::error!("Code: {code:?}", code = child.status.code());
        log::error!("Stderr: {}", String::from_utf8_lossy(&child.stderr));

        Err(anyhow::anyhow!("Failed to run `youtube-dl`."))
    }
}
