use lex::{ParseContext, Token};

/// Parses a qualified name.
///
/// Returns `(Some(qualified_name), after_qualified_name)`.
/// Returns `(None, c)` when the next token is not a qualified name.
pub fn parse_qualified_name(c: ParseContext) -> (Option<Token>, ParseContext) {
    if let (Some(name), after_name) =
        unsafe { c.match_prefix_optional(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'.') }
    {
        (Some(name.token()), after_name)
    } else {
        (None, c)
    }
}
