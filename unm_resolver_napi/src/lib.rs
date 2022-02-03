use napi::bindgen_prelude::*;
use napi_derive::napi;
use unm_resolver::engine::{Album as RustAlbum, Artist as RustArtist, Context, Song as RustSong};
use unm_resolver::resolve::{resolve as rust_resolve, Engine as RustEngine};

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
}

impl From<Engine> for RustEngine {
    fn from(engine: Engine) -> Self {
        match engine {
            Engine::Bilibili => RustEngine::Bilibili,
            Engine::PyNCM => RustEngine::PyNCM,
            Engine::YtDlp => RustEngine::YtDlp,
            Engine::YtDl => RustEngine::YtDl,
        }
    }
}

/// (napi-rs) The metadata of the artist of a song.
#[derive(Clone, Default)]
#[napi(object)]
pub struct Artist {
    /// The identifier of this artist.
    pub id: String,
    /// The name of this artist.
    pub name: String,
}

impl From<Artist> for RustArtist {
    fn from(artist: Artist) -> Self {
        RustArtist {
            id: artist.id,
            name: artist.name,
        }
    }
}

/// (napi-rs) The metadata of the album of a song.
#[derive(Clone, Default)]
#[napi(object)]
pub struct Album {
    /// The identifier of this artist.
    pub id: String,
    /// The name of this album.
    pub name: String,
    /// The song this album includes.
    pub songs: Vec<Song>,
}

impl From<Album> for RustAlbum {
    fn from(album: Album) -> Self {
        RustAlbum {
            id: album.id,
            name: album.name,
            songs: album.songs.into_iter().map(Into::into).collect(),
        }
    }
}

/// (napi-rs) The metadata of a song.
#[derive(Clone, Default)]
#[napi(object)]
pub struct Song {
    /// The identifier of this song.
    pub id: String,
    /// The name of this song.
    pub name: String,
    /// The duration of this song.
    pub duration: Option<i64>,
    /// The artist of this song.
    pub artists: Vec<Artist>,
    /// The album of this song.
    pub album: Option<Album>,
}

impl From<Song> for RustSong {
    fn from(song: Song) -> Self {
        RustSong {
            id: song.id,
            name: song.name,
            duration: song.duration,
            artists: song.artists.into_iter().map(Into::into).collect(),
            album: song.album.map(Into::into),
        }
    }
}

/// (napi-rs) Resolve the `song` with the specified engines parallelly.
#[napi]

pub async fn resolve(engines: Vec<Engine>, info: Song) -> Result<String> {
    let engines = engines
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<RustEngine>>();

    rust_resolve(&engines, &info.into(), &Context::default())
        .await
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to resolve: {:?}", e),
            )
        }) //
}
