use http::header::ToStrError;
pub use http::Error as HttpErr;
pub use log4rs::config::runtime::ConfigErrors as LogConfErr;
pub use serde_json::Error as SerdeJsonErr;
use thiserror::Error as BaseErr;
pub use url::ParseError as UrlErr;

pub use crate::crypto::CryptoError as CryptoErr;
use crate::request::error::RequestError;
use crate::server::error::ServerError as ServerErr;

#[derive(BaseErr, Debug)]
pub enum JsonErr {
    #[error("{0}")]
    SerdeJsonError(#[from] SerdeJsonErr),
    #[error("`{0}` not found or is not {1} type")]
    ParseError(&'static str, &'static str),
}

#[derive(BaseErr, Debug)]
pub enum Error {
    #[error("RequestError: {0}")]
    RequestError(#[from] RequestError),
    #[error("ServerError: {0}")]
    ServerError(#[from] ServerErr),
    #[error("Failed to parse JSON: {0}")]
    JsonParseFail(#[from] JsonErr),
    #[error("Failed to configure log: {0}")]
    LogConfigFailed(#[from] LogConfErr),
    #[error("Failed to setup log: {0}")]
    LogSetupFailed(String),
    #[error("Failed to crypto: {0}")]
    CryptoFailed(#[from] CryptoErr),
    #[error("Failed to convert a string: {0}")]
    StringConvertFailed(#[from] ToStrError),
    #[error("Failed to parse the arguments: {0}")]
    ArgumentError(String),
    #[error("{0}")]
    CustomError(String),
    #[error("Error storing unknown data.")]
    Unknown,
}

pub type ErrorResult<T> = std::result::Result<T, Error>;
