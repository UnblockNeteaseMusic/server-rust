use std::io::Error as IoErr;

use thiserror::Error as BaseErr;

use unm_common::{JsonErr, StringError};

#[derive(BaseErr, Debug)]
pub enum ServerError {
    #[error("Failed to extract 'Host' field in your header.")]
    ExtractHostFailed,
    #[error("The request is invalid.")]
    InvalidRequest,
    #[error("Failed to aggregate body.")]
    BodyAggregateError,
    #[error("I/O error: {0}")]
    IoError(#[from] IoErr),
    #[error("JSON Error: {0}")]
    JsonError(#[from] JsonErr),
    #[error("String Error: {0}")]
    StringError(#[from] StringError),
}

pub type ServerResult<T> = Result<T, ServerError>;
