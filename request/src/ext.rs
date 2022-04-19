use std::borrow::Cow;

use reqwest::{Body, Response};
use serde::de::DeserializeOwned;
use thiserror::Error;

use super::extract_jsonp;

#[async_trait::async_trait]
pub trait ResponseExt {
    async fn jsonp<T: DeserializeOwned>(self) -> RequestExtModuleResult<T>;
}

#[async_trait::async_trait]
impl ResponseExt for Response {
    async fn jsonp<T: DeserializeOwned>(self) -> RequestExtModuleResult<T> {
        let json_str = {
            let jsonp_str = self
                .text()
                .await
                .map_err(RequestExtModuleError::GetTextFailed)?;
            extract_jsonp(&jsonp_str)
        };

        serde_json::from_str::<T>(&json_str)
            .map_err(RequestExtModuleError::DeserializeFailed)
    }
}

/// The extension method of [`Body`].
#[deprecated = "I implemented it in a wrong structure…"]
pub trait BodyExt<'a> {
    /// Transform the body to a String.
    fn to_text(&'a self) -> RequestExtModuleResult<Cow<'a, str>>;

    /// Transform the body to a JSON structure.
    fn to_json<T: DeserializeOwned>(&self) -> RequestExtModuleResult<T>;

    /// Transform the JSONP body to a JSON structure.
    fn to_jsonp<T: DeserializeOwned>(&self) -> RequestExtModuleResult<T>;
}

impl<'a> BodyExt<'a> for Body {
    fn to_text(&'a self) -> RequestExtModuleResult<Cow<'a, str>> {
        let bytes = self
            .as_bytes()
            .ok_or(RequestExtModuleError::NoBytesAvailable)?;
        let str = String::from_utf8_lossy(bytes);

        Ok(str)
    }

    fn to_json<T: DeserializeOwned>(&self) -> RequestExtModuleResult<T> {
        let text = self.to_text()?;

        serde_json::from_str(&text).map_err(RequestExtModuleError::DeserializeFailed)
    }

    fn to_jsonp<T: DeserializeOwned>(&self) -> RequestExtModuleResult<T> {
        let text = {
            let raw = self.to_text()?;
            extract_jsonp(&raw)
        };

        serde_json::from_str(&text).map_err(RequestExtModuleError::DeserializeFailed)
    }
}

/// Error in this module.
#[derive(Error, Debug)]
pub enum RequestExtModuleError {
    /// No bytes available.
    #[error("no bytes available")]
    #[deprecated]
    NoBytesAvailable,

    #[error("failed to call .text() on the response: {0}")]
    GetTextFailed(reqwest::Error),

    #[error("failed to deserialize to a struct: {0}")]
    DeserializeFailed(serde_json::Error),
}

/// The [`Result`] of this module.
pub type RequestExtModuleResult<T> = Result<T, RequestExtModuleError>;

#[cfg(test)]
mod tests {
    mod body_ext {
        use reqwest::Body;
        use serde::Deserialize;

        use super::super::BodyExt;

        #[derive(Deserialize)]
        struct MockJsonStructure {
            name: String,
            age: u8,
        }

        #[test]
        fn test_to_text() {
            let body: Body = "Hello, world!".into();
            let text = body.to_text().unwrap();
            assert_eq!(text, "Hello, world!");
        }

        #[test]
        fn test_to_json() {
            let body: Body = r#"{"name": "uwu", "age": 11}"#.into();
            let text = body.to_json::<MockJsonStructure>().unwrap();
            assert_eq!(text.name, "uwu");
            assert_eq!(text.age, 11);
        }

        #[test]
        fn test_to_jsonp() {
            let body: Body = r#"receiver({"name": "uwu", "age": 11});"#.into();
            let text = body.to_jsonp::<MockJsonStructure>().unwrap();
            assert_eq!(text.name, "uwu");
            assert_eq!(text.age, 11);
        }
    }
}
