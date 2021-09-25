pub use http::{HeaderMap, HeaderValue};

// accept: 'application/json, text/plain, */*',
// 'accept-encoding': 'gzip, deflate',
// 'accept-language': 'zh-CN,zh;q=0.9',
// 'user-agent':
// 	'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.181 Safari/537.36',
pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "accept",
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers.insert(
        "accept-language",
        HeaderValue::from_static("zh-CN,zh;q=0.9"),
    );
    return headers;
}
