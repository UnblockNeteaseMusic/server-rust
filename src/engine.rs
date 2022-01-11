//! UNM Resolver: Engine
//! 
//! "Engine" is a music platform unit, which can be used for
//! resolving the audio URL of a music.

pub use async_trait::async_trait;
pub use serde_json::Value as Json;

/// The metadata of the artist of a song.
#[derive(Clone, Default)]
pub struct Artist {
    /// The name of this artist.
    pub name: String,
    /// The Netease Cloud Music ID of this artist.
    pub ncm_id: Option<i64>,
}

/// The metadata of the album of a song.
#[derive(Clone, Default)]
pub struct Album {
    /// The name of this album.
    pub name: String,
    /// The Netease Cloud Music ID of this artist.
    pub ncm_id: Option<i64>,
    /// The song this album includes.
    pub songs: Vec<Song>,
}

/// The metadata of a song.
#[derive(Clone)]
pub struct Song {
    /// The name of this song.
    pub name: String,
    /// The duration of this song.
    pub duration: Option<i64>,
    /// The artist of this song.
    pub artists: Vec<Artist>,
    /// The album of this song.
    pub album: Option<Album>,
    /// The Netease Cloud Music ID of this song.
    pub ncm_id: Option<i64>,
}

#[async_trait]
/// The provider trait.
pub trait Provider {
    /// The result from [`Provider::check`].
    type CheckResult;

    /// Search a audio similar to the info from Provider,
    /// and return the audio link.
    async fn check(&self, info: &Song) -> Self::CheckResult;
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

/// iterate `list` and pick up a song which similar with `expect`
pub fn select_similar_song<'a>(
    list: &'a [Song],
    expect: &'a Song,
) -> Option<&'a Song> {
    if list.is_empty() {
        return None;
    }
    let duration = expect.duration.unwrap_or(i64::MAX);
    for (idx, i) in list.iter().enumerate() {
        // 只挑前五個結果
        if idx > 5 {
            break;
        }

        if let Some(d) = i.duration {
            if i64::abs(d - duration) < 5000 {
                // 第一个时长相差5s (5000ms) 之内的结果
                return Some(i);
            }
        }
    }
    // 没有就播放第一条
    Some(&list[0])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_engine_keyword() {
        let meta = Song {
            ncm_id: Some(114514),
            name: "U2FsdGVkX1".to_string(),
            duration: Some(7001),
            artists: vec![
                Artist {
                    ncm_id: Some(114514),
                    name: "elonh".to_string(),
                },
                Artist {
                    ncm_id: Some(114516),
                    name: "pan93412".to_string(),
                },
            ],
            album: Some(Album {
                ncm_id: Some(334511),
                name: "OWOOW".to_string(),
                ..Default::default()
            })
        };

        assert_eq!(meta.keyword(), "U2FsdGVkX1 - elonh, pan93412");
    }

    #[test]
    fn test_select() {
        let expect = gen_meta(Some(7001));
        let list = gen_metas(vec![Some(1000), Some(2000), Some(3000)]);
        let x = select_similar_song(&list, &expect).unwrap();
        assert_eq!(x.duration, list[2].duration);
        let list = gen_metas(vec![
            Some(1000),
            Some(2000),
            Some(3000),
            Some(4000),
            Some(5000),
            Some(6000),
        ]);
        let x = select_similar_song(&list, &expect).unwrap();
        assert_eq!(x.duration, list[2].duration);
        let list = gen_metas(vec![Some(1000)]);
        let x = select_similar_song(&list, &expect).unwrap();
        assert_eq!(x.duration, list[0].duration);
    }

    fn gen_meta(d: Option<i64>) -> Song {
        Song {
            album: None,
            artists: Vec::new(),
            duration: d,
            ncm_id: None,
            name: String::new(),
        }
    }

    fn gen_metas(ds: Vec<Option<i64>>) -> Vec<Song> {
        let mut res: Vec<Song> = Vec::new();
        for d in ds {
            res.push(gen_meta(d))
        }
        res
    }
}
