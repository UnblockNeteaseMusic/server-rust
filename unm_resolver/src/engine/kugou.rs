//! UNM Resolver [Engine]: Kugou
//!
//! It can fetch audio from Kugou Music.

use std::str::FromStr;

use async_trait::async_trait;
use http::Method;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::Url;
use urlencoding::encode;

use crate::{request::request, utils::UnableToExtractJson};

use super::{similar_song_selector_constructor, Album, Context, Engine, Song};
use crate::engine::Json;

/// The search and track engine powered by Kugou Music.
pub struct KugouEngine;

/// The context for determining the song to fetch from Kugou Music.
#[derive(Clone)]
pub struct KugouSongContext {
    /// The ID of HQ audio.
    pub id_hq: Option<String>,
    /// The ID of SQ audio.
    pub id_sq: Option<String>,
}

#[async_trait]
impl Engine for KugouEngine {
    async fn check<'a>(&self, info: &'a Song, ctx: &'a Context) -> anyhow::Result<Option<String>> {
        todo!()
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
pub async fn search(info: &Song, ctx: &Context<'_>) -> anyhow::Result<Song<KugouSongContext>> {
    let url_str = format!(
        "http://mobilecdn.kugou.com/api/v3/search/song?keyword={}&page=1&pagesize=10",
        encode(&info.keyword())
    );
    let url = Url::from_str(&url_str)?;

    let resp = request(Method::GET, &url, None, None, ctx.proxy.cloned()).await?;
    let data = resp.json::<Json>().await?;

    let lists = data
        .pointer("/data/lists")
        .and_then(|v| v.as_array())
        .ok_or(UnableToExtractJson("/data/lists", "string"))?;

    let selector = similar_song_selector_constructor(info).1;

    let similar_song = lists
        .par_iter()
        .map(format)
        .map(|v| v.ok())
        .find_first(|s| selector(&s))
        .expect("should be Some");

    similar_song.ok_or(anyhow::anyhow!("no such a song"))
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
