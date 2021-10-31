use hyper::Body;
use thiserror::Error as BaseErr;

#[derive(BaseErr, Debug)]
pub enum ServerError {
    #[error("Failed to extract 'Host' field in your header.")]
    ExtractHostFailed,
    #[error("The request is invalid.")]
    InvalidRequest,
    #[error("Failed to aggregate body.")]
    BodyAggregateError,
}
