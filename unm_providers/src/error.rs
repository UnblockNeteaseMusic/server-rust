use thiserror::Error;

use unm_proxy_server::ServerError;
use unm_request::RequestError;

#[derive(Error, Debug)]
pub enum ProvidersError {
    #[error("Request Error: {0}")]
    RequestError(#[from] RequestError),
    #[error("Server Error: {0}")]
    ServerError(#[from] ServerError),

    #[error("Serde Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("`{0}` not found or is not {1} type")]
    ParseError(&'static str, &'static str),
}

pub type ProvidersResult<T> = Result<T, ProvidersError>;
