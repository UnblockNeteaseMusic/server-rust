use crate::Error;

pub trait Middleware {
    type ContextT;

    fn execute(context: &mut Self::ContextT) -> Result<(), Error>;
}
