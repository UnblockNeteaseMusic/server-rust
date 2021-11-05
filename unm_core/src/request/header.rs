use http::{HeaderMap, HeaderValue};
use phf::phf_map;

static DEFAULT_HEADERS_SET: phf::Map<&'static str, &'static str> = phf_map! {
    "accept" => "application/json, text/plain, */*",
    "accept-encoding" => "gzip, deflate",
    "accept-language" => "zh-CN,zh;q=0.9",
    "user-agent" => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4677.0 Safari/537.36"
};

pub fn default_headers() -> HeaderMap {
    // Inspired from https://github.com/cnsilvan/UnblockNeteaseMusic/blob/master/network/network.go#L107

    let mut headers = HeaderMap::new();

    for &header_name in DEFAULT_HEADERS_SET.keys() {
        if !headers.contains_key(header_name) {
            let header_value = HeaderValue::from_static(
                DEFAULT_HEADERS_SET
                    .get(header_name)
                    .expect("header_name must be in DEFAULT_HEADERS_SET since that name is from DEFAULT_HEADERS_SET")
            );
            headers.insert(header_name, header_value);
        }
    }

    headers
}
