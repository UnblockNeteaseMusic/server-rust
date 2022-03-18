use napi::bindgen_prelude::*;
use napi_derive::napi;
pub(crate) use unm_resolver::resolve::Engine as RustEngine;

/// (napi-rs) The engine uses to resolve audio.
#[napi]
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

impl From<Engine> for RustEngine {
    fn from(engine: Engine) -> Self {
        match engine {
            Engine::Bilibili => RustEngine::Bilibili,
            Engine::PyNCM => RustEngine::PyNCM,
            Engine::YtDlp => RustEngine::YtDlp,
            Engine::YtDl => RustEngine::YtDl,
            Engine::Migu => RustEngine::Migu,
            Engine::Kugou => RustEngine::Kugou,
        }
    }
}
