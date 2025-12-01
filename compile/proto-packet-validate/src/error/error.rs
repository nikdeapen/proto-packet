use crate::ErrorInfo;
use lex::Context;

/// An error validating a parsed schema file.
pub trait Error {
    /// Gets the error info.
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo;
}
