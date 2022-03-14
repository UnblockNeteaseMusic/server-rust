//! UNM Resolver [Engine]: Kugou
//!
//! It can fetch audio from Kugou Music.

use std::{borrow::Cow, str::FromStr, sync::Arc};

use async_trait::async_trait;
use futures::FutureExt;
use http::Method;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

use crate::{request::request, utils::UnableToExtractJson};

use super::{
    similar_song_selector_constructor, Album, Context, Engine, RetrievedSongInfo,
    SerializedIdentifier, Song, SongSearchInformation,
};
use crate::engine::Json;

const ENGINE_NAME: &str = "kugou";

/// The search and track engine powered by Kugou Music.
pub struct KugouEngine;

/// The context for determining the song to fetch from Kugou Music.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KugouSongContext {
    /// The ID of HQ audio.
    pub id_hq: Option<String>,
    /// The ID of SQ audio.
    pub id_sq: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum KugouFormat {
    /// The normal song quality.
    Hash,
    /// The High-Quality song quality.
    HqHash,
    /// The Super-Quality song quality.
    SqHash,
}

#[async_trait]
impl Engine for KugouEngine {
    async fn search<'a>(
        &self,
        info: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation<'static>>> {
        let response = search(info, ctx).await;

        match response {
            Ok(response) => match response {
                Some(response) => Ok(Some(SongSearchInformation {
                    source: Cow::Borrowed(ENGINE_NAME),
                    identifier: serde_json::to_string(&response)?,
                })),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    async fn retrieve<'a>(
        &self,
        identifier: &'a SerializedIdentifier,
        ctx: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo<'static>> {
        let song: Arc<Song<KugouSongContext>> = Arc::new(serde_json::from_str(identifier)?);

        let format_to_fetch = if ctx.enable_flac {
            [KugouFormat::HqHash, KugouFormat::SqHash]
        } else {
            [KugouFormat::Hash, KugouFormat::HqHash]
        };

        let song_futures = format_to_fetch.into_iter().map(|format| {
            let song = song.clone();

            async move {
                let response = single(&*song, format, ctx).await;
                match response {
                    Ok(response) => match response {
                        Some(response) => Ok(response),
                        None => Err(anyhow::anyhow!(
                            "unable to find the format {format:?} of song"
                        )),
                    },
                    Err(err) => Err(err),
                }
            }
            .boxed()
        });

        let url = futures::future::select_ok(song_futures).await?.0;

        Ok(RetrievedSongInfo {
            url,
            source: Cow::Borrowed(ENGINE_NAME),
        })
    }
}

fn format(entry: &Json) -> anyhow::Result<Song<KugouSongContext>> {
    let valstr = |key| {
        entry[key]
            .as_str()
            .ok_or(UnableToExtractJson(key, "string"))
            .map(|v| v.to_string())
    };

    Ok(Song {
        id: valstr("hash")?,
        name: valstr("songname")?,
        duration: entry["duration"].as_i64().map(|v| v * 1000),
        artists: vec![],
        album: Some(Album {
            id: valstr("album_id")?,
            name: valstr("album_name")?,
            songs: vec![],
        }),
        context: KugouSongContext {
            id_hq: entry["320hash"].as_str().map(|v| v.to_string()),
            id_sq: entry["sqhash"].as_str().map(|v| v.to_string()),
        },
    })
}

/// Search and get song (with metadata) from Kugou Music.
pub async fn search(
    info: &Song,
    ctx: &Context<'_>,
) -> anyhow::Result<Option<Song<KugouSongContext>>> {
    let url_str = format!(
        "http://mobilecdn.kugou.com/api/v3/search/song?keyword={}&page=1&pagesize=10",
        encode(&info.keyword())
    );
    let url = Url::from_str(&url_str)?;

    let resp = request(Method::GET, &url, None, None, ctx.try_get_proxy()?).await?;
    let data = resp.json::<Json>().await?;

    let lists = data
        .pointer("/data/info")
        .and_then(|v| v.as_array())
        .ok_or(UnableToExtractJson("/data/info", "string"))?;

    let selector = similar_song_selector_constructor(info).0;

    let similar_song = lists
        .par_iter()
        .map(format)
        .filter_map(|v| v.ok())
        .find_first(|s| selector(&s));

    Ok(similar_song)
}

pub async fn single(
    song: &Song<KugouSongContext>,
    format: KugouFormat,
    ctx: &Context<'_>,
) -> anyhow::Result<Option<String>> {
    let hash = extract_hash_id(song, format)?;
    let key = format!("{hash}kgcloudv2");

    let album_id = song
        .album
        .as_ref()
        .map(|v| v.id.to_string())
        .unwrap_or_else(|| String::from(""));

    let url_str = format!("http://trackercdn.kugou.com/i/v2/?key={key}&hash={hash}&appid=1005&pid=2&cmd=25&behavior=play&album_id={album_id}");
    let url = Url::from_str(&url_str)?;

    let response = request(Method::GET, &url, None, None, ctx.try_get_proxy()?).await?;
    let data = response.json::<Json>().await?;

    Ok(data
        .pointer("/url/0")
        .and_then(|v| v.as_str())
        .map(|v| v.to_string()))
}

pub fn extract_hash_id(
    song: &Song<KugouSongContext>,
    format: KugouFormat,
) -> anyhow::Result<String> {
    let id = match format {
        KugouFormat::Hash => song.context.id_hq.as_ref(),
        KugouFormat::HqHash => song.context.id_hq.as_ref(),
        KugouFormat::SqHash => song.context.id_sq.as_ref(),
    };

    match id {
        Some(id) => Ok(id.to_string()),
        None => Err(anyhow::anyhow!("No such a format.")),
    }
}

/*
const insure = require('./insure');
const select = require('./select');
const crypto = require('../crypto');
const request = require('../request');
const { getManagedCacheStorage } = require('../cache');

const format = (song) => {
    return {
        // id: song.FileHash,
        // name: song.SongName,
        // duration: song.Duration * 1000,
        // album: {id: song.AlbumID, name: song.AlbumName},
        // artists: song.SingerId.map((id, index) => ({id, name: SingerName[index]}))
        id: song['hash'],
        id_hq: song['320hash'],
        id_sq: song['sqhash'],
        name: song['songname'],
        duration: song['duration'] * 1000,
        album: { id: song['album_id'], name: song['album_name'] },
    };
};

const search = (info) => {
    const url =
        // 'http://songsearch.kugou.com/song_search_v2?' +
        'http://mobilecdn.kugou.com/api/v3/search/song?' +
        'keyword=' +
        encodeURIComponent(info.keyword) +
        '&page=1&pagesize=10';

    return request('GET', url)
        .then((response) => response.json())
        .then((jsonBody) => {
            // const list = jsonBody.data.lists.map(format)
            const list = jsonBody.data.info.map(format);
            const matched = select(list, info);
            return matched ? matched : Promise.reject();
        })
        .catch(() => insure().kugou.search(info));
};

const single = (song, format) => {
    const getHashId = () => {
        switch (format) {
            case 'hash':
                return song.id;
            case 'hqhash':
                return song.id_hq;
            case 'sqhash':
                return song.id_sq;
            default:
                break;
        }
        return '';
    };

    const url =
        'http://trackercdn.kugou.com/i/v2/?' +
        'key=' +
        crypto.md5.digest(`${getHashId()}kgcloudv2`) +
        '&hash=' +
        getHashId() +
        '&' +
        'appid=1005&pid=2&cmd=25&behavior=play&album_id=' +
        song.album.id;
    return request('GET', url)
        .then((response) => response.json())
        .then((jsonBody) => jsonBody.url[0] || Promise.reject());
};

const track = (song) =>
    Promise.all(
        ['sqhash', 'hqhash', 'hash']
            .slice(select.ENABLE_FLAC ? 0 : 1)
            .map((format) => single(song, format).catch(() => null))
    )
        .then((result) => result.find((url) => url) || Promise.reject())
        .catch(() => insure().kugou.track(song));

const cs = getManagedCacheStorage('provider/kugou');
const check = (info) => cs.cache(info, () => search(info)).then(track);

module.exports = { check, search };
*/
