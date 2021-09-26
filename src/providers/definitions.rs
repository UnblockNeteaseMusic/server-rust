use async_trait::async_trait;
pub use serde_json::Value as Json;

pub struct SongArtistsMetadata {
    pub id: i32,
    pub name: String,
}

pub struct SongAlbumMetadata {
    pub id: i32,
    pub name: String,
}

pub struct SongMetadata {
    pub id: i32,
    pub name: String,
    pub duration: Option<u64>,
    pub artists: Option<SongArtistsMetadata>,
    pub album: Option<SongAlbumMetadata>,
}

#[async_trait]
pub trait Provider {
    type SearchResultType;

    async fn check(info: &Json) -> SongMetadata;
    async fn track(search_result: Self::SearchResultType) -> SongMetadata;
}
