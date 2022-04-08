//! UNM Engine: Migu
//!
//! It can fetch audio from Migu Music.

use std::borrow::Cow;
use std::str::FromStr;

use anyhow::Ok;
use anyhow::Result;

use async_trait::async_trait;
use futures::future::join_all;
use http::header::HeaderName;
use http::header::ORIGIN;
use http::header::REFERER;
use http::HeaderMap;

use http::Method;
use rand::Rng;
use unm_engine::interface::Engine;
use unm_request::json::{Json, UnableToExtractJson};
use unm_request::request;
use unm_selector::SimilarSongSelector;
use unm_types::Artist;
use unm_types::Context;
use unm_types::RetrievedSongInfo;
use unm_types::SerializedIdentifier;
use unm_types::Song;
use unm_types::SongSearchInformation;
use url::Url;
use urlencoding::encode;

pub const ENGINE_ID: &str = "migu";

/// The `migu` engine that can fetch audio from Migu Music.
pub struct MiguEngine;

#[async_trait]
impl Engine for MiguEngine {
    async fn search<'a>(
        &self,
        info: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation<'static>>> {
        log::info!("Searching “{info}” with Migu engine…");

        let response = get_search_data(&info.keyword(), ctx).await?;
        let result = response
            .pointer("/musics")
            .ok_or_else(|| anyhow::anyhow!("/musics not found"))?
            .as_array()
            .ok_or(UnableToExtractJson {
                json_pointer: "/musics",
                expected_type: "array",
            })?;

        let matched = find_match(info, result).await?;

        Ok(matched.map(|identifier| SongSearchInformation {
            source: Cow::Borrowed(ENGINE_ID),
            identifier,
            song: None,
        }))
    }

    async fn retrieve<'a>(
        &self,
        identifier: &'a SerializedIdentifier,
        ctx: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo<'static>> {
        log::info!("Retrieving with Migu engine…");

        let num = get_rand_num();
        let enabled_flac = ctx.enable_flac;
        let qualities = if enabled_flac {
            vec!["ZQ", "SQ", "HQ", "PQ"]
        } else {
            vec!["HQ", "PQ"]
        };

        let futures = qualities
            .iter()
            .map(|&format| single(identifier, format, &num, ctx));

        let urls = join_all(futures).await;

        urls.into_iter()
            .find(|result_url| result_url.is_ok())
            .map(|result_url| result_url.expect("should be Some"))
            .map(|url| RetrievedSongInfo {
                source: Cow::Borrowed(ENGINE_ID),
                url,
            })
            .ok_or_else(|| anyhow::anyhow!("not able to retrieve identifier"))
    }
}

fn get_header(aversionid: Option<&str>) -> HeaderMap {
    log::debug!("Getting the header for request…");

    let mut header = HeaderMap::new();
    header.insert(ORIGIN, "http://music.migu.cn/".parse().unwrap());
    header.insert(REFERER, "http://m.music.migu.cn/v3/".parse().unwrap());
    header.insert(HeaderName::from_static("channel"), "0".parse().unwrap());

    if let Some(aversionid) = aversionid {
        header.insert(
            HeaderName::from_static("aversionid"),
            aversionid.parse().unwrap(),
        );
    }

    header
}

fn get_rand_num() -> String {
    log::debug!("Taking a random number…");

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0.0..1.0);
    num.to_string()
        .split('.')
        .nth(1)
        .expect("index 2: nothing there")
        .to_string()
}

async fn get_search_data(keyword: &str, ctx: &Context<'_>) -> Result<Json> {
    log::debug!("Send a search request of “{keyword}” to Migu Music…");

    let url_str = format!(
        "https://m.music.migu.cn/migu/remoting/scr_search_tag?keyword={0}&type=2&rows=20&pgc=1",
        encode(keyword)
    );
    let url = Url::from_str(&url_str)?;

    let res = request(
        Method::GET,
        &url,
        Some(get_header(ctx.migu_aversionid)),
        None,
        ctx.try_get_proxy()?,
    )
    .await?;
    Ok(res.json().await?)
}

