//! UNM Resolver [Engine]: Bilibili
//! 
//! It can fetch audio from Bilibili Music.

use std::str::FromStr;

use http::Method;
use url::Url;
use urlencoding::encode;
use crate::request::request;

use super::*;

/// The `bilibili` provider that can fetch audio from Bilibili Music.
pub struct BilibiliProvider;

impl BilibiliProvider {
    /// find music id in bilibili
    async fn search(&self, info: &Song, proxy: Option<Proxy>) -> anyhow::Result<Option<String>> {
        let url_str = format!(
            "https://api.bilibili.com/audio/music-service-c/s?\
			search_type=music&page=1&pagesize=30&\
			keyword=${0}",
            encode(info.keyword().as_str())
        );
        let url = Url::from_str(&url_str)?;

        let res = request(Method::GET, &url, None, None, proxy).await?;
        let jsonbody = res.json::<Json>().await?;
        let mut list = Vec::new();
        let list_json = jsonbody["data"]["result"]
            .as_array().ok_or(UnableToExtractJson("data.result", "array"))?;

        for entry in list_json {
            list.push(format(entry)?);
        }
        
        let matched = select_similar_song(&list, info).map(|song| song.id.to_string());

        Ok(matched)
    }

    /// trace music id and find out music link
    async fn track(&self, id: String) -> anyhow::Result<Option<String>> {
        let url_str = format!(
            "https://www.bilibili.com/audio/music-service-c/web/url?rivilege=2&quality=2&sid={0}",
            id
        );
        let url = Url::from_str(&url_str)?;

        let res = request(Method::GET, &url, None, None, None).await?;
        let jsonbody = res.json::<Json>().await?;
        let links = jsonbody["data"]["cdns"]
            .as_array().ok_or(UnableToExtractJson("data.cdns", "array"))?;

        if links.is_empty() {
            return Ok(None);
        }

        let link = links[0]
            .as_str().ok_or(UnableToExtractJson("data.cdns[0]", "string"))?
            .replace("https", "http");

        Ok(Some(link))
    }
}

#[async_trait]
impl Provider for BilibiliProvider {
    async fn check(&self, info: &Song, proxy: Option<Proxy>) -> anyhow::Result<Option<String>> {
        match self.search(info, proxy).await? {
            None => Ok(None),
            Some(id) => Ok(self.track(id).await?),
        }
    }
}

fn format(song: &Json) -> anyhow::Result<Song> {
    let id = song["id"]
        .as_i64()
        .ok_or(UnableToExtractJson("id", "i64"))?;
    let name = song["title"]
        .as_str()
        .ok_or(UnableToExtractJson("title", "string"))?;
    let mid = song["mid"]
        .as_i64()
        .ok_or(UnableToExtractJson("mid", "i64"))?;
    let author = song["author"]
        .as_str()
        .ok_or(UnableToExtractJson("author", "string"))?;
    let x = Song {
        id: id.to_string(),
        name: String::from(name),
        artists: vec![Artist {
            id: mid.to_string(),
            name: String::from(author),
        }],
        ..Default::default()
    };
    Ok(x)
}

#[derive(Debug)]
struct UnableToExtractJson<'a>(&'a str, &'a str);
impl<'a> std::error::Error for UnableToExtractJson<'a> {}
impl<'a> std::fmt::Display for UnableToExtractJson<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to extract json: {} (type: {})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use tokio::test;

    use super::*;

    fn get_info_1() -> Song {
        // https://music.163.com/api/song/detail?ids=[385552]
        Song {
            name: String::from("干杯"),
            artists: vec![Artist {
                name: String::from("五月天"),
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    #[test]
    async fn bilibili_search() {
        let p = BilibiliProvider {};
        let info = get_info_1();
        let id = p.search(&info, None).await.unwrap();
        assert_eq!(id, Some("349595".to_string()));
    }

    #[test]
    async fn bilibili_track() {
        let p = BilibiliProvider;
        let url = p.track("349595".to_string()).await.unwrap().unwrap();
        println!("{}", url);
    }

    #[test]
    async fn bilibili_check() {
        let p = BilibiliProvider;
        let info = get_info_1();
        let url = p.check(&info, None).await.unwrap().unwrap();
        println!("{}", url);
    }
}
