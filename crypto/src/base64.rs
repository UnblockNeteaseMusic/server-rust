use crate::error::CryptoResult;
pub use openssl::base64::{decode_block, encode_block};

pub fn encode_crypto_base64(src: &[u8]) -> String {
    encode_block(src).replace('+', "-").replace('/', "_")
}

pub fn decode_crypto_base64(src: &str) -> CryptoResult<Vec<u8>> {
    let src = src.replace('+', "-").replace('/', "_");
    Ok(decode_block(&src)?)
}
