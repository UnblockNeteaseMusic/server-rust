pub use reqwest::Error as ReqErr;
pub use serde_json::Error as JsonErr;
use thiserror::Error as BaseErr;

#[derive(BaseErr, Debug)]
pub enum Error {
    #[error("Failed to request: {0}")]
    RequestFail(ReqErr),
    #[error("The request headers are invalid.")]
    HeadersDataInvalid,
    #[error("Failed to parse JSON: {0}")]
    JsonParseFail(JsonErr),
    #[error("Error storing unknown data.")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
