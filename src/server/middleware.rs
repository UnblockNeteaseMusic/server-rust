use crate::Error;

pub enum Decision {
    Proxy,
}

pub struct Context {
    /// The hook decision.
    pub decision: Decision,
    pub target_host: &'static [&'static str],
}

pub trait Middleware {
    type Request;

    fn execute(request: &mut Self::Request, context: &mut Context) -> Result<(), Error>;
}
