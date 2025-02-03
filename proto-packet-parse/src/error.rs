/// A parsing error.
pub trait Error {
    /// Gets the error info.
    fn info(&self, token: &str) -> ErrorInfo;
}

/// The info for a parsing error.
#[derive(Debug)]
pub struct ErrorInfo {
    pub code: &'static str,   // The report error code.
    pub header: &'static str, // The report error code message.
    pub message: String,      // The file token info message.
}

pub const P_TYPE_TAG: &'static str = "P_TYPE_TAG";
pub const P_TAG_NUMBER: &'static str = "P_TAG_NUMBER";
pub const P_MESSAGE_FIELD: &'static str = "P_MESSAGE_FIELD";

/// Creates the file token info message for `expected x, got y instead`.
pub fn expected_got_instead(expected: &str, got_instead: &str) -> String {
    if got_instead.is_empty() {
        format!("expected {}, got EOF instead", expected)
    } else {
        format!("expected {}, got `{}` instead", expected, got_instead)
    }
}
