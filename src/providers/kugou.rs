use std::convert::TryFrom;
use std::convert::TryInto;

use super::definitions::*;
use crate::crypto;
use crate::error::*;
use crate::request::*;
use urlencoding::encode;

#[derive(Debug, Clone)]
pub struct KugouSongMetadata {
    pub id: String,
    pub id_hq: Option<String>,
    pub id_sq: Option<String>,
    pub name: String,
    pub duration: i64,
    pub album: Option<SongAlbumMetadata>,
}

impl KugouSongMetadata {
    pub fn to_general(&self, id: i64) -> SongMetadata {
        SongMetadata {
            id,
            name: self.name.to_string(),
            duration: Some(self.duration),
            album: self.album.clone(),
            artists: vec![],
        }
    }
}

fn format(song: &Json) -> Result<KugouSongMetadata> {
    let id = &song["hash"]
        .as_str()
        .ok_or(JsonErr::ParseError("hash", "string"))?;
    let id_hq = &song["320hash"]
        .as_str()
        .ok_or(JsonErr::ParseError("320hash", "string"))?;
    let id_sq = &song["sqhash"]
        .as_str()
        .ok_or(JsonErr::ParseError("sqhash", "string"))?;
    let name = song["songname"]
        .as_str()
        .ok_or(JsonErr::ParseError("songname", "string"))?;
    let duration = &song["duration"]
        .as_i64()
        .ok_or(JsonErr::ParseError("duration", "i64"))?;
    let aid_str = &song["album_id"]
        .as_str()
        .ok_or(JsonErr::ParseError("album_id", "string"))?;
    let aname = song["album_name"]
        .as_str()
        .ok_or(JsonErr::ParseError("album_name", "string"))?;
    let album = if aid_str.is_empty() {
        None
    } else {
        Some(SongAlbumMetadata {
            name: aname.to_string(),
            id: aid_str
                .parse::<i64>()
                .map_err(|_| Error::CustomError(String::from("album_id is not string of num.")))?,
        })
    };
    let x = KugouSongMetadata {
        id: id.to_string(),
        id_hq: if id_hq.is_empty() {
            None
        } else {
            Some(id_hq.to_string())
        },
        id_sq: if id_sq.is_empty() {
            None
        } else {
            Some(id_sq.to_string())
        },
        name: name.to_string(),
        duration: *duration * 1000,
        album,
    };
    Ok(x)
}

pub struct KugouProvider {
    enable_flac: bool,
}

impl KugouProvider {
    /// generate provider for kugou source
    pub fn new(enable_flac: &Option<bool>) -> KugouProvider {
        KugouProvider {
            enable_flac: match enable_flac {
                None => false,
                Some(v) => *v,
            },
        }
    }
    /// find music id in kugou
    async fn search(&self, info: &SongMetadata) -> Result<Option<KugouSongMetadata>> {
        let url_str = format!(
            "http://mobilecdn.kugou.com/api/v3/search/song?keyword={}&page=1&pagesize=10",
            encode(info.keyword().as_str())
        );

        let res = request_str(Method::GET, url_str.as_str(), None, None, None).await?;
        let jsonbody = res.json::<Json>().await.map_err(Error::RequestFail)?;
        let mut list: Vec<KugouSongMetadata> = Vec::new();
        for item in jsonbody["data"]["info"]
            .as_array()
            .ok_or(JsonErr::ParseError("data.info", "array"))?
            .iter()
        {
            list.push(format(item)?);
        }
        // convert to General SongMetadata
        let mut glist: Vec<SongMetadata> = Vec::new();
        for item in list.iter().enumerate() {
            // id is index
            glist.push(item.1.to_general(item.0.try_into().unwrap()));
        }
        let matched = select_similar_song(&glist, info);
        match matched {
            None => Ok(None),
            Some(song) => {
                let idx = usize::try_from(song.id).unwrap();
                Ok(Some(list[idx].clone()))
            }
        }
    }

