use napi_derive::napi;
use napi::bindgen_prelude::*;

#[napi]
pub enum Engine {
    Bilibili,
    Kugou,
    Migu,
    PyNCM,
    YtDl,
}

impl Engine {
    pub fn as_str(&self) -> &'static str {
        match self {
            Engine::Bilibili => "bilibili",
            Engine::Kugou => "kugou",
            Engine::Migu => "migu",
            Engine::PyNCM => "pyncm",
            Engine::YtDl => "ytdl",
        }
    }
}
