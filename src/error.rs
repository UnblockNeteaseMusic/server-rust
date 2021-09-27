pub use crate::crypto::CryptoError;
pub use log4rs::config::runtime::ConfigErrors as LogConfErr;
pub use reqwest::Error as ReqErr;
pub use serde_json::Error as JsonErr;
use thiserror::Error as BaseErr;

#[derive(BaseErr, Debug)]
pub enum Error {
    #[error("Failed to request: {0}")]
    RequestFail(#[from] ReqErr),
    #[error("The request headers are invalid.")]
    HeadersDataInvalid,
    #[error("Failed to parse JSON: {0}")]
    JsonParseFail(#[from] JsonErr),
    #[error("Failed to configure log: {0}")]
    LogConfigFailed(#[from] LogConfErr),
    #[error("Failed to setup log: {0}")]
    LogSetupFailed(String),
    #[error("Failed to crypto: {0}")]
    CryptoFailed(CryptoError),
    #[error("Error storing unknown data.")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
