pub use reqwest::Error as ReqErr;
pub use serde_json::Error as JsonErr;
use thiserror::Error as BaseErr;
pub use url::ParseError as UrlErr;

#[derive(BaseErr, Debug)]
pub enum Error {
    #[error("Failed to request: {0}")]
    RequestFail(ReqErr),
    #[error("The request headers are invalid.")]
    HeadersDataInvalid,
    #[error("Failed to parse JSON: {0}")]
    JsonParseFail(JsonErr),
    #[error("Failed to parse URL: {0}")]
    UrlParseFail(UrlErr),
    #[error("Failed to XOR this ID char (u32) {0} with this key char (u32) {1}")]
    UriEncryptXorFail(u32, u32),
    #[error("Error storing unknown data.")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
