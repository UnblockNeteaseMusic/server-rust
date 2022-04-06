//! The JSON utilities for the response UNM ecosystem received.

pub use serde_json::Value as Json;

/// Throws when the JSON is not able to extract.
///
/// (pointer, expected_type)
#[derive(Debug)]
pub struct UnableToExtractJson<'a> {
    pub json_pointer: &'a str,
    pub expected_type: &'a str,
}

impl<'a> std::error::Error for UnableToExtractJson<'a> {}
impl<'a> std::fmt::Display for UnableToExtractJson<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unable to extract json: {} (type: {})",
            self.json_pointer, self.expected_type
        )
    }
}
