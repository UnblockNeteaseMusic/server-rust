pub use reqwest::Error as ReqErr;
use thiserror::Error as BaseErr;
pub use url::ParseError as UrlErr;

pub use crate::error::JsonErr;

#[derive(BaseErr, Debug)]
pub enum RequestError {
    #[error("Failed to request: {0}")]
    RequestFail(#[from] ReqErr),
    #[error("The request headers are invalid.")]
    HeadersDataInvalid,
    #[error("Failed to parse URL: {0}")]
    UrlParseFail(UrlErr),
}
