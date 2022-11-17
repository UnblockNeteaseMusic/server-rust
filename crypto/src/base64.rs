pub use base64::{decode, encode};

use crate::error::CryptoResult;

pub fn encode_crypto_base64(src: &[u8]) -> String {
    encode(src).replace('+', "-").replace('/', "_")
}

pub fn decode_crypto_base64(src: &str) -> CryptoResult<Vec<u8>> {
    let src = src.replace('+', "-").replace('/', "_");
    Ok(decode(src)?)
}
