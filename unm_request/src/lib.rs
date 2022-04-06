pub mod ext;

// FIXME: separate to a crate.
pub mod json;

use std::{collections::HashMap, time::Duration};

use http::{
    header::{HeaderMap, HeaderValue},
    Method,
};
use reqwest::{Body, Proxy, Response};
use thiserror::Error;
use url::Url;

/// Construct the request header.
///
/// `url` is the URL to request;
/// `additional` is the additional header to send to.
pub fn construct_header(
    url: &Url,
    additional: Option<HeaderMap>,
) -> RequestModuleResult<HeaderMap> {
    let mut header_map = HeaderMap::new();

    header_map.insert(http::header::HOST, {
        let host_value = url.host_str().ok_or(RequestModuleError::UrlWithoutHost)?;
        HeaderValue::from_str(host_value)?
    });
    header_map.insert(
        http::header::ACCEPT,
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    header_map.insert(
        http::header::ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate"),
    );
    header_map.insert(
        http::header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9"),
    );
    header_map.insert(http::header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.181 Safari/537.36"));

    if let Some(additional) = additional {
        header_map.extend(additional.into_iter());
    }

    Ok(header_map)
}

/// Translate the specified host to the user-specified one.
///
/// `map` must be comprised of the `(src, tgt)` pair.
#[inline]
pub fn translate_host<'a>(map: &'a HashMap<&str, &str>, host: &'a str) -> &'a str {
    map.get(&host).unwrap_or(&host)
}

/// The `translate_host()` wrapper for [`Url`].
pub fn translate_url<'a>(map: &'a HashMap<&str, &str>, url: &mut Url) -> RequestModuleResult<()> {
    let host = url.host_str();

    if let Some(host) = host {
        let new_host = translate_host(map, host).to_string();
        url.set_host(Some(&new_host))
            .map_err(RequestModuleError::InvalidHost)?;
    }

    Ok(())
}

/// Extract the JSON string from a JSONP response.
pub fn extract_jsonp(data: &str) -> String {
    // jsonp({"data": {"id": "1", "name": "test"}});
    //       ~ START HERE                        ~ END HERE

    let left_bracket_index = data.find('(').map(|idx| idx + 1);
    let right_bracket_index = data.rfind(')');

    data.chars()
        .skip(left_bracket_index.unwrap_or(0))
        .take(right_bracket_index.unwrap_or(data.len()) - left_bracket_index.unwrap_or(0))
        .collect()
}

/// Request the specified URL.
///
/// `method` is the method to request. It can be one of
/// `GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS` or `PATCH`.
///
/// `url` is the URL to request. You may call [`translate_url`] to
/// translate the host of your URL to your desired one.
///
/// `additional_headers` is the additional header to send to.
/// Whatever you passed it or not, we will call
/// [`construct_header`] to construct the header.
///
/// `body` is the body to send to the server.
///
/// `proxy` is the proxy to use.
pub async fn request(
    method: Method,
    url: &Url,
    additional_headers: Option<HeaderMap>,
    body: Option<Body>,
    proxy: Option<Proxy>,
) -> RequestModuleResult<Response> {
    // Construct the headers according to the URL and the optional `additional_headers`.
    let headers = construct_header(url, additional_headers)?;

    // Construct the client builder.
    let mut client_builder = reqwest::Client::builder();

    // Set the proxy if the user specified it.
    if let Some(proxy) = proxy {
        client_builder = client_builder.proxy(proxy);
    }

    // Set the timeout (10s), then build the client.
    let client = client_builder
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(RequestModuleError::ConstructClientFailed)?;

    // Consturct the request. Here, we set up the method
    // and the headers to send to the server.
    let mut response_builder = client.request(method, url.to_string()).headers(headers);

    // Set the body if the user specified it.
    if let Some(body) = body {
        response_builder = response_builder.body(body);
    }

    // Send the request and get the response.
    let response = response_builder
        .send()
        .await
        .map_err(RequestModuleError::RequestFailed)?;

    // Return it.
    Ok(response)
}

/// Error in this module.
#[derive(Error, Debug)]
pub enum RequestModuleError {
    /// No host in the specified URL.
    #[error("no host in the specified URL.")]
    UrlWithoutHost,
    /// Invalid header value.
    #[error("invalid header value: {0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    /// Invalid host.
    #[error("invalid host: {0}")]
    InvalidHost(url::ParseError),
    /// Construct client failed.
    #[error("failed to construct client: {0}")]
    ConstructClientFailed(reqwest::Error),
    /// Request failed.
    #[error("failed to request: {0}")]
    RequestFailed(reqwest::Error),
}

/// The [`Result`] of this module.
pub type RequestModuleResult<T> = Result<T, RequestModuleError>;

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    static TRANSLATE_MAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("www.cloudflare.com", "1.1.1.1");
        map.insert("www.google.com", "www.bing.com");

        map
    });

    mod construct_header {
        use super::super::construct_header;

        use http::{HeaderMap, HeaderValue};
        use once_cell::sync::Lazy;
        use url::Url;

        static TEST_URL: Lazy<Url> = Lazy::new(|| Url::parse("https://www.baidu.com").unwrap());

        #[test]
        fn can_construct_the_correct_header_without_header_specified() {
            let constructed_header = construct_header(&TEST_URL, None).unwrap();

            assert_eq!(
                constructed_header.get(http::header::HOST).unwrap(),
                &HeaderValue::from_static("www.baidu.com")
            );
            assert_eq!(
                constructed_header.get(http::header::ACCEPT),
                Some(&HeaderValue::from_static(
                    "application/json, text/plain, */*"
                ))
            );
            assert_eq!(
                constructed_header.get(http::header::ACCEPT_ENCODING),
                Some(&HeaderValue::from_static("gzip, deflate"))
            );
            assert_eq!(
                constructed_header.get(http::header::ACCEPT_LANGUAGE),
                Some(&HeaderValue::from_static("zh-CN,zh;q=0.9"))
            );
            assert_eq!(constructed_header.get(http::header::USER_AGENT), Some(&HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.181 Safari/537.36")));
        }

        #[test]
        fn can_construct_the_correct_header_with_header_specified() {
            let mut additional_header = HeaderMap::new();
            additional_header.insert(
                http::header::COOKIE,
                HeaderValue::from_static("cookie=value"),
            );

            let additional_header = additional_header;
            let constructed_header = construct_header(&TEST_URL, Some(additional_header)).unwrap();

            assert_eq!(
                constructed_header.get(http::header::HOST).unwrap(),
                &HeaderValue::from_static("www.baidu.com")
            );
            assert_eq!(
                constructed_header.get(http::header::ACCEPT),
                Some(&HeaderValue::from_static(
                    "application/json, text/plain, */*"
                ))
            );
            assert_eq!(
                constructed_header.get(http::header::ACCEPT_ENCODING),
                Some(&HeaderValue::from_static("gzip, deflate"))
            );
            assert_eq!(
                constructed_header.get(http::header::ACCEPT_LANGUAGE),
                Some(&HeaderValue::from_static("zh-CN,zh;q=0.9"))
            );
            assert_eq!(constructed_header.get(http::header::USER_AGENT), Some(&HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.181 Safari/537.36")));
            assert_eq!(
                constructed_header.get(http::header::COOKIE),
                Some(&HeaderValue::from_static("cookie=value"))
            );
        }
    }

    mod translate_host {
        use super::super::translate_host;

        #[test]
        fn test_translate_host() {
            assert_eq!(
                translate_host(&super::TRANSLATE_MAP, "www.cloudflare.com"),
                "1.1.1.1"
            );
            assert_eq!(
                translate_host(&super::TRANSLATE_MAP, "www.google.com"),
                "www.bing.com"
            );
        }
    }

    mod translate_url {
        use url::Url;

        use super::super::translate_url;

        #[test]
        fn test_translate_url() {
            let mut testdata_1 = "https://www.cloudflare.com/owo".parse::<Url>().unwrap();
            let mut testdata_2 = "https://www.google.com/?search=Rickroll"
                .parse::<Url>()
                .unwrap();

            translate_url(&super::TRANSLATE_MAP, &mut testdata_1).unwrap();
            translate_url(&super::TRANSLATE_MAP, &mut testdata_2).unwrap();

            assert_eq!(testdata_1.to_string(), "https://1.1.1.1/owo");
            assert_eq!(
                testdata_2.to_string(),
                "https://www.bing.com/?search=Rickroll"
            );
        }
    }

    mod extract_jsonp {
        use once_cell::sync::Lazy;

        use super::super::extract_jsonp;

        struct Testdata {
            pub src: &'static str,
            pub testdata: Vec<String>,
        }

        static TEST_DATA_SETS: Lazy<Vec<Testdata>> = Lazy::new(|| {
            vec![
                "[100,500,300,200,400]",
                r##"[{"color":"red","value":"#f00"},{"color":"green","value":"#0f0"},{"color":"blue","value":"#00f"},{"color":"cyan","value":"#0ff"},{"color":"magenta","value":"#f0f"},{"color":"yellow","value":"#ff0"},{"color":"black","value":"#000"}]"##,
                r##"{
                    "color":"red",
                    "value":"#f00"
                }"##]
                .into_iter()
                .map(|json| Testdata {
                    src: json,
                    testdata: vec![
                        format!("qq({json})"),
                        format!("qq({json});"),
                        format!("fg_1odla({json})"),
                        format!("$fkaolv({json});;;"),
                    ],
                })
                .collect::<Vec<_>>()
        });

        #[test]
        fn test_extract_jsonp() {
            for test_data_set in TEST_DATA_SETS.iter() {
                for test_data in test_data_set.testdata.iter() {
                    assert_eq!(extract_jsonp(test_data.as_str()), test_data_set.src);
                }
            }
        }
    }
}
