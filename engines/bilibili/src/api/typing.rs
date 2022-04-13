use serde::Deserialize;
use unm_types::{Artist, Song};

pub type SearchResult = BilibiliApiResponse<BilibiliSearchApiData>;
pub type TrackResult = BilibiliApiResponse<BilibiliTrackApiData>;

#[derive(Debug, Clone, Deserialize)]
pub struct BilibiliSearchApiData {
    pub result: Vec<BilibiliSearchResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BilibiliTrackApiData {
    pub cdns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BilibiliApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BilibiliSearchResult {
    /// The song ID.
    pub id: i64,

    /// The song name.
    pub title: String,

    /// The artist ID.
    pub mid: i64,

    /// The artist name.
    pub author: String,
}

impl From<BilibiliSearchResult> for Song {
    fn from(result: BilibiliSearchResult) -> Self {
        log::debug!("Converting BilibiliSearchResult to Song…");

        Song {
            id: result.id.to_string(),
            name: result.title,
            duration: None,
            artists: vec![Artist {
                id: result.mid.to_string(),
                name: result.author,
            }],
            album: None,
            context: None,
        }
    }
}

impl BilibiliTrackApiData {
    pub fn get_music_url(&self) -> Option<String> {
        log::debug!("Getting the URL from BilibiliTrackApiData…");

        self.cdns.get(0).map(|s| s.replace("https", "http"))
    }
}