async fn find_match(info: &Song, data: &[Json]) -> Result<Option<String>> {
    log::debug!("Finding the matched song from data…");

    let SimilarSongSelector {
        optional_selector, ..
    } = SimilarSongSelector::new(info);

    let similar_song = data
        .iter()
        .map(|entry| format(entry).ok())
        .find(|s| optional_selector(&s))
        .expect("should be Some");

    Ok(similar_song.map(|song| song.id))
}

fn format(song: &Json) -> Result<Song> {
    log::debug!("Formatting the Migu Music data to Song…");

    let id = song["id"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "id",
        expected_type: "str",
    })?;
    let name = song["songName"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "name",
        expected_type: "string",
    })?;
    let singer_id = song["singerId"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "singerId",
        expected_type: "string",
    })?;
    let singer_name = song["singerName"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "singerName",
        expected_type: "string",
    })?;

    let si: Vec<&str> = singer_id.split(',').collect();
    let sn: Vec<&str> = singer_name.split(',').collect();

    let mut artists = Vec::new();
    for index in 0..si.len() {
        artists.push(Artist {
            id: si.get(index).cloned().unwrap_or_default().to_string(),
            name: sn.get(index).cloned().unwrap_or_default().to_string(),
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

async fn get_single_data(id: &str, format: &str, num: &str, ctx: &Context<'_>) -> Result<Json> {
    log::debug!("Getting the single data…");

    let url_str = format!(
        "https://app.c.nf.migu.cn/MIGUM2.0/strategy/listen-url/v2.2?lowerQualityContentId={0}&netType=01&resourceType=E&songId={1}&toneFlag={2}",
        encode(num),
        encode(id),
        encode(format),
    );
    let url = Url::from_str(&url_str)?;
    let res = request(
        Method::GET,
        &url,
        Some(get_header(ctx.migu_aversionid)),
        None,
        ctx.try_get_proxy()?,
    )
    .await?;
    Ok(res.json().await?)
}

async fn single(id: &str, format: &str, num: &str, ctx: &Context<'_>) -> Result<String> {
    let response = get_single_data(id, format, num, ctx).await?;
    let format_type = response
        .pointer("/data/formatType")
        .ok_or_else(|| anyhow::anyhow!("/data/formatType not found"))?
        .as_str()
        .ok_or(UnableToExtractJson {
            json_pointer: "formatType",
            expected_type: "string",
        })?;
    let url = response
        .pointer("/data/url")
        .ok_or_else(|| anyhow::anyhow!("/data/url not found"))?
        .as_str()
        .ok_or(UnableToExtractJson {
            json_pointer: "url",
            expected_type: "string",
        })?;

    if format_type == format {
        Ok(String::from(url))
    } else {
        Err(anyhow::anyhow!("format not equals"))
    }
}

#[cfg(all(test, migu_test))]
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
        let info = MiguEngine
            .search(&info, &Context::default())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(info.source, ENGINE_NAME);
        assert_eq!(info.identifier, "4300399");
    }

    #[test]
    async fn migu_search_json() {
        let info = get_info_1();
        let json = get_search_data(&info.keyword(), &Context::default())
            .await
            .unwrap();
        println!("{}", json);
    }

    #[test]
    async fn migu_single() {
        let url = single(
            "4300399",
            "PQ",
            get_rand_num().as_str(),
            &Context::default(),
        )
        .await
        .unwrap();
        println!("{}", url);
    }

    #[test]
    async fn migu_single_json() {
        let json = get_single_data(
            "4300399",
            "PQ",
            get_rand_num().as_str(),
            &Context::default(),
        )
        .await
        .unwrap();
        println!("{}", json);
    }

    #[test]
    async fn migu_track() {
        let info = MiguEngine
            .retrieve(&String::from("4300399"), &Context::default())
            .await
            .unwrap();

        assert_eq!(info.source, ENGINE_NAME);
        println!("{}", info.url);
    }

    #[test]
    async fn migu_check() {
        let p = MiguEngine;
        let info = get_info_1();
        let url = p.check(&info, &Context::default()).await.unwrap().unwrap();
        println!("{}", url);
    }
}
