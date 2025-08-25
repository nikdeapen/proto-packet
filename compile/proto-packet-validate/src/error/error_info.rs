use clerr::{Report, Severity};
use colored::ColoredString;
use lex::{ParseContext, Token};

/// The info for a validation error.
#[derive(Debug)]
pub struct ErrorInfo {
    pub code: &'static str,       // The report error code.
    pub header: String,           // The report error code message.
    pub info: Vec<ColoredString>, // The error entry.
}

impl ErrorInfo {
    //! Mutations

    /// Adds the `info`.
    pub fn with_info(mut self, mut info: Vec<ColoredString>) -> Self {
        info.drain(..).for_each(|s| self.info.push(s));
        self
    }

    /// Adds the token info.
    pub fn with_token_info(
        self,
        file_name: &str,
        context: ParseContext,
        token: Token,
        message: &str,
    ) -> Self {
        self.with_info(Report::token_info(
            file_name,
            token.line() + 1,
            token.position(),
            context
                .get_line_text(token.line())
                .expect("invalid token for context"),
            token.len(),
            Severity::Error,
            message,
        ))
    }
}
