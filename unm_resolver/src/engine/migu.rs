//! UNM Resolver [Engine]: Migu
//!
//! It can fetch audio from Migu Music.

use std::str::FromStr;

use anyhow::Ok;
use anyhow::Result;

use futures::future::join_all;
use http::HeaderMap;
use http::header::HeaderName;
use http::header::ORIGIN;
use http::header::REFERER;

use rand::Rng;
use rayon::iter::ParallelIterator;
use url::Url;
use http::Method;
use urlencoding::encode;
use crate::request::request;
use crate::utils::UnableToExtractJson;

use super::*;

/// The `migu` engine that can fetch audio from Migu Music.
pub struct MiguEngine;

#[async_trait]
impl Engine for MiguEngine {
    async fn check(&self, info: &Song, proxy: Option<Proxy>) -> Result<Option<String>> {
        match search(info, proxy.clone()).await? {
            None => Ok(None),
            Some(id) => Ok(track(id.as_str(), proxy, get_rand_num().as_str()).await?),
        }
    }
}

fn get_header() -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(ORIGIN, "http://music.migu.cn/".parse().unwrap());
    header.insert(REFERER, "http://m.music.migu.cn/v3/".parse().unwrap());
    header.insert(HeaderName::from_static("channel"), "0".parse().unwrap());
    // FIXME: 传入cookie
    //header.insert(HeaderName::from_static("aversionid"), cookie.parse().unwrap());

    header
}

fn get_rand_num() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0.0..1.0);
    num.to_string()
        .split('.')
        .skip(1).next()
        .expect("index 2: nothing there")
        .to_string()
}

async fn get_search_data(keyword: &str, proxy: Option<Proxy>) -> Result<Json> {
    let url_str = format!(
        "https://m.music.migu.cn/migu/remoting/scr_search_tag?keyword={0}&type=2&rows=20&pgc=1",
        encode(keyword)
    );
    let url = Url::from_str(&url_str)?;

    let res = request(Method::GET, &url, Some(get_header()), None, proxy).await?;
    Ok(res.json().await?)
}

async fn find_match(info: &Song, data: &[Json]) -> Result<Option<String>> {
    let list = data
        .par_iter()
        .map(|entry| format(entry).ok())
        .filter(|v| v.is_some())
        .map(|v| v.expect("should be Some"))
        .collect::<Vec<_>>();

    Ok(select_similar_song(&list, info).map(|song| song.id.to_string()))
}

async fn search(info: &Song, proxy: Option<Proxy>) -> Result<Option<String>> {
    let response = get_search_data(&info.keyword(), proxy).await?;
    let result = response
        .pointer("/musics")
        .ok_or(anyhow::anyhow!("/musics not found"))?
        .as_array()
        .ok_or(UnableToExtractJson("/musics", "array"))?;

    let matched = find_match(info, result).await?;

    Ok(matched)
}

async fn track(id: &str, proxy: Option<Proxy>, num: &str) -> Result<Option<String>> {
    // FIXME: 传入enabled_flac
    let enabled_flac = false;
    let qualities = if enabled_flac {
        vec!["ZQ", "SQ", "HQ", "PQ"]
     } else{
        vec!["HQ", "PQ"]
    };

    let futures = qualities
    .iter()
    .map(|&format| single(id, format, num, proxy.clone()));

    let urls = join_all(futures).await;
    let mut result = None;
    for u in urls {
        let o = u.ok();
        if o.is_some() {
            result = o;
            break;
        }
    }

    Ok(result)
}

fn format(song: &Json) -> Result<Song> {
    let id = song["id"]
        .as_str()
        .ok_or(UnableToExtractJson("id", "str"))?;
    let name = song["songName"]
        .as_str()
        .ok_or(UnableToExtractJson("name", "string"))?;
    let singer_id = song["singerId"]
        .as_str()
        .ok_or(UnableToExtractJson("singerId", "string"))?;
    let singer_name = song["singerName"]
        .as_str()
        .ok_or(UnableToExtractJson("singerName", "string"))?;

    let si: Vec<&str> = singer_id.split(",").collect();
    let sn: Vec<&str> = singer_name.split(",").collect();

    let mut artists = Vec::new();
    for index in 0..si.len() {
        artists.push(Artist{
            id: si.get(index).cloned().unwrap_or_default().to_string(),
            name: sn.get(index).cloned().unwrap_or_default().to_string()
        })
    }

    let x = Song {
        id: String::from(id),
        name: String::from(name),
        artists,
        ..Default::default()
    };
    Ok(x)
}

async fn get_single_data(id: &str, format: &str, num: &str, proxy: Option<Proxy>) -> Result<Json> {
    let url_str = format!(
        "https://app.c.nf.migu.cn/MIGUM2.0/strategy/listen-url/v2.2?lowerQualityContentId={0}&netType=01&resourceType=E&songId={1}&toneFlag={2}",
        encode(num),
        encode(id),
        encode(format),
    );
    let url = Url::from_str(&url_str)?;
    let res = request(Method::GET, &url, Some(get_header()), None, proxy).await?;
    Ok(res.json().await?)
}

async fn single(id: &str, format: &str, num: &str, proxy: Option<Proxy>) -> Result<String> {
    let response = get_single_data(id, format, num, proxy).await?;
    let format_type = response
        .pointer("/data/formatType")
        .ok_or(anyhow::anyhow!("/data/formatType not found"))?
        .as_str()
        .ok_or(UnableToExtractJson("formatType", "string"))?;
    let url = response
        .pointer("/data/url")
        .ok_or(anyhow::anyhow!("/data/url not found"))?
        .as_str()
        .ok_or(UnableToExtractJson("url", "string"))?;

    if format_type == format {
        Ok(String::from(url))
    } else {
        Err(anyhow::anyhow!("format not equals"))
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
    async fn migu_search() {
        let info = get_info_1();
        let id = search(&info, None).await.unwrap().unwrap();
        assert_eq!(id, "4300399".to_string());
    }

    #[test]
    async fn migu_search_json() {
        let info = get_info_1();
        let json = get_search_data(&info.keyword(), None).await.unwrap();
        println!("{}", json);
    }

    #[test]
    async fn migu_single() {
        let url = single("4300399", "PQ", get_rand_num().as_str(), None).await.unwrap();
        println!("{}", url);
    }

    #[test]
    async fn migu_single_json() {
        let json = get_single_data("4300399", "PQ", get_rand_num().as_str(), None).await.unwrap();
        println!("{}", json);
    }

    #[test]
    async fn migu_track() {
        let url = track("4300399", None, get_rand_num().as_str()).await.unwrap().unwrap();
        println!("{}", url);
    }

    #[test]
    async fn migu_check() {
        let p = MiguEngine;
        let info = get_info_1();
        let url = p.check(&info, None).await.unwrap().unwrap();
        println!("{}", url);
    }
}