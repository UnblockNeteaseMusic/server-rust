//! UNM Engine: ytdl
//!
//! It can fetch audio from YouTube with
//! the specified `youtube-dl`-like command.
//!
//! The default is `yt-dlp`. You can configure it by passing
//! `ytdl:exe` in the ctx.config [`HashMap`] field.

use std::borrow::Cow;

use concat_string::concat_string;
use log::{debug, info};
use serde::Deserialize;
use unm_engine::interface::Engine;
use unm_types::{
    config::ConfigManager, Artist, Context, RetrievedSongInfo, SerializedIdentifier, Song,
    SongSearchInformation,
};

pub const DEFAULT_EXECUTABLE: &str = "yt-dlp";
pub const ENGINE_ID: &str = "ytdl";

/// The response that the `youtube-dl` instance will return.
#[derive(Deserialize)]
#[non_exhaustive]
struct YtDlResponse {
    /// The YouTube video ID.
    id: String,
    /// The YouTube video title.
    title: String,
    /// The audio URL.
    url: String,
    /// The duration of this audio (sec).
    duration: i32,
    /// The uploader's YouTube channel ID.
    uploader_id: String,
    /// The uploader's YouTube channel name.
    uploader: String,
}

/// The search and track engine powered by the `youtube-dl`-like command.
pub struct YtDlEngine;

#[async_trait::async_trait]
impl Engine for YtDlEngine {
    async fn search<'a>(
        &self,
        info: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation<'static>>> {
        let exe = decide_ytdl_exe(&ctx.config);

        info!("Searching for {info} with {exe}…");

        let response = fetch_from_youtube(exe, &info.keyword(), ctx.proxy_uri.as_deref()).await?;

        // We return the URL we got from youtube-dl as the song identifier,
        // so we can return the URL in retrieve() easily.
        if let Some(response) = response {
            let url = response.url.to_string();
            let song = Song::from(response);
            Ok(Some(SongSearchInformation {
                source: Cow::Borrowed(ENGINE_ID),
                identifier: url,
                song: Some(song),
            }))
        } else {
            Ok(None)
        }
    }

    async fn retrieve<'a>(
        &self,
        identifier: &'a SerializedIdentifier,
        _: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo<'static>> {
        info!("Retrieving {identifier}…");

        // We just return the identifier as the URL of song.
        Ok(RetrievedSongInfo {
            source: Cow::Borrowed(ENGINE_ID),
            url: identifier.to_string(),
        })
    }
}

fn decide_ytdl_exe(config: &Option<ConfigManager>) -> &str {
    debug!("Deciding the executable to use in `ytdl` engine…");

    config
        .as_ref()
        .map(|c| c.get_or_default(Cow::Borrowed("ytdl:exe"), DEFAULT_EXECUTABLE))
        .unwrap_or(DEFAULT_EXECUTABLE)
}

/// Get the response from `<exe>`.
///
/// The `<exe>` should be a `youtube-dl`-like command,
/// such as `yt-dlp` or `youtube-dl`.
///
/// ```plain
/// <exe> -f bestaudio --dump-json [--proxy <proxy>] ytsearch1:<keyword>
///     -f bestaudio    choose the best quality of the audio
///     --dump-json     dump the information as JSON without downloading it
///     --proxy URL     Use the specified HTTP/HTTPS/SOCKS proxy.
///                     To enable SOCKS proxy, specify a proper
///                     scheme. For example
///                     socks5://user:pass@127.0.0.1:1080/. Pass in
///                     an empty string (--proxy "") for direct
///                     connection
/// ```
async fn fetch_from_youtube(
    exe: &str,
    keyword: &str,
    proxy: Option<&str>,
) -> anyhow::Result<Option<YtDlResponse>> {
    info!("Calling external application “{exe}”!");

    let mut cmd = tokio::process::Command::new(exe);

    debug!("Receiving the search result from {exe}…");

    // <cmd> -f bestaudio --dumpjson
    cmd.args(&["-f", "bestaudio", "--dump-json"]);

    // --proxy <proxy>
    if let Some(proxy) = proxy {
        cmd.args(&["--proxy", proxy]);
    }

    // search query
    cmd.arg(concat_string!("ytsearch1:", keyword));

    let child = cmd.kill_on_drop(true).output().await?;

    if child.status.success() {
        let response = String::from_utf8_lossy(&child.stdout);

        Ok(if response.is_empty() {
            None
        } else {
            debug!("Serializing the search result…");
            let json = serde_json::from_str::<'_, YtDlResponse>(&response)?;
            Some(json)
        })
    } else {
        log::error!("Failed to run `{exe}`.");
        log::error!("Code: {code:?}", code = child.status.code());
        log::error!("Stderr: {}", String::from_utf8_lossy(&child.stderr));

        Err(anyhow::anyhow!("Failed to run `{exe}`."))
    }
}

impl From<YtDlResponse> for Song {
    fn from(res: YtDlResponse) -> Self {
        debug!("Formatting response…");

        Song {
            id: res.id,
            name: res.title,
            artists: vec![Artist {
                id: res.uploader_id,
                name: res.uploader,
            }],
            duration: Some(res.duration as i64 * 1000),
            album: None,
            context: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_decide_ytdl_exe() {
        use super::*;

        assert_eq!(decide_ytdl_exe(&None), DEFAULT_EXECUTABLE);

        let config = HashMap::new();
        let config = ConfigManager::new(config);
        assert_eq!(decide_ytdl_exe(&Some(config)), DEFAULT_EXECUTABLE);

        let mut config = HashMap::with_capacity(1);
        config.insert(Cow::Borrowed("ytdl:exe"), "youtube-dl".to_string());
        let config = ConfigManager::new(config);
        assert_eq!(decide_ytdl_exe(&Some(config)), "youtube-dl");
    }
}
