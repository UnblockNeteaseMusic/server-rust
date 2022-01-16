/// Throws when the JSON is not able to extract.
///
/// (pointer, expected_type)
#[derive(Debug)]
pub struct UnableToExtractJson<'a>(pub &'a str, pub &'a str);
impl<'a> std::error::Error for UnableToExtractJson<'a> {}
impl<'a> std::fmt::Display for UnableToExtractJson<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to extract json: {} (type: {})", self.0, self.1)
    }
}
