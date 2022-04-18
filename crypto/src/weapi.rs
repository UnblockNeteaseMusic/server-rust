//! The utilities for working on WEAPI requests.
//!
//! It is originally from <https://github.com/Binaryify/NeteaseCloudMusicApi/blob/master/util/crypto.js>.
//! Thanks to Binaryify!

use once_cell::sync::OnceCell;
use openssl::{
    pkey::Public,
    rand::rand_bytes,
    rsa::{Padding, Rsa},
};
use serde::Serialize;
use serde_json::Value;
use smallvec::SmallVec;

use crate::error::{CryptoError, CryptoResult};

const BASE62_CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const WEAPI_PRESET_KEY: &[u8] = b"0CoJUm6Qyw8W8jud";
const WEAPI_IV: &[u8] = b"0102030405060708";
const WEAPI_PUBKEY: &[u8] = b"-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB\n-----END PUBLIC KEY-----";

static WEAPI_RSA_INSTANCE: OnceCell<Rsa<Public>> = OnceCell::new();

/// Generate random bytes with [`rand_bytes`], and store them in a [`SmallVec`].
pub fn gen_random_bytes<const LEN: usize>() -> CryptoResult<SmallVec<[u8; LEN]>> {
    let mut bytes = SmallVec::<[u8; LEN]>::new();

    rand_bytes(bytes.as_mut_slice())?;

    Ok(bytes)
}

fn gen_weapi_secret_key() -> CryptoResult<SmallVec<[u8; 16]>> {
    let bytes = gen_random_bytes::<16>()?;
    let b62_char_at = |n| {
        BASE62_CHARSET.chars().nth(n % 62).unwrap_or({
            log::error!(
                "{}. Fill 'a' instead.",
                CryptoError::UnexpectedIndex(n, BASE62_CHARSET.into())
            );
            'a'
        })
    };

    Ok(bytes
        .into_iter()
        .map(|n| {
            u8::try_from(b62_char_at(n as usize)).unwrap_or_else(|e| {
                log::error!("[char2u8] Out of range: {e}. Return 0 instead.");
                0
            })
        })
        .collect::<SmallVec<[u8; 16]>>())
}

fn get_weapi_rsa_instance() -> CryptoResult<&'static Rsa<Public>> {
    Ok(WEAPI_RSA_INSTANCE.get_or_try_init(|| Rsa::public_key_from_pem(WEAPI_PUBKEY))?)
}

/// Encrypts data using WEAPI's key, returning the number of encrypted bytes.
pub fn encrypt_with_weapi_rsa(data: &[u8], to: &mut [u8]) -> CryptoResult<usize> {
    let mut padded_data = SmallVec::<[u8; 128]>::with_capacity(128 - data.len());
    padded_data.fill(0u8);
    padded_data.extend_from_slice(data);

    Ok(get_weapi_rsa_instance()?.public_encrypt(padded_data.as_slice(), to, Padding::NONE)?)
}

pub fn construct_weapi_payload<S: Serialize>(object: &S) -> CryptoResult<Value> {
    let json_payload = serde_json::to_string(object)?;
    let mut secret_key = gen_weapi_secret_key()?;
    let mut buf = Vec::with_capacity(1024);

    let aes_128_b64 = |data| -> CryptoResult<String> {
        Ok(base64::encode(crate::aes_128::encrypt_cbc(
            data,
            WEAPI_PRESET_KEY,
            WEAPI_IV,
        )?))
    };

    // Reverse the secret key since it is the requirement of `enc_sec_key` (?)
    secret_key.reverse();

    /* Params */
    let params_inside = aes_128_b64(json_payload.as_bytes())?;
    let params = aes_128_b64(params_inside.as_bytes())?;

    /* encSecKey */
    encrypt_with_weapi_rsa(secret_key.as_slice(), buf.as_mut_slice())?;
    let enc_sec_key = faster_hex::hex_string(buf.as_slice());

    Ok(serde_json::json!({
        "params": params,
        "encSecKey": enc_sec_key,
    }))
}

// FIXME: tests
