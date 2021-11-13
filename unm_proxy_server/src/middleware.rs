use crate::ServerResult;
use async_trait::async_trait;
use log::error;
use serde_json::Value;

pub enum Decision {
    Proxy,
    Close,
}

pub struct Context {
    /// The hook decision.
    pub decision: Decision,
    pub package: Option<Package>,
    /// The context from Netease Music.
    pub netease_context: Option<NeteaseApiContext>,
    pub target_host: &'static [&'static str],
}

#[async_trait]
pub trait Middleware {
    type Request;

    fn name(&self) -> String;

    async fn execute(&self, request: &mut Self::Request, context: &mut Context)
        -> ServerResult<()>;
}

pub async fn middleware_executor<'a, T: Middleware>(
    middleware: &'a T,
    middleware_context: MiddlewareContext<'a, T>,
) -> ServerResult<()> {
    let name = middleware.name();
    let result = middleware
        .execute(middleware_context.request, middleware_context.context)
        .await;

    if let Err(error) = result {
        error!("{}: {}", name, error);
        middleware_context.context.decision = Decision::Close;
        Err(error)
    } else {
        Ok(())
    }
}

pub struct MiddlewareContext<'a, T: Middleware> {
    request: &'a mut T::Request,
    context: &'a mut Context,
}

pub struct NeteaseApiContext {
    pub pad: Option<String>,
    pub web: bool,
    pub forward: bool,
    pub path: Option<String>,
    pub param: Option<Value>,
}

impl NeteaseApiContext {
    pub fn new() -> Self {
        NeteaseApiContext {
            pad: None,
            web: false,
            forward: false,
            path: None,
            param: None,
        }
    }
}

pub struct Package {
    pub id: String,
}
