use http::header::ToStrError;
pub use log4rs::config::runtime::ConfigErrors as LogConfErr;
pub use reqwest::Error as ReqErr;
pub use serde_json::Error as SerdeJsonErr;
use thiserror::Error as BaseErr;
pub use url::ParseError as UrlErr;

pub use crate::crypto::CryptoError;

#[derive(BaseErr, Debug)]
pub enum JsonErr {
    #[error("{0}")]
    SerdeJsonError(#[from] SerdeJsonErr),
    #[error("`{0}` not found or is not {1} type")]
    ParseError(&'static str, &'static str),
}

#[derive(BaseErr, Debug)]
pub enum Error {
    #[error("Failed to request: {0}")]
    RequestFail(#[from] ReqErr),
    #[error("The request headers are invalid.")]
    HeadersDataInvalid,
    #[error("Failed to parse JSON: {0}")]
    JsonParseFail(#[from] JsonErr),
    #[error("Failed to parse URL: {0}")]
    UrlParseFail(UrlErr),
    #[error("Failed to configure log: {0}")]
    LogConfigFailed(#[from] LogConfErr),
    #[error("Failed to setup log: {0}")]
    LogSetupFailed(String),
    #[error("Failed to crypto: {0}")]
    CryptoFailed(CryptoError),
    #[error("Failed to convert a string: {0}")]
    StringConvertFailed(#[from] ToStrError),
    #[error("{0}")]
    CustomError(String),
    #[error("Error storing unknown data.")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
