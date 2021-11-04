use std::error::Error;

use serde::Serialize;
use url::Url;

use crate::common;

const LINUX_API_KEY: &[u8; 16] = b"rFgB&h#%2?^eDg:Q";

#[derive(Serialize)]
struct LinuxApiResponse<'a, T: Serialize> {
    method: &'static str,
    // Nothing than ourselves use this field. We can safely
    // use reference so we can reduce the memory usage.
    url: &'a str,
    params: T,
}

pub fn decrypt(data: &[u8]) -> common::CryptResponse {
    common::decrypt(data, LINUX_API_KEY)
}

pub fn encrypt(data: &[u8]) -> common::CryptResponse {
    common::encrypt(data, LINUX_API_KEY)
}

pub struct EncryptRequestResponse {
    url: String,
    body: String,
}

pub fn encrypt_request<T: Serialize>(
    url: &str,
    object: &T,
) -> Result<EncryptRequestResponse, Box<dyn Error>> {
    let response_url: Url = Url::parse(url).and_then(|url| url.join("/api/linux/forward"))?;
    let response = LinuxApiResponse {
        method: "POST",
        url,
        params: object,
    };
    let serialized = serde_json::to_string(&response)?;

    Ok(EncryptRequestResponse {
        url: response_url.to_string(),
        body: format!(
            "eparams={}",
            hex::encode(encrypt(serialized.as_bytes())?).to_uppercase()
        ),
    })
}
