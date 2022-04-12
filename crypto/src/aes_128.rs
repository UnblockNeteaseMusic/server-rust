use crate::error::CryptoResult;
use openssl::symm::{decrypt as symm_decrypt, encrypt as symm_encrypt, Cipher};

pub type AesResult = CryptoResult<Vec<u8>>;

pub fn decrypt(data: &[u8], key: &[u8]) -> AesResult {
    let cipher = Cipher::aes_128_ecb();
    Ok(symm_decrypt(cipher, key, None, data)?)
}

pub fn encrypt(data: &[u8], key: &[u8]) -> AesResult {
    let cipher = Cipher::aes_128_ecb();
    Ok(symm_encrypt(cipher, key, None, data)?)
}
