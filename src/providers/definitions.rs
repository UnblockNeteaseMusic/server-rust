use async_trait::async_trait;
pub use serde_json::Value as Json;
use crate::error::*;


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
    pub artists: Vec<SongArtistsMetadata>,
    pub album: Option<SongAlbumMetadata>,
}

#[async_trait]
pub trait Provide {
    type SearchResultType;

    async fn check(info: &SongMetadata) -> Result<()>;
    async fn track(search_result: Self::SearchResultType) -> Result<()>;
}


impl SongMetadata {
    pub fn keyword(&self) -> String {
        let mut ret: String = String::new();
        ret.push_str(self.name.as_str());
        ret.push_str(" - ");
        
        let mut len = 0;
        for (idx, artist) in self.artists.iter().enumerate() {
            ret.push_str(artist.name.as_str());
            if idx > 0 {
                ret.push_str(" - ");
            };
            len += artist.name.len();
            if len > 15 {
                break;
            }
        }
        return ret;
    }

}
