use std::io::{self, Read};

use flate2::read::{DeflateDecoder, GzDecoder, ZlibDecoder};
use hyper::header::{HeaderValue, CONTENT_LENGTH};
use hyper::{Body, HeaderMap, Request};

use crate::error::{ServerError, ServerResult};

const CONTENT_LENGTH_DEFAULTS: i32 = 0;

fn get_encoding_from_header(header: &HeaderMap) -> String {
    header
        .get("content-encoding")
        .map(|hv| hv.to_str().unwrap_or(""))
        .unwrap_or("")
        .to_string()
}

fn decompress_data<T: Read>(mut decoder: T) -> io::Result<String> {
    let mut data = String::new();

    decoder.read_to_string(&mut data)?;
    Ok(data)
}

pub async fn extract_request_body(req: &mut Request<Body>) -> ServerResult<String> {
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
        _ => std::str::from_utf8(buf_ref)?.to_string(),
    };

    Ok(data)
}

pub fn get_header_host(header: &HeaderMap) -> ServerResult<String> {
    let header_host_default_value = HeaderValue::from_static("");
    let header_host = header
        .get("Host")
        .unwrap_or(&header_host_default_value)
        .to_str()?
        .to_string();

    Ok(header_host)
}

pub fn get_content_length(header: &HeaderMap) -> i32 {
    // Get the value of Content-Length in request.
    let content_length_header = header.get(CONTENT_LENGTH);

    if let Some(value) = content_length_header {
        value
            .to_str() // Turn the Content-Length to &str.
            .map(|v| {
                v.parse::<i32>() // Turn the Content-Length(&str) to i32.
                    .unwrap_or(CONTENT_LENGTH_DEFAULTS)
            })
            .unwrap_or(CONTENT_LENGTH_DEFAULTS)
    } else {
        CONTENT_LENGTH_DEFAULTS
    }
}
