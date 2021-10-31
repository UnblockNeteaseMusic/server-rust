use std::io::Read;

use flate2::read::{DeflateDecoder, GzDecoder, ZlibDecoder};
use hyper::{Body, HeaderMap, Request};

use crate::server::error::ServerError;
use crate::{Error, ErrorResult};

fn get_encoding_from_header(header: &HeaderMap) -> String {
    header
        .get("content-encoding")
        .map(|hv| hv.to_str().unwrap_or(""))
        .unwrap_or("")
        .to_string()
}

fn decompress_data<T: Read>(mut decoder: T) -> ErrorResult<String> {
    let mut data = String::new();

    decoder.read_to_string(&mut data)?;
    Ok(data)
}

pub async fn extract_request_body(req: &mut Request<Body>) -> ErrorResult<String> {
    let header = req.headers();
    let encoding = get_encoding_from_header(header);

    let buf = hyper::body::to_bytes(req)
        .await
        .map_err(|_| ServerError::BodyAggregateError)?;
    let buf_ref = buf.as_ref();

    let data = match encoding.as_ref() {
        "deflate" => decompress_data(DeflateDecoder::new(buf_ref))?,
        "gzip" => decompress_data(GzDecoder::new(buf_ref))?,
        "zlib" => decompress_data(ZlibDecoder::new(buf_ref))?,
        _ => std::str::from_utf8(buf_ref)
            .map_err(Error::DecodeUtf8Failed)?
            .to_string(),
    };

    Ok(data)
}
