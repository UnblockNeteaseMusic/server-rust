use openssl::error::ErrorStack;
use openssl::symm::{decrypt as symm_decrypt, encrypt as symm_encrypt, Cipher};

pub type CryptResponse = Result<Vec<u8>, ErrorStack>;

pub fn decrypt(data: &[u8], key: &[u8]) -> CryptResponse {
    let cipher = Cipher::aes_128_ecb();
    symm_decrypt(cipher, key, None, data)
}

pub fn encrypt(data: &[u8], key: &[u8]) -> CryptResponse {
    let cipher = Cipher::aes_128_ecb();
    symm_encrypt(cipher, key, None, data)
}
