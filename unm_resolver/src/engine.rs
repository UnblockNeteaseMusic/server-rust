//! UNM Resolver: Engine
//!
//! "Engine" is a music platform unit, which can be used for
//! resolving the audio URL of a music.

pub mod bilibili;
pub mod kugou;
pub mod migu;
pub mod pyncm;
pub mod ytdl;
pub mod ytdlp;

use std::borrow::Cow;

// Re-export types from unm_types for compatibility.
pub use unm_types::*;

pub use async_trait::async_trait;
pub use serde_json::Value as Json;

#[async_trait]
/// The engine that can search and track the specified [`Song`].
pub trait Engine {
    /// Search an audio matched the `info`, and
    /// return the identifier for retrieving audio URL with [`retrieve`].
    async fn search<'a>(
        &self,
        info: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation<'static>>>;
    // FIXME: allow dynamically generate the source name.

    /// Retrieve the audio URL of the specified `identifier`.
    async fn retrieve<'a>(
        &self,
        identifier: &'a SerializedIdentifier,
        ctx: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo<'static>>;
    // FIXME: allow dynamically generate the source name.
}

/// Construct a "similar song selector" to pass to `.find()`.
///
/// # Example
///
/// ```ignore
/// let (selector, optional_selector) = similar_song_selector_constructor(expected);
/// vec![Song {..Default::default()}].iter().find(selector);
/// vec![Some(Song::default()), None].iter().find(optional_selector)
/// ```
pub fn similar_song_selector_constructor<EC, LC>(
    expected: &Song<EC>,
) -> (
    impl Fn(&&Song<LC>) -> bool,
    impl Fn(&&Option<Song<LC>>) -> bool,
) {
    let expected_duration = expected.duration;
    let basic_func = move |song: &&Song<LC>| {
        if let Some(expected_duration) = expected_duration {
            if let Some(song_duration) = song.duration {
                // 第一个时长相差5s (5000ms) 之内的结果
                i64::abs(song_duration - expected_duration) < 5000
            } else {
                // 歌曲沒有長度，而期待有長度，則回傳 false。
                false
            }
        } else {
            // 沒有期待長度，則回傳 true 直接取出任一選擇。
            true
        }
    };

    (basic_func, move |song| {
        if let Some(s) = song {
            basic_func(&s)
        } else {
            false
        }
    })
}

/// iterate `list` and pick up a song which similar with `expect`
#[deprecated]
pub fn select_similar_song<'a, C>(list: &'a [Song<C>], expect: &'a Song) -> Option<&'a Song<C>> {
    if list.is_empty() {
        return None;
    }
    let duration = expect.duration.unwrap_or(i64::MAX);

    // 並行尋找所有相似歌曲
    // 如果沒有，就播放第一条
    Some(
        list.iter()
            .find(|song| {
                if let Some(d) = song.duration {
                    if i64::abs(d - duration) < 5000 {
                        // 第一个时长相差5s (5000ms) 之内的结果
                        return true;
                    }
                }

                false
            })
            .unwrap_or_else(|| &list[0]),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_engine_keyword() {
        let meta = Song {
            id: "114514".to_string(),
            name: "U2FsdGVkX1".to_string(),
            duration: Some(7001),
            artists: vec![
                Artist {
                    id: "114514".to_string(),
                    name: "elonh".to_string(),
                },
                Artist {
                    id: "114516".to_string(),
                    name: "pan93412".to_string(),
                },
            ],
            album: Some(Album {
                id: "334511".to_string(),
                name: "OWOOW".to_string(),
            }),
            ..Default::default()
        };

        assert_eq!(meta.keyword(), "U2FsdGVkX1 - elonh, pan93412");
    }

    #[test]
    fn test_select() {
        let expect = gen_meta(Some(7001));

        {
            let selector = similar_song_selector_constructor(&expect).0;
            let list = gen_metas(vec![Some(1000), Some(2000), Some(3000)]);
            let x = list.iter().find(selector).expect("must be Some");
            assert_eq!(x.duration, list[2].duration);
        }

        {
            let selector = similar_song_selector_constructor(&expect).0;
            let list = gen_metas(vec![
                Some(1000),
                Some(2000),
                Some(3000),
                Some(4000),
                Some(5000),
                Some(6000),
            ]);
            let x = list.iter().find(selector).expect("must be Some");
            assert_eq!(x.duration, list[2].duration);
        }

        {
            let selector = similar_song_selector_constructor(&expect).0;
            let list = gen_metas(vec![Some(1000)]);
            let x = list.iter().find(selector);
            assert!(matches!(x, None));
        }
    }

    fn gen_meta(d: Option<i64>) -> Song {
        Song {
            album: None,
            artists: Vec::new(),
            duration: d,
            name: String::new(),
            ..Default::default()
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
