use crate::ParseMessageFieldErrorReason::*;
use crate::{
    parse_tag_number, parse_type_tag, Error, ErrorInfo, ParseTagNumberError, ParseTypeTagError,
    TypeTagTree, P_MESSAGE_FIELD,
};
use lex::{ParseContext, ParseResult, Token};

#[derive(Debug)]
pub struct MessageFieldTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub field_name: Token<'a>,
    pub type_tag: TypeTagTree<'a>,
    pub tag_number: Token<'a>,
}

#[derive(Debug)]
pub struct ParseMessageFieldError<'a> {
    pub field_name: Token<'a>,
    pub reason: ParseMessageFieldErrorReason,
}

#[derive(Debug)]
pub enum ParseMessageFieldErrorReason {
    ExpectedColon,
    InvalidTypeTag(ParseTypeTagError),
    InvalidTagNumber(ParseTagNumberError),
    ExpectedSemicolon,
}

impl<'a> Error for ParseMessageFieldError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self.reason {
            ExpectedColon => "a colon",
            InvalidTypeTag(e) => return e.info(token),
            InvalidTagNumber(e) => return e.info(token),
            ExpectedSemicolon => "a semicolon",
        };
        P_MESSAGE_FIELD.expected_got_instead(expected, token)
    }
}

/// Parses an optional message field.
///
/// Returns `Ok(message_field, after_semicolon)`.
/// Returns `Ok((None, c))` if the next non-white token is not a symbol.
pub fn parse_message_field(
    c: ParseContext,
) -> ParseResult<Option<MessageFieldTree>, ParseMessageFieldError> {
    let (comments, after_comments) = c.line_comment_block();
    if let (Some(field_name), after_field_name) = after_comments.symbol() {
        let (tree, after_semi) = parse_with_field_name(comments, field_name, after_field_name)?;
        Ok((Some(tree), after_semi))
    } else {
        Ok((None, c))
    }
}

fn parse_with_field_name<'a>(
    comments: Vec<Token<'a>>,
    field_name: Token<'a>,
    c: ParseContext<'a>,
) -> ParseResult<'a, MessageFieldTree<'a>, ParseMessageFieldError<'a>> {
    let (_white, after_white) = c.whitespace();
    if let (Some(_colon), after_colon) = after_white.exact(":") {
        let (type_tag, after_type_tag) = parse_type_tag(after_colon).map_err(|e| {
            e.map(|e| ParseMessageFieldError {
                field_name,
                reason: InvalidTypeTag(e),
            })
        })?;
        let (tag_number, after_tag_number) = parse_tag_number(after_type_tag).map_err(|e| {
            e.map(|e| ParseMessageFieldError {
                field_name,
                reason: InvalidTagNumber(e),
            })
        })?;
        let (_white, after_white) = after_tag_number.whitespace();
        if let (Some(_semi), after_semi) = after_white.exact(";") {
            let tree: MessageFieldTree = MessageFieldTree {
                comments,
                field_name,
                type_tag,
                tag_number,
            };
            Ok((tree, after_semi))
        } else {
            Err(after_white.to_error(ParseMessageFieldError {
                field_name,
                reason: ExpectedSemicolon,
            }))
        }
    } else {
        Err(after_white.to_error(ParseMessageFieldError {
            field_name,
            reason: ExpectedColon,
        }))
    }
}
