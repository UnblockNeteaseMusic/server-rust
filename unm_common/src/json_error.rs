use serde_json::Error as SerdeJsonErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonErr {
    #[error("{0}")]
    SerdeJsonError(#[from] SerdeJsonErr),
    #[error("`{0}` not found or is not {1} type")]
    ParseError(&'static str, &'static str),
}

pub type JsonResult<T> = Result<T, JsonErr>;
