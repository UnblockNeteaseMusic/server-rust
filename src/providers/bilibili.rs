use super::definitions::*;
use crate::error::*;
use crate::request::*;
use urlencoding::encode;

pub struct BilibiliResult {}
pub struct BilibiliProvider {}

impl BilibiliProvider {
    async fn search(&self, info: &SongMetadata) -> Result<()> {
        let url_str = format!(
            "https://api.bilibili.com/audio/music-service-c/s?\
			search_type=music&page=1&pagesize=30&\
			keyword=${0}",
            encode(info.keyword().as_str())
        );
        let url = Url::parse(url_str.as_str()).map_err(|e| Error::UrlParseFail(e))?;
        let res = request(Method::GET, url, None, None, None).await?;
        // res.json().map(); // unimplement
        let res_json = res
            .json::<Json>()
            .await
            .map_err(|e| Error::RequestFail(e))?;
        println!("{}", res_json);
        return Ok(());
    }
}

#[async_trait]
impl Provide for BilibiliProvider {
    type SearchResultType = Json;

    async fn check(info: &SongMetadata) -> Result<()> {
        todo!()
    }

    async fn track(search_result: Self::SearchResultType) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::{runtime, sync::oneshot, test};

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
    async fn bilibili_search() {
        let p = BilibiliProvider {};
        let info = get_info_1();
        p.search(&info).await.unwrap();
    }
}
