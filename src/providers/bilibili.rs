use super::definitions::*;
use crate::error::*;
use urlencoding::encode;
use crate::request::*;

pub struct BilibiliResult {}
pub struct BilibiliProvider {

}

impl BilibiliProvider {
	async fn search(&self, info: &SongMetadata) -> Result<()> {
		let url_str = format!(
			"https://api.bilibili.com/audio/music-service-c/s?\
			search_type=music&page=1&pagesize=30&\
			keyword=${0}", encode(info.keyword().as_str()));
		let url = Url::parse(url_str.as_str()).map_err(|e| {Error::UrlParseFail(e)})?;
		let res = request(Method::GET, url, None, None, None).await?;
		// res.json().map(); // unimplement
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
