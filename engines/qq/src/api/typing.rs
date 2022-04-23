use std::collections::HashMap;

use concat_string::concat_string;
use serde::Deserialize;
use unm_types::{Album, Artist, Song};

use super::identifier::QQResourceIdentifier;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct QQSongData {
    pub curnum: i32,
    pub curpage: i32,
    pub list: Vec<QQSongEntry>,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct QQSongEntry {
    /// The ID of this song entry.
    pub songid: i64,
    /// The name of this song entry.
    pub songname: String,
    /// The duration of this song entry, in seconds.
    pub interval: i64,

    /// The album ID of this song entry.
    pub albumid: i64,
    /// The album name of this song entry.
    pub albumname: String,

    /// The singers of this song entry.
    pub singer: Vec<QQSongSinger>,

    /// The media ID of this song entry.
    ///
    /// If no `media_mid` is provided, we return an empty string.
    /// You may want to filter it with [`Iterator::filter`].
    #[serde(default)]
    pub media_mid: String,
    /// The song MID of this song entry.
    pub songmid: String,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct QQSongSinger {
    pub id: i64,
    // pub mid: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct QQSingleResponseRoot {
    pub code: i64,
    pub data: Option<QQSingleResponse>,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct QQSingleResponse {
    /// The available servers to receive the audio.
    pub sip: Vec<String>,

    /// The segment of audio URLs to receive.
    pub midurlinfo: Vec<QQSingleUrlInfo>,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct QQSingleUrlInfo {
    /// The filename of this audio.
    pub filename: String,

    /// The URL segment of this audio.
    ///
    /// You can combine this segment with the server
    /// in the `sip` field of [`QQSingleResponse`].
    pub purl: String,
}

impl From<QQSongEntry> for Song {
    fn from(entry: QQSongEntry) -> Self {
        Song::builder()
            .id(entry.songid.to_string())
            .name(entry.songname)
            .duration(Some(entry.interval * 1000))
            .album(Some(
                Album::builder()
                    .id(entry.albumid.to_string())
                    .name(entry.albumname)
                    .build(),
            ))
            .artists(entry.singer.into_iter().map(Into::into).collect())
            .context({
                let mut ctx = HashMap::new();
                let songmid = entry.songmid;
                let media_mid = entry.media_mid;

                ctx.insert(
                    "identifier".into(),
                    QQResourceIdentifier {
                        mid: &songmid,
                        file: &media_mid,
                    }
                    .serialize(),
                );
                ctx.insert("songmid".into(), songmid);
                ctx.insert("media_mid".into(), media_mid);

                Some(ctx)
            })
            .build()
    }
}

impl From<QQSongSinger> for Artist {
    fn from(singer: QQSongSinger) -> Self {
        Artist::builder()
            .id(singer.id.to_string())
            .name(singer.name)
            .build()
    }
}

impl QQSingleResponse {
    pub fn get_url(&self) -> Result<String, FieldNotPickable> {
        log::info!("Extracting the URL from the single response…");

        let server = self
            .sip
            .get(fastrand::usize(0..self.sip.len()))
            .ok_or(FieldNotPickable("sip"))?;
        let url_info = self
            .midurlinfo
            .get(0)
            .ok_or(FieldNotPickable("midurlinfo"))?;

        Ok(concat_string!(server, url_info.purl))
    }
}

/// The error that will return if no element is in the
/// specified field (`self.0`) of the response.
#[derive(Debug)]
pub struct FieldNotPickable(&'static str);
impl std::fmt::Display for FieldNotPickable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to pick {} from response", self.0)
    }
}
impl std::error::Error for FieldNotPickable {}

#[cfg(test)]
mod tests {
    use super::{QQSingleResponse, QQSingleUrlInfo};

    #[test]
    fn test_single_response_get_url_with_single_data() {
        let single_response = QQSingleResponse {
            sip: vec!["http://helloworld.com/".into()],
            midurlinfo: vec![QQSingleUrlInfo {
                filename: "filename".into(),
                purl: "purl?114514".into(),
            }],
        };

        assert_eq!(
            single_response.get_url().unwrap(),
            "http://helloworld.com/purl?114514"
        );
    }

    #[test]
    fn test_single_response_get_url_with_multiple_sip() {
        let single_response = QQSingleResponse {
            sip: vec![
                "http://helloworld.com/".into(),
                "http://helloworld.org/".into(),
            ],
            midurlinfo: vec![QQSingleUrlInfo {
                filename: "filename".into(),
                purl: "purl?114514".into(),
            }],
        };

        let data = single_response.get_url().unwrap();
        assert!(vec![
            "http://helloworld.com/purl?114514",
            "http://helloworld.org/purl?114514"
        ]
        .contains(&data.as_str()));
    }

    #[test]
    fn test_single_response_get_url_with_multiple_midurlinfo() {
        let single_response = QQSingleResponse {
            sip: vec!["http://helloworld.com/".into()],
            midurlinfo: vec![
                QQSingleUrlInfo {
                    filename: "filename".into(),
                    purl: "purl?114514".into(),
                },
                QQSingleUrlInfo {
                    filename: "DO_NOT_PICK_THIS".into(),
                    purl: "!!!DONTPICKTHIS!!!".into(),
                },
            ],
        };

        assert_eq!(
            single_response.get_url().unwrap(),
            "http://helloworld.com/purl?114514"
        );
    }
}
