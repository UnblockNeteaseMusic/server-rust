//! UNM Resolver: High-Level Resolver Methods

use futures::FutureExt;
use log::{error, info};
use once_cell::sync::Lazy;
pub use reqwest::Proxy;

use crate::engine::{
    bilibili::BilibiliEngine, migu::MiguEngine, pyncm::PyNCMEngine, ytdl::YtDlEngine,
    ytdlp::YtDlpEngine, Context,
};
pub use crate::engine::{Engine as EngineTrait, Song};

/// Engine: Bilibili Music
static BILIBILI_ENGINE: Lazy<BilibiliEngine> = Lazy::new(|| BilibiliEngine);
/// Engine: PyNCM
static PYNCM_ENGINE: Lazy<PyNCMEngine> = Lazy::new(|| PyNCMEngine);
/// Engine: yt-dlp
static YTDLP_ENGINE: Lazy<YtDlpEngine> = Lazy::new(|| YtDlpEngine);
/// Engine: youtube-dl
static YTDL_ENGINE: Lazy<YtDlEngine> = Lazy::new(|| YtDlEngine);
/// Engine: Migu Music
static MIGU_ENGINE: Lazy<MiguEngine> = Lazy::new(|| MiguEngine);

/// The engine uses to resolve audio.
#[derive(Debug)]
pub enum Engine {
    /// Bilibili Music.
    Bilibili,
    /// Unoffical Netease Cloud Music API
    PyNCM,
    /// YouTube with `yt-dlp`.
    YtDlp,
    /// YouTube with `youtube-dl`.
    YtDl,
    /// Migu Music.
    Migu,
}

#[async_trait::async_trait]
impl EngineTrait for Engine {
    async fn check<'a>(
        &self,
        info: &'a Song,
        context: &'a Context,
    ) -> anyhow::Result<Option<String>> {
        let result = match self {
            Engine::Bilibili => BILIBILI_ENGINE.check(info, context),
            Engine::PyNCM => PYNCM_ENGINE.check(info, context),
            Engine::YtDlp => YTDLP_ENGINE.check(info, context),
            Engine::YtDl => YTDL_ENGINE.check(info, context),
            Engine::Migu => MIGU_ENGINE.check(info, context),
        };

        result.await
    }
}

/// Resolve the `song` with the specified engines parallelly.
pub async fn resolve(
    engines: &[Engine],
    info: &Song,
    context: &Context<'_>,
) -> ResolveResult<String> {
    let keyword = info.keyword();
    info!("Resolving: {}", keyword);

    let futures = engines.iter().map(|engine| {
        let keyword = keyword.clone();
        async move {
            info!("Resolving with engine: {:?}", engine);

            let result = engine
                .check(info, context)
                .await
                .map_err(ResolveError::EngineError)?
                .ok_or(ResolveError::NoMatchedSong { keyword })?;

            // Specify the Error type explicitly.
            Ok::<String, ResolveError>(result)
        }
        .boxed()
    });

    let selected_future = futures::future::select_ok(futures).await;

    match selected_future {
        Ok((result, _)) => {
            info!("{} resolved: {}", info.keyword(), result);
            Ok(result)
        }
        Err(e) => {
            error!("{:?}", e);
            Err(e)
        }
    }
}

/// The error type of the resolve module.
#[derive(thiserror::Error, Debug)]
pub enum ResolveError {
    /// No matched song.
    #[error("no matched song: {keyword}")]
    NoMatchedSong {
        /// The keyword of the song.
        keyword: String,
    },
    /// The internal error of the engine.
    #[error("Engine error: {0}")]
    EngineError(anyhow::Error),
}
/// The result type of the resolve module.
pub type ResolveResult<T> = Result<T, ResolveError>;
