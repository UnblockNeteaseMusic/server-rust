use urlencoding::encode;

use crate::error::*;
use crate::request::error::RequestError;
use crate::request::*;

use super::definitions::*;

pub struct BilibiliProvider {}

impl BilibiliProvider {
    /// find music id in bilibili
    async fn search(&self, info: &SongMetadata) -> ErrorResult<Option<i64>> {
        let url_str = format!(
            "https://api.bilibili.com/audio/music-service-c/s?\
			search_type=music&page=1&pagesize=30&\
			keyword=${0}",
            encode(info.keyword().as_str())
        );
        let res = request_str(Method::GET, url_str.as_str(), None, None, None).await?;
        let jsonbody = res
            .json::<Json>()
            .await
            .map_err(RequestError::RequestFail)?;
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

    /// trace music id and find out music link
    async fn track(&self, id: i64) -> ErrorResult<Option<String>> {
        let url_str = format!(
            "https://www.bilibili.com/audio/music-service-c/web/url?rivilege=2&quality=2&sid={0}",
            id
        );
        let res = request_str(Method::GET, url_str.as_str(), None, None, None).await?;
        let jsonbody = res
            .json::<Json>()
            .await
            .map_err(RequestError::RequestFail)?;
        let links = jsonbody["data"]["cdns"]
            .as_array()
            .ok_or(JsonErr::ParseError("data.cdns", "array"))?;
        if links.is_empty() {
            return Ok(None);
        }
        let link = links[0]
            .as_str()
            .ok_or(JsonErr::ParseError("data.cdns[0]", "string"))?
            .replace("https", "http");
        Ok(Some(link))
    }
}

#[async_trait]
impl Provide for BilibiliProvider {
    async fn check(&self, info: &SongMetadata) -> ErrorResult<Option<String>> {
        match self.search(info).await? {
            None => Ok(None),
            Some(id) => Ok(self.track(id).await?),
        }
    }
}

fn format(song: &Json) -> ErrorResult<SongMetadata> {
    let id = &song["id"]
        .as_i64()
        .ok_or(JsonErr::ParseError("id", "i64"))?;
    let name = song["title"]
        .as_str()
        .ok_or(JsonErr::ParseError("title", "string"))?;
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

    #[test]
    async fn bilibili_track() {
        let p = BilibiliProvider {};
        let url = p.track(349595).await.unwrap().unwrap();
        println!("{}", url);
    }

    #[test]
    async fn bilibili_check() {
        let p = BilibiliProvider {};
        let info = get_info_1();
        let url = p.check(&info).await.unwrap().unwrap();
        println!("{}", url);
    }
}
