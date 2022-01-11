// pub use async_trait::async_trait;
pub use serde_json::Value as Json;

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

// #[async_trait]
// pub trait Provide {
//     /// Search a audio similar with info from Provider,
//     /// and return the audio link
//     async fn check(&self, info: &SongMetadata) -> Self::Result<Option<String>>;
// }

impl SongMetadata {
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

#[cfg(test)]
mod bench {
    fn bench_keyword() {
        
    }
}