use lex::{white_line_comments, white_line_comments_optional, LexResult, Token};

/// The line-comment delimiter.
pub const LINE_COMMENT_DELIMITER: &'static str = "//";

/// Parses optionally empty whitespace, line-endings, & line-comments.
pub fn white_block(token: Token) -> (Token, Token) {
    if let (Some(white_line_comments), token) =
        white_line_comments_optional(token, LINE_COMMENT_DELIMITER)
    {
        (white_line_comments, token)
    } else {
        token.split(0)
    }
}

/// Parses optional non-empty whitespace, line-endings, & line-comments.
pub fn white_optional(token: Token) -> (Option<Token>, Token) {
    white_line_comments_optional(token, LINE_COMMENT_DELIMITER)
}

/// Parses non-empty whitespace, line-endings, & line-comments.
pub fn white(token: Token) -> LexResult<Token, ()> {
    white_line_comments(token, LINE_COMMENT_DELIMITER)
}
