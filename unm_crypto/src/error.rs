use openssl::error::ErrorStack;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Failed in OpenSSL: {0}")]
    OpenSSLFail(ErrorStack),
    #[error("Failed to XOR this ID char (u32) {0} with this key char (u32) {1}")]
    UriEncryptXorFail(u32, u32),
}

pub type CryptoResult<T> = Result<T, CryptoError>;
