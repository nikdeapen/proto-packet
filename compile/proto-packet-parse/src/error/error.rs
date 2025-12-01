use crate::ErrorInfo;

/// A parsing error.
pub trait Error {
    /// Gets the error info.
    fn info(&self, token: &str) -> ErrorInfo;
}