    async fn track_single(data: &KugouSongMetadata, id: Option<String>) -> Result<Option<String>> {
        match id {
            None => Ok(None),
            Some(id) => {
                let key = crypto::md5::digest(format!("{}kgcloudv2", id));
                let mut url =
                    format!("http://trackercdn.kugou.com/i/v2/?key={}&hash={}&appid=1005&pid=2&cmd=25&behavior=play", key, id);
                match &data.album {
                    None => {}
                    Some(album) => {
                        url.push_str(format!("&album_id={}", album.id).as_str());
                    }
                }
                println!("{}", url);
                let res = request_str(Method::GET, url.as_str(), None, None, None).await?;
                let jsonbody = res.json::<Json>().await.map_err(Error::RequestFail)?;
                println!("{}", jsonbody);
                for item in jsonbody["url"]
                    .as_array()
                    .ok_or(JsonErr::ParseError("url", "array"))?
                    .iter()
                {
                    let music_link = item
                        .as_str()
                        .ok_or(JsonErr::ParseError("url[0]", "string"))?;
                    return Ok(Some(music_link.to_string()));
                }
                Ok(None)
            }
        }
    }
    /// trace kugou's music and find out music link
    async fn track(&self, data: &KugouSongMetadata) -> Result<Option<String>> {
        let mut collector: Vec<Option<String>> = vec![None, None, None];

        if self.enable_flac {
            let res = tokio::join!(
                KugouProvider::track_single(data, data.id_sq.clone()),
                KugouProvider::track_single(data, data.id_hq.clone()),
                KugouProvider::track_single(data, Some(data.id.to_string()))
            );
            collector[0] = res.0?;
            collector[1] = res.1?;
            collector[2] = res.2?;
        } else {
            let res = tokio::join!(
                KugouProvider::track_single(data, data.id_hq.clone()),
                KugouProvider::track_single(data, Some(data.id.to_string()))
            );
            collector[0] = None;
            collector[1] = res.0?;
            collector[2] = res.1?;
        }

        let val = collector.iter().find(|v| {
            return v.is_some();
        });
        match val {
            None => Ok(None),
            Some(v) => Ok(v.clone()),
        }
    }
}

#[async_trait]
impl Provide for KugouProvider {
    async fn check(&self, info: &SongMetadata) -> Result<Option<String>> {
        match self.search(info).await? {
            None => Ok(None),
            Some(data) => Ok(self.track(&data).await?),
        }
    }
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
    async fn kugou_search() {
        let p = KugouProvider::new(&None);
        let info = get_info_1();
        let data = p.search(&info).await.unwrap();
        println!("{:#?}", data);
        assert!(data.is_some());
        let meta = data.unwrap();
        assert_eq!(
            meta.id.clone(),
            "daa9732c81195df32bd46d92e0dfc09e".to_string()
        );

        assert!(meta.id_hq.is_some());
        assert_eq!(
            meta.id_hq.unwrap().clone(),
            "ccc679734019d9b26207e1c0b24230ba".to_string()
        );

        assert!(meta.id_sq.is_some());
        assert_eq!(
            meta.id_sq.unwrap().clone(),
            "bc31f2cdf882ec44511d33825b2a5339".to_string()
        );
    }

    fn gen_kugou_song() -> KugouSongMetadata {
        KugouSongMetadata {
            id: "daa9732c81195df32bd46d92e0dfc09e".to_string(),
            id_hq: Some("ccc679734019d9b26207e1c0b24230ba".to_string()),
            id_sq: Some("bc31f2cdf882ec44511d33825b2a5339".to_string()),
            duration: 289000,
            name: "干杯".to_string(),
            album: Some(SongAlbumMetadata {
                id: 520409,
                name: "第二人生（明日版）".to_string(),
            }),
        }
    }

    #[test]
    async fn kugou_track_single() {
        let d = gen_kugou_song();
        let url = KugouProvider::track_single(&d, Some(d.id.clone()))
            .await
            .unwrap()
            .unwrap();
        println!("{}", url);
    }

    //#[test]
    //async fn kugou_track() {
    //    let p = KugouProvider::new(&None);
    //    let d = gen_kugou_song();
    //    let url = p.track(&d).await.unwrap().unwrap();
    //    println!("{}", url);
    //}

    //#[test]
    //async fn kugou_check() {
    //    let p = KugouProvider::new(&None);
    //    let info = get_info_1();
    //    let url = p.check(&info).await.unwrap().unwrap();
    //    println!("{}", url);
    //}
}
