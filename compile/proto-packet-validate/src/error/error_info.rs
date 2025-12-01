use clerr::{Severity, TokenInfo};
use colored::ColoredString;
use lex::{Context, Token};

/// The info for a validation error.
#[derive(Debug)]
pub struct ErrorInfo {
    pub code: &'static str,          // The report error code.
    pub header: String,              // The report error code message.
    pub entries: Vec<ColoredString>, // The error entry.
}

impl ErrorInfo {
    //! Mutations

    /// Adds the `entry`.
    pub fn with_entry(mut self, mut entry: Vec<ColoredString>) -> Self {
        entry.drain(..).for_each(|s| self.entries.push(s));
        self
    }

    /// Adds the token info.
    pub fn with_token_info(
        self,
        file_name: &str,
        context: Context,
        token: Token,
        message: &str,
    ) -> Self {
        let info: TokenInfo = TokenInfo {
            file_name,
            line: token.line() + 1,
            position: token.position(),
            line_text: context
                .get_line_text(token.line())
                .expect("error: invalid token context"),
            token_len: token.len(),
            severity: Severity::Error,
            message,
        };
        self.with_entry(info.entry())
    }
}
