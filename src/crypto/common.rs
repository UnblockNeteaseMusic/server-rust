use openssl::error::ErrorStack;
use openssl::symm::{decrypt as symm_decrypt, encrypt as symm_encrypt, Cipher};
use thiserror::Error as BaseErr;

#[derive(BaseErr, Debug)]
pub enum CryptoError {
    #[error("Failed in OpenSSL: {0}")]
    OpenSSLFail(ErrorStack),
    #[error("Failed to XOR this ID char (u32) {0} with this key char (u32) {1}")]
    UriEncryptXorFail(u32, u32),
}

pub type CryptResponse = Result<Vec<u8>, ErrorStack>;

pub fn decrypt(data: &[u8], key: &[u8]) -> CryptResponse {
    let cipher = Cipher::aes_128_ecb();
    symm_decrypt(cipher, key, None, data)
}

pub fn encrypt(data: &[u8], key: &[u8]) -> CryptResponse {
    let cipher = Cipher::aes_128_ecb();
    symm_encrypt(cipher, key, None, data)
}
