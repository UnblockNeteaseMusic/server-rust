use std::any::Any;

pub struct SongArtistsMetadata {
    id: i32,
    name: String,
}

pub struct SongAlbumMetadata {
    id: i32,
    name: String,
}

pub struct SongMetadata {
    id: i32,
    name: String,
    duration: Option<u64>,
    artists: Option<SongArtistsMetadata>,
    album: Option<SongAlbumMetadata>,
}

pub trait Provider {
    type SearchResultType;

    fn check(info: Box<dyn Any>) -> SongMetadata;
    fn track(search_result: Self::SearchResultType) -> SongMetadata;
}
