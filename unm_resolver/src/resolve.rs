//! UNM Resolver: High-Level Resolver Methods

use std::fmt::{Debug, Display};

use bimap::BiMap;
use futures::FutureExt;
use log::{debug, error, info};
use once_cell::sync::Lazy;
pub use reqwest::Proxy;

use crate::engine::{
    bilibili::BilibiliEngine, migu::MiguEngine, pyncm::PyNCMEngine, ytdl::YtDlEngine,
    ytdlp::YtDlpEngine, Context, RetrievedSongInfo, SongSearchInformation, kugou::KugouEngine,
};
pub use crate::engine::{Engine as EngineTrait, Song};

/// The bidirectional map with the engine and the identifier.
static ENGINE_IDENTIFIER_MAP: Lazy<BiMap<Engine, &str>> = Lazy::new(|| {
    let mut elements = BiMap::new();
    elements.insert(Engine::Bilibili, "bilibili");
    elements.insert(Engine::PyNCM, "pyncm");
    elements.insert(Engine::Migu, "migu");
    elements.insert(Engine::YtDl, "ytdl");
    elements.insert(Engine::YtDlp, "ytdlp");
    elements.insert(Engine::Kugou, "kugou");
    elements
});

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
/// Engine: Migu Music
static KUGOU_ENGINE: Lazy<KugouEngine> = Lazy::new(|| KugouEngine);

/// The engine uses to resolve audio.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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
    /// Kugou Music.
    Kugou,
}

impl TryFrom<&str> for Engine {
    type Error = ResolveError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ENGINE_IDENTIFIER_MAP
            .get_by_right(&value)
            .copied()
            .ok_or_else(|| ResolveError::NoSuchEngine(value.to_string()))
    }
}

impl TryFrom<String> for Engine {
    type Error = ResolveError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            *ENGINE_IDENTIFIER_MAP
                .get_by_left(self)
                .unwrap_or(&"(<!> No engine identifier defined. Please report to developers!)")
        )
    }
}

impl Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl Engine {
    /// Reflect the explicit engine. `func` is the function
    /// to call with the reflected engine.
    fn reflect_engine<'a, T>(&self, func: impl Fn(&'a dyn EngineTrait) -> T) -> T {
        match self {
            Engine::Bilibili => func(&*BILIBILI_ENGINE),
            Engine::PyNCM => func(&*PYNCM_ENGINE),
            Engine::YtDlp => func(&*YTDLP_ENGINE),
            Engine::YtDl => func(&*YTDL_ENGINE),
            Engine::Migu => func(&*MIGU_ENGINE),
            Engine::Kugou => func(&*KUGOU_ENGINE)
        }
    }
}

#[async_trait::async_trait]
impl EngineTrait for Engine {
    async fn search<'a>(
        &self,
        info: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation<'static>>> {
        self.reflect_engine(|engine| engine.search(info, ctx)).await
    }

    async fn retrieve<'a>(
        &self,
        identifier: &'a crate::engine::SerializedIdentifier,
        ctx: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo<'static>> {
        self.reflect_engine(|engine| engine.retrieve(identifier, ctx))
            .await
    }
}

/// Batch search the `song` with the specified engines parallelly.
pub async fn batch_search<'a>(
    engines: &'a [Engine],
    info: &Song,
    context: &Context<'_>,
) -> anyhow::Result<SongSearchInformation<'a>> {
    let keyword = info.keyword();
    info!("Search {} with engines {:?}...", keyword, engines);

    let futures = engines.iter().map(|engine| {
        let keyword = keyword.clone();
        async move {
            debug!("Passing search parameters to engine {}", engine);

            let result = engine
                .search(info, context)
                .await
                .map_err(ResolveError::EngineError)?
                .ok_or(ResolveError::NoMatchedSong { keyword })?;

            // Specify the Error type explicitly.
            Ok::<SongSearchInformation, ResolveError>(result)
        }
        .boxed()
    });

    let selected_future = futures::future::select_ok(futures).await;

    match selected_future {
        Ok((result, _)) => {
            info!("Found {} with engine {}!", keyword, result.source);
            Ok(result)
        }
        Err(e) => {
            error!("{:?}", e);
            Err(e.into())
        }
    }
}

/// Retrieve the song with [`SongSearchInformation`].
pub async fn retrieve<'a>(
    info: &'a SongSearchInformation<'a>,
    context: &'a Context<'_>,
) -> anyhow::Result<RetrievedSongInfo<'static>> {
    let engine = info.source.as_ref().try_into() as Result<Engine, ResolveError>;
    let engine = engine.expect("must be a valid engine");

    engine.retrieve(&info.identifier, context).await
}

/// The error type of the resolve module.
#[derive(thiserror::Error, Debug)]
pub enum ResolveError {
    #[error("no matched song: {keyword}")]
    NoMatchedSong {
        /// The keyword of the song.
        keyword: String,
    },
    /// The internal error of the engine.
    #[error("engine error: {0}")]
    EngineError(anyhow::Error),
    #[error("no such engine: {0}")]
    NoSuchEngine(String),
}
/// The result type of the resolve module.
pub type ResolveResult<T> = Result<T, ResolveError>;
