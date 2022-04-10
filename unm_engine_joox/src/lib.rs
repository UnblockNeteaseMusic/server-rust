//! UNM Engine: Joox
//!
//! You may need to register a Joox account,
//! obtain its cookie and fill the cookie to
//! the `joox:cookie` config.
//!
//! You can configure the cookie in the
//! `ctx.config` HashMap. For example:
//!
//! ```
//! let config = {
//!     let mut hm = HashMap::<String, String>::new();
//!     hm.insert(
//!         "joox:cookie".to_string(),
//!         r#"wmid=<your_wmid>; session_key=<your_session_key>;"#.to_string()
//!     );
//!     hm
//! };
//! ```

use std::{borrow::Cow, str::FromStr};

use http::{
    header::{COOKIE, ORIGIN, REFERER},
    HeaderValue, Method,
};
use once_cell::sync::OnceCell;
use regex::Regex;
use reqwest::{header::HeaderMap, Url};
use unm_engine::interface::Engine;
use unm_request::{
    extract_jsonp,
    json::{Json, UnableToExtractJson},
    request,
};
use unm_selector::SimilarSongSelector;
use unm_types::{
    Album, Artist, Context, RetrievedSongInfo, SerializedIdentifier, Song, SongSearchInformation,
};

static REPLACE_AUDIO_URL_REGEX: OnceCell<Regex> = OnceCell::new();

pub const ENGINE_ID: &str = "joox";
pub struct JooxEngine;

#[async_trait::async_trait]
impl Engine for JooxEngine {
    async fn search<'a>(
        &self,
        song: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation<'static>>> {
        log::debug!("Searching “{song}” with Joox Engine…");

        let keyword = fit(song);
        let joox_cookie = ctx
            .config
            .as_ref()
            .and_then(|hm| hm.get("joox:cookie").cloned());

        let url_str = format!(
            concat!(
                "http://api-jooxtt.sanook.com/web-fcgi-bin/web_search?",
                "country=hk&lang=zh_TW&sin=0&ein=30&",
                "search_input={}",
            ),
            keyword
        );
        let url = Url::from_str(&url_str)?;

        let response = request(
            Method::GET,
            &url,
            Some(construct_header(joox_cookie.as_deref())?),
            None,
            ctx.try_get_proxy()?,
        )
        .await?;

        log::debug!("Deserializing the response of “{song}”…");
        let json_string = response.text().await?.replace('\'', "\"");
        let json = serde_json::from_str::<Json>(&json_string)?;

        log::debug!("Converting the Joox response to Vec<Song>…");
        let empty = Vec::new();
        let mut song_iterator = json["itemlist"]
            .as_array()
            .unwrap_or(&empty)
            .iter()
            .map(format)
            .filter_map(|v| match v {
                Ok(v) => Some(v),
                Err(e) => {
                    log::warn!("Failed to parse an item: {e}. Ignoring.");
                    None
                }
            });

        log::debug!("Selecting the similar song…");
        let SimilarSongSelector { selector, .. } = SimilarSongSelector::new(song);
        let matched = song_iterator.find(|s| selector(&s));

