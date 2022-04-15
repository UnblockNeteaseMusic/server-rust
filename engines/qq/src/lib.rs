use std::{borrow::Cow, collections::HashMap};

use async_trait::async_trait;
use http::{
    header::{COOKIE, ORIGIN, REFERER},
    HeaderMap, HeaderValue, Method,
};
use log::debug;
use reqwest::Url;
use unm_engine::interface::Engine;
use unm_request::{
    json::{Json, UnableToExtractJson},
    request,
};
use unm_selector::SimilarSongSelector;
use unm_types::{
    Album, Context, RetrievedSongInfo, SerializedIdentifier, Song, SongSearchInformation,
};

pub const ENGINE_ID: &str = "qq";

pub struct QQEngine;

#[async_trait]
impl Engine for QQEngine {
    async fn search<'a>(
        &self,
        info: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation>> {
        log::info!("Searching with QQ engine");

        let response = get_search_data(&info.keyword(), ctx).await?;
        let result = response
            .pointer("/data/song/list")
            .ok_or_else(|| anyhow::anyhow!("/data/song/list not found"))?
            .as_array()
            .ok_or(UnableToExtractJson {
                json_pointer: "/data/song/list",
                expected_type: "array",
            })?;

        let matched = find_match(info, result).await?;

        if let Some(song) = matched {
            Ok(Some(
                SongSearchInformation::builder()
                    .source(ENGINE_ID.into())
                    .identifier(song.id.to_string())
                    .song(Some(song))
                    .build(),
            ))
        } else {
            Ok(None)
        }
    }

    async fn retrieve<'a>(
        &self,
        _identifier: &'a SerializedIdentifier,
        _ctx: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo> {
        todo!()
    }
}

async fn get_search_data(keyword: &str, ctx: &Context) -> anyhow::Result<Json> {
    debug!("Getting the search data from QQ Music…");

    let url = construct_search_url(keyword)?;
    let cookie = get_cookie(ctx);
    let res = request(
        Method::GET,
        &url,
        Some(construct_header(cookie)?),
        None,
        ctx.try_get_proxy()?,
    )
    .await?;
    Ok(res.json().await?)
}

fn get_cookie(context: &Context) -> Option<&str> {
    if let Some(ref config) = context.config {
        config.get_deref(Cow::Borrowed("qq:cookie"))
    } else {
        None
    }
}

async fn find_match(info: &Song, data: &[Json]) -> anyhow::Result<Option<Song>> {
    let selector = SimilarSongSelector::new(info).optional_selector;
    let similar_song = data
        .iter()
        .map(|entry| format(entry).ok())
        .find(|s| selector(&s))
        .expect("shoule be Some");

    Ok(similar_song)
}

fn format(song: &Json) -> anyhow::Result<Song> {
    debug!("Formatting the response to Song…");

    let id = song["songid"].as_i64().ok_or(UnableToExtractJson {
        json_pointer: "/songid",
        expected_type: "i64",
    })?;
    let name = song["songname"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "/songname",
        expected_type: "string",
    })?;
    let duration = song["interval"].as_i64().ok_or(UnableToExtractJson {
        json_pointer: "/interval",
        expected_type: "i64",
    })?;
    let album_id = song["albumid"].as_i64().ok_or(UnableToExtractJson {
        json_pointer: "/albumid",
        expected_type: "i64",
    })?;
    let album_name = song["albumname"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "/albumname",
        expected_type: "string",
    })?;

    let media_mid = song["media_mid"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "/media_mid",
        expected_type: "string",
    })?;
    let song_mid = song["songmid"].as_str().ok_or(UnableToExtractJson {
        json_pointer: "/songmid",
        expected_type: "string",
    })?;
    let context = {
        let mut context = HashMap::new();
        context.insert("media_mid".to_string(), media_mid.to_string());
        context.insert("songmid".to_string(), song_mid.to_string());

        context
    };

    Ok(Song::builder()
        .id(id.to_string())
        .name(name.to_string())
        .duration(Some(duration * 1000))
        .album(Some(
            Album::builder()
                .id(album_id.to_string())
                .name(album_name.to_string())
                .build(),
        ))
        .context(Some(context))
        .build())
}

fn construct_header(cookie: Option<&str>) -> anyhow::Result<HeaderMap> {
    log::debug!("Constructing the header for QQ Music…");

    let mut hm = HeaderMap::new();

    hm.insert(ORIGIN, HeaderValue::from_static("http://y.qq.com"));
    hm.insert(REFERER, HeaderValue::from_static("http://y.qq.com"));

    if let Some(cookie) = cookie {
        hm.insert(COOKIE, HeaderValue::from_str(cookie)?);
    }

    Ok(hm)
}

fn construct_search_url(keyword: &str) -> anyhow::Result<Url> {
    Ok(Url::parse_with_params(
        "https://c.y.qq.com/soso/fcgi-bin/client_search_cp?",
        &[
            ("ct", "24"),
            ("qqmusic_ver", "1298"),
            ("remoteplace", "txt.yqq.center"),
            ("t", "0"),
            ("aggr", "1"),
            ("cr", "1"),
            ("catZhida", "1"),
            ("lossless", "1"),
            ("flag_qc", "0"),
            ("p", "1"),
            ("n", "20"),
            ("w", keyword),
            ("g_tk", "5381"),
            ("loginUin", "0"),
            ("hostUin", "0"),
            ("format", "json"),
            ("inCharset", "utf8"),
            ("outCharset", "utf-8"),
            ("notice", "0"),
            ("platform", "yqq"),
            ("needNewCode", "0"),
        ],
    )?)
}

#[cfg(test)]
mod tests {
    use tokio::test;
    use unm_types::{Artist, Context};

    use super::*;

    fn get_info_1() -> Song {
        // https://music.163.com/api/song/detail?ids=[385552]
        Song::builder()
            .name("干杯".to_string())
            .artists(vec![Artist::builder().name("五月天".to_string()).build()])
            .build()
    }

    #[test]
    async fn qq_search() {
        let info = get_info_1();
        let info = QQEngine
            .search(&info, &Context::default())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(info.identifier, "1056382");
        assert_eq!(info.source, ENGINE_ID);
    }
}
