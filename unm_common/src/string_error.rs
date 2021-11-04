use std::str::Utf8Error;

use http::header::ToStrError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StringError {
    #[error("Failed to convert a string: {0}")]
    StringConvertFailed(#[from] ToStrError),
    #[error("Failed to decode UTF-8 array: {0}")]
    DecodeUtf8Failed(#[from] Utf8Error),
}

pub type StringResult<T> = Result<T, StringError>;
