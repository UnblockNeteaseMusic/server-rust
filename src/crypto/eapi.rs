//! The port of `crypto.js`. Commit `eb8e5691272e0b5ee7f70b317ebbce32403ea6b4`.

use openssl::error::ErrorStack;
use openssl::symm::{decrypt as symm_decrypt, encrypt as symm_encrypt, Cipher};
use serde::Serialize;
use std::error::Error;

const EAPI_KEY: &[u8; 16] = b"e82ckenh8dichen8";
const LINUX_API_KEY: &[u8; 16] = b"rFgB&h#%2?^eDg:Q";

type CryptResponse = Result<Vec<u8>, ErrorStack>;

fn decrypt(data: &[u8], key: &[u8]) -> CryptResponse {
    let cipher = Cipher::aes_128_ecb();
    symm_decrypt(cipher, key, None, data)
}

fn encrypt(data: &[u8], key: &[u8]) -> CryptResponse {
    let cipher = Cipher::aes_128_ecb();
    symm_encrypt(cipher, key, None, data)
}

pub fn decrypt_eapi(data: &[u8]) -> CryptResponse {
    decrypt(data, EAPI_KEY)
}

pub fn encrypt_eapi(data: &[u8]) -> CryptResponse {
    encrypt(data, EAPI_KEY)
}

pub struct EncryptRequestResponse {
    url: String,
    body: String,
}

pub fn encrypt_request<T: Serialize>(
    url: &str,
    object: &T,
) -> Result<EncryptRequestResponse, Box<dyn Error>> {
    let serialized: String = serde_json::to_string(object)?;
    let message = format!("deprecate{}md5{}please", url, serialized);
    let digest = md5::compute(message.into_bytes());
    let data = format!(
        "{}-36cd479b6b5-{}-36cd479b6b5-{:x}",
        url, serialized, digest
    );

    Ok(EncryptRequestResponse {
        url: url.to_string(),
        body: hex::encode(encrypt_eapi(data.as_bytes())?),
    })
}