        Ok(matched.map(|matched| SongSearchInformation {
            source: Cow::Borrowed(ENGINE_ID),
            identifier: matched.id.clone(),
            song: Some(matched),
        }))
    }

    /// Retrieve the audio URL of the specified `identifier`.
    async fn retrieve<'a>(
        &self,
        identifier: &'a SerializedIdentifier,
        ctx: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo<'static>> {
        log::debug!("Retrieving with Joox Engine…");

        let replace_audio_url_regex = REPLACE_AUDIO_URL_REGEX
            .get_or_init(|| Regex::new(r#"M\d00([\w]+).mp3"#).expect("should be constructable"));
        let joox_cookie = ctx
            .config
            .as_ref()
            .and_then(|hm| hm.get("joox:cookie").cloned());

        let url_str = format!(
            concat!(
                "http://api.joox.com/web-fcgi-bin/web_get_songinfo?",
                "country=hk&lang=zh_cn&from_type=-1&channel_id=-1&",
                "song_id={id}&_={timestamp}",
            ),
            id = identifier,
            timestamp = get_timestamp(),
        );
        let url = Url::from_str(&url_str)?;

        let response = request(
            Method::GET,
            &url,
            Some(construct_header(joox_cookie.as_deref())?),
            None,
            ctx.try_get_proxy()?,
        )
        .await?;
        let jsonp_string = response.text().await?;
        let json_string = extract_jsonp(jsonp_string.as_str());
        let json = serde_json::from_str::<Json>(&json_string)?;

        let raw_audio_url = ["r320Url", "r192Url", "mp3Url", "m4aUrl"]
            .into_iter()
            .filter_map(|k| json[k].as_str())
            .next();
        let audio_url = raw_audio_url.map(|u| replace_audio_url_regex.replace(u, "M800$1.mp3"));

        if let Some(url) = audio_url {
            Ok(RetrievedSongInfo {
                source: Cow::Borrowed(ENGINE_ID),
                url: url.to_string(),
            })
        } else {
            Err(anyhow::anyhow!("No audio URL found."))
        }
    }
}

fn construct_header(cookie: Option<&str>) -> anyhow::Result<HeaderMap> {
    log::debug!("Constructing the header for Joox…");

    let mut hm = HeaderMap::new();

    hm.insert(ORIGIN, HeaderValue::from_static("http://www.joox.com"));
    hm.insert(REFERER, HeaderValue::from_static("http://www.joox.com"));

    if let Some(cookie) = cookie {
        hm.insert(COOKIE, HeaderValue::from_str(cookie)?);
    }

    Ok(hm)
}

fn fit(song: &Song) -> String {
    log::debug!("Fitting the keyword {song}…");
    let is_japanese_character = |c| matches!(c, '\u{0800}'..='\u{4e00}');

    if song.name.chars().any(is_japanese_character) {
        log::debug!("Has Japanese characters, return only the name…");
        song.name.clone()
    } else {
        log::debug!("No Japanese characters, return keyword…");
        song.keyword()
    }
}

fn format(item: &Json) -> anyhow::Result<Song> {
    log::debug!("Formatting a Joox song item…");

    log::debug!("{item:#?}");

    let valstr = |data: &Json, json_pointer| {
        data.as_str()
            .map(ToString::to_string)
            .ok_or(UnableToExtractJson {
                json_pointer,
                expected_type: "str",
            })
    };

    let vali64 = |data: &Json, json_pointer| {
        data.as_i64().ok_or(UnableToExtractJson {
            json_pointer,
            expected_type: "i64",
        })
    };

    let b64_opt_decode = |data: Option<&str>| -> anyhow::Result<String> {
        if let Some(data) = data {
            let bytes = unm_crypto::base64::decode_block(data)?;
            Ok(String::from_utf8(bytes)?)
        } else {
            Ok("".to_string())
        }
    };

    let artists = item["singer_list"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|singer| -> anyhow::Result<Artist> {
            Ok(Artist {
                id: valstr(&singer["id"], "/singer_list/?/id")?,
                name: b64_opt_decode(singer["name"].as_str())?,
            })
        })
        .filter_map(|v| match v {
            Ok(v) => Some(v),
            Err(e) => {
                log::warn!("Failed to parse the artist: {e}. Ignoring.");
                None
            }
        })
        .collect::<Vec<Artist>>();

    Ok(Song {
        id: valstr(&item["songid"], "/songid")?,
        name: b64_opt_decode(item["info1"].as_str())?,
        duration: Some(vali64(&item["playtime"], "/playtime")? * 1000),
        album: Some(Album {
            id: valstr(&item["/albummid"], "/albummid")?,
            name: b64_opt_decode(item["info3"].as_str())?,
        }),
        artists,
        context: None,
    })
}

fn get_timestamp() -> u128 {
    use std::time::SystemTime;

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|v| v.as_millis())
        .unwrap_or(0)
}
