use crate::error::ServerError;
use async_trait::async_trait;
use serde_json::Value;

pub enum Decision {
    Proxy,
}

pub struct Context {
    /// The hook decision.
    pub decision: Decision,
    /// The context from Netease Music.
    pub netease_context: Option<NeteaseApiContext>,
    pub target_host: &'static [&'static str],
}

#[async_trait]
pub trait Middleware {
    type Request;

    async fn execute(request: &mut Self::Request, context: &mut Context)
        -> Result<(), ServerError>;
}

pub struct NeteaseApiContext {
    pub pad: String,
    pub forward: bool,
    pub path: String,
    pub param: Option<Value>,
}

impl NeteaseApiContext {
    pub fn new() -> Self {
        NeteaseApiContext {
            pad: "".to_string(),
            forward: false,
            path: "".to_string(),
            param: None,
        }
    }
}
