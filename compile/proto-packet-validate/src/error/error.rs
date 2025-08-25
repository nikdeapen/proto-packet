use lex::ParseContext;

use crate::ErrorInfo;

/// An error validating a parsed schema file.
pub trait Error {
    /// Gets the error info.
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo;
}
