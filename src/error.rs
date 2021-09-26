pub use reqwest::Error as ReqErr;
pub use serde_json::Error as JsonErr;
use thiserror::Error as BaseErr;

#[derive(BaseErr, Debug)]
pub enum Error {
    #[error("request fail {0}")]
    RequestFail(ReqErr),
    #[error("request headers is invalid")]
    HeadersDataInvalid,
    #[error("json parse fail {0}")]
    JsonParseFail(JsonErr),
    #[error("Failed to XOR this ID char (u32) {} with this key char (u32) {}")]
    UriEncryptXorFail(u32, u32),
    #[error("unknown data store error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
