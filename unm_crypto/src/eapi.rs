use std::error::Error;

use regex::Regex;
use serde::Serialize;

use crate::aes_128;
use crate::aes_128::AesResult;

const EAPI_KEY: &[u8; 16] = b"e82ckenh8dichen8";

pub fn decrypt(data: &[u8]) -> AesResult {
    aes_128::decrypt(data, EAPI_KEY)
}

pub fn encrypt(data: &[u8]) -> AesResult {
    aes_128::encrypt(data, EAPI_KEY)
}

#[allow(dead_code)]
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
        url: Regex::new("\\w*api")
            .unwrap()
            .replace(url, "eapi")
            .to_string(),
        // Since there is no special chars in the uppercase hex string,
        // we don't need to use something like serde_qs to serialize it.
        body: format!(
            "params={}",
            hex::encode(encrypt(data.as_bytes())?).to_uppercase()
        ),
    })
}
