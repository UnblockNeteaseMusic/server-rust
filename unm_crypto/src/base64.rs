pub use openssl::base64::{decode_block, encode_block};
use openssl::error::ErrorStack;

pub fn encode_crypto_base64(src: &[u8]) -> String {
    encode_block(src).replace("+", "-").replace("/", "_")
}

pub fn decode_crypto_base64(src: &str) -> Result<Vec<u8>, ErrorStack> {
    let src = src.replace("+", "-").replace("/", "_");
    decode_block(&src)
}
