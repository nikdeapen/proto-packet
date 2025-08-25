use crate::error::P_STRUCT_FIELD;
use crate::ParseStructFieldErrorReason::*;
use crate::{parse_type_tag, Error, ErrorInfo, ParseTypeTagError, TypeTagTree};
use lex::{ParseContext, ParseResult, Token};

#[derive(Debug)]
pub struct StructFieldTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub field_name: Token<'a>,
    pub type_tag: TypeTagTree<'a>,
}

#[derive(Debug)]
pub struct ParseStructFieldError<'a> {
    pub field_name: Token<'a>,
    pub reason: ParseStructFieldErrorReason,
}

#[derive(Debug)]
pub enum ParseStructFieldErrorReason {
    ExpectedColon,
    InvalidTypeTag(ParseTypeTagError),
    ExpectedSemicolon,
}

impl<'a> Error for ParseStructFieldError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self.reason {
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
    c: ParseContext,
) -> ParseResult<Option<StructFieldTree>, ParseStructFieldError> {
    let (comments, after_comments) = c.line_comment_block();
    if let (Some(field_name), after_field_name) = after_comments.symbol() {
        let (tree, after_semicolon) =
            parse_with_field_name(comments, field_name, after_field_name)?;
        Ok((Some(tree), after_semicolon))
    } else {
        Ok((None, c))
    }
}

fn parse_with_field_name<'a>(
    comments: Vec<Token<'a>>,
    field_name: Token<'a>,
    c: ParseContext<'a>,
) -> ParseResult<'a, StructFieldTree<'a>, ParseStructFieldError<'a>> {
    let (_white, after_white) = c.whitespace();
    if let (Some(_colon), after_colon) = after_white.exact(":") {
        let (type_tag, after_type_tag) = parse_type_tag(after_colon).map_err(|e| {
            e.map(|e| ParseStructFieldError {
                field_name,
                reason: InvalidTypeTag(e),
            })
        })?;
        let (_white, after_white) = after_type_tag.whitespace();
        if let (Some(_semicolon), after_semicolon) = after_white.exact(";") {
            let tree: StructFieldTree = StructFieldTree {
                comments,
                field_name,
                type_tag,
            };
            Ok((tree, after_semicolon))
        } else {
            Err(after_white.to_error(ParseStructFieldError {
                field_name,
                reason: ExpectedSemicolon,
            }))
        }
    } else {
        Err(after_white.to_error(ParseStructFieldError {
            field_name,
            reason: ExpectedColon,
        }))
    }
}
