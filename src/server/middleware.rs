use crate::Error;

pub trait Middleware {
    type Request;

    fn execute(request: &mut Self::Request) -> Result<(), Error>;
}
