use hyper::header::HeaderValue;
use hyper::{Body, Request};

use unm_macro::is_host_wrapper;

use crate::error::ErrorResult;

const NETEASE_MOCK_IP: &str = "118.88.88.88";

pub struct BeforeReqContext {
    pub decision: &'static str,
    pub url: String,
}

pub struct Hook {
    is_tls: bool,
}

impl Hook {
    pub fn before_req(&self, context: &Request<Body>) -> ErrorResult<BeforeReqContext> {
        let mut decision: &'static str = "";
        let url = context.uri().to_string();
        let header = context.headers();
        let header_host_default_value = &HeaderValue::from_static("");
        let header_host = header
            .get("Host")
            .unwrap_or(header_host_default_value)
            .to_str()?;
        let is_host = is_host_wrapper!(&url, header_host);

        if is_host(&"music.163.com") {
            decision = "proxy";
        }

        Ok(BeforeReqContext { decision, url })
    }
}
