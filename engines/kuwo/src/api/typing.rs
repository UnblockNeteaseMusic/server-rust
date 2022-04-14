use serde::Deserialize;
use unm_types::{Album, Artist, Song};

pub type MusicID = i64;

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct KuwoResponseList<T> {
    /// The total entries count.
    pub total: String,
    /// The entries.
    pub list: Vec<T>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct KuwoResponse<T> {
    /// The HTTP code of this response. Should be `200`.
    pub code: i32,
    /// The data part of this response.
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct KuwoAudioInfo {
    /// The music ID (MID) of this audio.
    pub rid: MusicID,

    /// The name of the audio.
    pub name: String,

    /// The duration of this song in second.
    pub duration: i64,

    /// The artist ID of this song.
    pub artistid: i64,

    /// The artist name of this song.
    pub artist: String,

    /// The album ID of this song.
    pub albumid: String,

    /// The album name of this song.
    pub album: String,

    /// Whether this song includes the Lossless version.
    pub has_lossless: bool,

    /// The flag for determining if this song need to pay.
    ///
    /// If the flag is `"0"`, this song is free to play.
    pub pay: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct KuwoPlayUrlData {
    /// The url of this MID.
    pub url: String,
}

pub type SearchResponse = KuwoResponse<KuwoResponseList<KuwoAudioInfo>>;
pub type GetPlayUrlResponse = KuwoResponse<KuwoPlayUrlData>;

impl From<KuwoAudioInfo> for Song {
    fn from(info: KuwoAudioInfo) -> Self {
        log::debug!("Converting KuwoAudioInfo to Song…");

        let artist = Artist::builder()
            .id(info.artistid.to_string())
            .name(info.artist)
            .build();

        let album = Album::builder()
            .id(info.albumid.to_string())
            .name(info.album)
            .build();

        Song::builder()
            .id(info.rid.to_string())
            .name(info.name)
            .duration(Some(info.duration * 1000))
            .artists(vec![artist])
            .album(Some(album))
            .build()
    }
}
