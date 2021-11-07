use std::io::Error as IoErr;

use thiserror::Error as BaseErr;
use tokio::task::JoinError;

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

#[derive(BaseErr, Debug)]
pub enum ServerServeError {
    #[error("Error serving HTTP server: {0}.")]
    HttpError(JoinError),
    #[error("Error serving HTTPS server: {0}.")]
    HttpsError(JoinError),
}

pub type ServerResult<T> = Result<T, ServerError>;
pub type ServerServeResult<T> = Result<T, ServerServeError>;
