use crate::error::P_STRUCT_FIELD;
use crate::ParseStructFieldError::*;
use crate::{parse_type_tag, Error, ErrorInfo, ParseTypeTagError, TypeTagTree};
use lex::{Context, ParseResult, Token};

#[derive(Debug)]
pub struct StructFieldTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub field_name: Token<'a>,
    pub type_tag: TypeTagTree<'a>,
}

#[derive(Debug)]
pub enum ParseStructFieldError {
    ExpectedColon,
    InvalidTypeTag(ParseTypeTagError),
    ExpectedSemicolon,
}

impl<'a> Error for ParseStructFieldError {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self {
            ExpectedColon => "a colon",
            InvalidTypeTag(e) => return e.info(token),
            ExpectedSemicolon => "a semicolon",
        };
        P_STRUCT_FIELD.expected_got_instead(expected, token)
    }
}

/// Parses an optional struct field.
///
/// Returns `Ok(struct_field, after_semicolon)`.
/// Returns `Ok((None, c))` if the next non-white token is not a symbol.
pub fn parse_struct_field(
    c: Context,
) -> ParseResult<Option<StructFieldTree>, ParseStructFieldError> {
    let (comments, after_comments) = c.line_comment_block();
    if let (Some(field_name), after_field_name) = after_comments.symbol() {
        let (_white, after_white) = after_field_name.whitespace();
        if let (Some(_colon), after_colon) = after_white.exact(":") {
            let (type_tag, after_type_tag) =
                parse_type_tag(after_colon).map_err(|e| e.map(|e| InvalidTypeTag(e)))?;
            let (_white, after_white) = after_type_tag.whitespace();
            if let (Some(_semicolon), after_semicolon) = after_white.exact(";") {
                let tree: StructFieldTree = StructFieldTree {
                    comments,
                    field_name,
                    type_tag,
                };
                Ok((Some(tree), after_semicolon))
            } else {
                Err(after_white.to_error(ExpectedSemicolon))
            }
        } else {
            Err(after_white.to_error(ExpectedColon))
        }
    } else {
        Ok((None, c))
    }
}
