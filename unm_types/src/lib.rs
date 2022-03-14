use reqwest::Proxy;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/**
 * The serialized identifier for passing to `retrieve()`.
 */
pub type SerializedIdentifier = String;

/// The metadata of the artist of a song.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Artist {
    /// The identifier of this artist.
    pub id: String,
    /// The name of this artist.
    pub name: String,
}

/// The metadata of the album of a song.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Album {
    /// The identifier of this artist.
    pub id: String,
    /// The name of this album.
    pub name: String,
    /// The song this album includes.
    pub songs: Vec<Song>,
}

/// The metadata of a song.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Song<C = ()> {
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
    /// The context of this song.
    ///
    /// For example, the URI identifier of this song.
    pub context: C,
}

/// The song identifier with the engine information.
#[derive(Clone, Serialize, Deserialize)]
pub struct SongSearchInformation<'a> {
    /// The retrieve source of this song, for example: `bilibili`.
    pub source: Cow<'a, str>,
    /// The serialized identifier of this song.
    pub identifier: SerializedIdentifier,
}

/// The information of the song retrieved with `retrieve()`.
#[derive(Clone, Serialize, Deserialize)]
pub struct RetrievedSongInfo<'a> {
    /// The retrieve source of this song, for example: `bilibili`.
    pub source: Cow<'a, str>,
    /// The URL of this song.
    pub url: String,
}

/// The context.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Context<'a> {
    /// The proxy URI
    pub proxy_uri: Option<String>,

    /// Whether to enable FLAC support.
    pub enable_flac: bool,

    /// Migu: The cookie "channel"
    pub migu_channel: Option<&'a str>,

    /// Migu: The cookie "aversionid"
    pub migu_aversionid: Option<&'a str>,
}

impl<'a> Context<'a> {
    pub fn try_get_proxy(&self) -> reqwest::Result<Option<Proxy>> {
        self.proxy_uri.clone().map(Proxy::all).transpose()
    }
}

impl Song {
    /// Generate the keyword of this song.
    pub fn keyword(&self) -> String {
        // {Song Name}
        let mut keyword = self.name.to_string();
        let max_idx = self.artists.len() - 1;

        // Add hyphen between the song name and the following artist name.
        keyword.push_str(" - ");

        for (idx, artist) in self.artists.iter().enumerate() {
            // "[keyword] {artist.name}"
            keyword.push_str(&artist.name);

            if idx != max_idx {
                // ", " if this is not the last item.
                keyword.push_str(", ");
            }
        }

        // {Song name} - {Artist 1's name}, {Artist 2's name}[, ...]
        keyword
    }
}
