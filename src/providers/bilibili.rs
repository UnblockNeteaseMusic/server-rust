use urlencoding::encode;

use crate::error::*;
use crate::request::*;

use super::definitions::*;

pub struct BilibiliResult {}
pub struct BilibiliProvider {}

impl BilibiliProvider {
    // find music id in bilibili
    async fn search(&self, info: &SongMetadata) -> Result<Option<i64>> {
        let url_str = format!(
            "https://api.bilibili.com/audio/music-service-c/s?\
			search_type=music&page=1&pagesize=30&\
			keyword=${0}",
            encode(info.keyword().as_str())
        );
        let url = Url::parse(url_str.as_str()).map_err(Error::UrlParseFail)?;
        let res = request(Method::GET, url, None, None, None).await?;
        let jsonbody = res.json::<Json>().await.map_err(Error::RequestFail)?;
        let mut list: Vec<SongMetadata> = Vec::new();
        for item in jsonbody["data"]["result"]
            .as_array()
            .ok_or(JsonErr::ParseError("data.result", "array"))?
            .iter()
        {
            list.push(format(item)?);
        }
        let matched = select_similar_song(&list, info);
        match matched {
            None => Ok(None),
            Some(song) => Ok(Some(song.id)),
        }
    }
}

#[async_trait]
impl Provide for BilibiliProvider {
    type SearchResultType = Json;

    async fn check(_info: &SongMetadata) -> Result<()> {
        todo!()
    }

    async fn track(_search_result: Self::SearchResultType) -> Result<()> {
        todo!()
    }
}

fn format(song: &Json) -> Result<SongMetadata> {
    let id = &song["id"]
        .as_i64()
        .ok_or(JsonErr::ParseError("id", "i64"))?;
    let name = song["title"]
        .as_str()
        .ok_or(JsonErr::ParseError("name", "string"))?;
    let mid = &song["mid"]
        .as_i64()
        .ok_or(JsonErr::ParseError("mid", "i64"))?;
    let author = song["author"]
        .as_str()
        .ok_or(JsonErr::ParseError("author", "string"))?;
    let x = SongMetadata {
        id: *id,
        name: String::from(name),
        duration: None,
        album: None,
        artists: vec![SongArtistMetadata {
            id: *mid,
            name: String::from(author),
        }],
    };
    Ok(x)
}

#[cfg(test)]
mod test {
    use tokio::test;

    use super::*;

    fn get_info_1() -> SongMetadata {
        // https://music.163.com/api/song/detail?ids=[385552]
        SongMetadata {
            id: 385552,
            name: String::from("干杯"),
            album: None,
            artists: vec![SongArtistMetadata {
                id: 385552,
                name: String::from("五月天"),
            }],
            duration: None,
        }
    }

    #[test]
    async fn bilibili_search() {
        let p = BilibiliProvider {};
        let info = get_info_1();
        let id = p.search(&info).await.unwrap();
        println!("{:#?}", id);
        assert_eq!(id, Some(349595));
    }
}
