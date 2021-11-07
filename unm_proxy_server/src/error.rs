use hex::FromHexError;
use std::io::Error as IoErr;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use thiserror::Error as BaseErr;
use tokio::task::JoinError;

#[derive(BaseErr, Debug)]
pub enum ServerError {
    #[error("The request is invalid.")]
    InvalidRequest,
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),

    #[error("Failed to extract 'Host' field in your header.")]
    ExtractHostFailed,
    #[error("Failed to aggregate body.")]
    BodyAggregateError,

    #[error("Failed to convert a header value to string: {0}")]
    HeaderToStringFailed(#[from] http::header::ToStrError),
    #[error("Failed to decode UTF-8 array: {0}")]
    DecodeUtf8Failed(#[from] Utf8Error),
    #[error("Error converting UTF-8 array to String: {0}")]
    StringFromUtf8Error(#[from] FromUtf8Error),
    #[error("I/O error: {0}")]
    IoError(#[from] IoErr),
    #[error("Error decoding from hex: {0}")]
    HexDecodeError(#[from] FromHexError),
    #[error("Error in serde_json: {0}")]
    SerdeError(#[from] serde_json::error::Error),
    #[error("Error in unm_crypto: {0}")]
    UnmCryptoError(#[from] unm_crypto::error::CryptoError),
}

#[derive(BaseErr, Debug)]
pub enum ServerServeError {
    #[error("Error serving HTTP server: {0}.")]
    HttpError(JoinError),
    #[error("Error serving HTTPS server: {0}.")]
    HttpsError(JoinError),
}

pub type ServerResult<T> = Result<T, ServerError>;
pub type ServerServeResult<T> = Result<T, ServerServeError>;
