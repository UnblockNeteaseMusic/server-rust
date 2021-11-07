use thiserror::Error;

use unm_common::JsonErr;
use unm_proxy_server::ServerError;
use unm_request::RequestError;

#[derive(Error, Debug)]
pub enum ProvidersError {
    #[error("Request Error: {0}")]
    RequestError(#[from] RequestError),

    #[error("Server Error: {0}")]
    ServerError(#[from] ServerError),

    #[error("Json Error: {0}")]
    JsonError(#[from] JsonErr),
}

pub type ProvidersResult<T> = Result<T, ProvidersError>;
