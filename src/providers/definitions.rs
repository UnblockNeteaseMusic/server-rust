pub use async_trait::async_trait;
pub use serde_json::Value as Json;

use crate::error::*;

#[derive(Clone)]
pub struct SongArtistMetadata {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct SongAlbumMetadata {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct SongMetadata {
    pub id: i64,
    pub name: String,
    pub duration: Option<i64>,
    pub artists: Vec<SongArtistMetadata>,
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

        ret
    }
}

// iterate `list` and pick up a song which similar with `expect`
pub fn select_similar_song<'a>(
    list: &'a [SongMetadata],
    expect: &'a SongMetadata,
) -> Option<&'a SongMetadata> {
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

    fn gen_meta(d: Option<i64>) -> SongMetadata {
        SongMetadata {
            album: None,
            artists: Vec::new(),
            duration: d,
            id: 0,
            name: String::new(),
        }
    }

    fn gen_metas(ds: Vec<Option<i64>>) -> Vec<SongMetadata> {
        let mut res: Vec<SongMetadata> = Vec::new();
        for d in ds {
            res.push(gen_meta(d))
        }
        res
    }
}
