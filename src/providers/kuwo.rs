use regex::Regex;
use urlencoding::encode;

use crate::error::*;
use crate::request::*;

use super::definitions::*;

pub struct KuwoProvider {}

impl KuwoProvider {
    /// find music id in kuwo
    async fn search(&self, info: &SongMetadata) -> Result<Option<i64>> {
        let keyword_origin = info.keyword().replace(" - ", "");
        let keyword = encode(keyword_origin.as_str());
        let url = format!(
            "http://www.kuwo.cn/api/www/search/searchMusicBykeyWord?key={}&pn=1&rn=30",
            keyword,
        );
        // println!("{}", url);
        let res = request_str(
            Method::GET,
            format!("http://kuwo.cn/search/list?key={}", keyword).as_str(),
            None,
            None,
            None,
        )
        .await?;

        let re = Regex::new(r"kw_token=(\S+);").expect("wrong regex of token");
        let mut token: Option<String> = None;
        for val in res.headers().get_all("set-cookie").iter() {
            let token_str = val.to_str().map_err(|_| Error::HeadersDataInvalid)?;
            match re.captures(token_str) {
                None => {}
                Some(cap) => {
                    token = Some(
                        cap.get(1)
                            .ok_or(Error::HeadersDataInvalid)?
                            .as_str()
                            .to_string(),
                    );
                    break;
                }
            };
        }
        let token = token.ok_or(Error::CustomError(String::from("kuwo token not found.")))?;

        let res = request_str(
            Method::GET,
            url.as_str(),
            Some(json!({
                "referer": format!("http://www.kuwo.cn/search/list?key={}", keyword),
                "csrf": token,
                "cookie": format!("kw_token={}", token),
            })),
            None,
            None,
        )
        .await?;
        let jsonbody = res.json::<Json>().await.map_err(Error::RequestFail)?;
        match jsonbody["code"].as_i64() {
            Some(code) => {
                if code != 200 {
                    return Err(Error::CustomError(String::from(
                        "response code is not 200.",
                    )));
                }
            }
            _ => {}
        }
        let mut list: Vec<SongMetadata> = Vec::new();
        for item in jsonbody["data"]["list"]
            .as_array()
            .ok_or(JsonErr::ParseError("data.list", "array"))?
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

fn format(song: &Json) -> Result<SongMetadata> {
    let id_sp = &song["musicrid"]
        .as_str()
        .ok_or(JsonErr::ParseError("musicrid", "string"))?
        .split('_')
        .collect::<Vec<&str>>();
    let id_str = id_sp
        .last()
        .ok_or(Error::CustomError(String::from("musicrid is invalid")))?;
    let id = id_str
        .parse::<i64>()
        .map_err(|_| Error::CustomError(String::from("musicrid is invalid")))?;

    let name = song["name"]
        .as_str()
        .ok_or(JsonErr::ParseError("name", "string"))?;
    let duration = &song["duration"]
        .as_i64()
        .ok_or(JsonErr::ParseError("duration", "i64"))?;
    let albumid = &song["albumid"]
        .as_str()
        .ok_or(JsonErr::ParseError("albumid", "string"))?
        .parse::<i64>()
        .map_err(|_| Error::CustomError(String::from("albumid is not string of num.")))?;
    let album = &song["album"]
        .as_str()
        .ok_or(JsonErr::ParseError("album", "string"))?;
    let artists_str = &song["artist"]
        .as_str()
        .ok_or(JsonErr::ParseError("artist", "string"))?
        .split('&')
        .collect::<Vec<&str>>();

    let mut artists: Vec<SongArtistMetadata> = Vec::new();
    for (i, v) in artists_str.iter().enumerate() {
        let mut data = SongArtistMetadata {
            id: -1, // TODO: reconstruct as None(i64)?
            name: String::from(*v),
        };
        if i == 0 {
            let artist_id = &song["artistid"]
                .as_i64()
                .ok_or(JsonErr::ParseError("artistid", "i64"))?;
            data.id = *artist_id;
        }
        artists.push(data);
    }

    let x = SongMetadata {
        id: id,
        name: String::from(name),
        duration: Some(*duration * 1000),
        album: Some(SongAlbumMetadata {
            id: *albumid,
            name: String::from(*album),
        }),
        artists: artists,
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
    async fn kuwo_search() {
        let p = KuwoProvider {};
        let info = get_info_1();
        let id = p.search(&info).await.unwrap();
        println!("{:#?}", id);
        assert_eq!(id, Some(3654739));
    }
}
