//! UNM Engine: Bilibili
//!
//! It can fetch audio from Bilibili Music.

use async_trait::async_trait;
use log::{debug, info};
use unm_engine::interface::Engine;
use unm_selector::SimilarSongSelector;
use unm_types::{
    Artist, Context, RetrievedSongInfo, SerializedIdentifier, Song, SongSearchInformation,
};
use url::Url;

use std::borrow::Cow;

use http::Method;
use unm_request::json::{Json, UnableToExtractJson};
use unm_request::request;

pub const ENGINE_ID: &str = "bilibili";

/// The `bilibili` engine that can fetch audio from Bilibili Music.
pub struct BilibiliEngine;

#[async_trait]
impl Engine for BilibiliEngine {
    async fn search<'a>(
        &self,
        info: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation<'static>>> {
        info!("Searching with Bilibili engine…");

        let response = get_search_data(&info.keyword(), ctx).await?;
        let result = response
            .pointer("/data/result")
            .ok_or_else(|| anyhow::anyhow!("/data/result not found"))?
            .as_array()
            .ok_or(UnableToExtractJson {
                json_pointer: "/data/result",
                expected_type: "array",
            })?;

        let matched = find_match(info, result).await?;

        if let Some(song) = matched {
            Ok(Some(SongSearchInformation {
                source: Cow::Borrowed(ENGINE_ID),
                identifier: song.id.to_string(),
                song: Some(song),
            }))
        } else {
            Ok(None)
        }
    }

    async fn retrieve<'a>(
        &self,
        identifier: &'a SerializedIdentifier,
        ctx: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo<'static>> {
        info!("Retrieving the song by identifier…");

        let response = get_tracked_data(identifier.as_ref(), ctx).await?;
        let links = response
            .pointer("/data/cdns")
            .ok_or_else(|| anyhow::anyhow!("/data/cdns not found"))?
            .as_array()
            .ok_or(UnableToExtractJson {
                json_pointer: "/data/cdns",
                expected_type: "array",
            })?;

        if links.is_empty() {
            return Err(anyhow::anyhow!("unable to retrieve the identifier"));
        }

        let url = links[0]
            .as_str()
            .ok_or(UnableToExtractJson {
                json_pointer: "/data/cdns/0",
                expected_type: "string",
            })?
            .replace("https", "http");

        Ok(RetrievedSongInfo {
            source: Cow::Borrowed(ENGINE_ID),
            url,
        })
    }
}

/// Get search data from Bilibili Music.
async fn get_search_data(keyword: &str, ctx: &Context) -> anyhow::Result<Json> {
    debug!("Getting the search data from Bilibili Music…");

    let url = Url::parse_with_params(
        "https://api.bilibili.com/audio/music-service-c/s",
        &[
            ("search_type", "music"),
            ("page", "1"),
            ("pagesize", "30"),
            ("keyword", keyword),
        ]
    )?;

    let res = request(Method::GET, &url, None, None, ctx.try_get_proxy()?).await?;
    Ok(res.json().await?)
}

/// Track the ID from Bilibili Music.
async fn get_tracked_data(id: &str, ctx: &Context) -> anyhow::Result<Json> {
    debug!("Tracking the ID from Bilibili Music…");

    let url = Url::parse_with_params(
        "https://www.bilibili.com/audio/music-service-c/web/url",
        &[
            ("rivilege", "2"),
            ("quality", "2"),
            ("sid", id)
        ],
    )?;

    let res = request(Method::GET, &url, None, None, ctx.try_get_proxy()?).await?;
    Ok(res.json().await?)
}

/// Find the matched song.
///
/// `data` is the `data/result` of the Bilibili Music response.
async fn find_match(info: &Song, data: &[Json]) -> anyhow::Result<Option<Song>> {
    info!("Finding the matched song from the response…");

    let selector = SimilarSongSelector::new(info).optional_selector;
    let similar_song = data
        .iter()
        .map(|entry| format(entry).ok())
        .find(|s| selector(&s))
        .expect("should be Some");

    Ok(similar_song)
}

/// Format the Bilibili song metadata to [`Song`].
fn format(song: &Json) -> anyhow::Result<Song> {
    debug!("Formatting the response to Song…");

    let id = song["id"].as_i64().ok_or(UnableToExtractJson {
        json_pointer: "/id",
        expected_type: "i64",
    })?;
    let name = song["title"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "/title",
        expected_type: "string",
    })?;
    let mid = song["mid"].as_i64().ok_or(UnableToExtractJson {
        json_pointer: "/mid",
        expected_type: "i64",
    })?;
    let author = song["author"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "/author",
        expected_type: "string",
    })?;
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
        let info = get_info_1();
        let info = BilibiliEngine
            .search(&info, &Context::default())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(info.identifier, "349595");
        assert_eq!(info.source, ENGINE_ID);
    }

    #[test]
    async fn bilibili_retrieve() {
        let info = BilibiliEngine
            .retrieve(&String::from("349595"), &Context::default())
            .await
            .unwrap();

        assert_eq!(info.source, ENGINE_ID);
        println!("{}", info.url);
    }
}
