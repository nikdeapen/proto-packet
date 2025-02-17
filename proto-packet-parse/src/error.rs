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

pub const P_SCHEMA_FILE: &'static str = "P_SCHEMA_FILE";
pub const P_IMPORT: &'static str = "P_IMPORT";
pub const P_TYPE_TAG: &'static str = "P_TYPE_TAG";
pub const P_TAG_NUMBER: &'static str = "P_TAG_NUMBER";
pub const P_MESSAGE: &'static str = "P_MESSAGE";
pub const P_MESSAGE_FIELD: &'static str = "P_MESSAGE_FIELD";
pub const P_ENUM: &'static str = "P_ENUM";
pub const P_ENUM_CASE: &'static str = "P_ENUM_CASE";

/// Creates the file token info message for `expected x, got y instead`.
pub fn expected_got_instead(expected: &str, got_instead: &str) -> String {
    if got_instead.is_empty() {
        format!("expected {}, got EOF instead", expected)
    } else {
        format!("expected {}, got `{}` instead", expected, got_instead)
    }
}
