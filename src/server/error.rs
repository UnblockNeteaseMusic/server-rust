use thiserror::Error as BaseErr;

#[derive(BaseErr, Debug)]
pub enum ServerError {
    #[error("Failed to extract 'Host' field in your header.")]
    ExtractHostFailed,
}
