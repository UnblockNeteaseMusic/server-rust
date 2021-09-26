use std::any::Any;

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

pub trait Provider {
    type SearchResultType;

    fn check(info: Box<dyn Any>) -> SongMetadata;
    fn track(search_result: Self::SearchResultType) -> SongMetadata;
}
