use lex::{ParseContext, Token};

use crate::ParseMessageFieldErrorReason::*;
use crate::{
    expected_got_instead, parse_tag_number_optional, parse_var, Error, ErrorInfo,
    ParseTagNumberError, ParseTypeTagError, ParseVarErrorReason, TypeTagTree, P_MESSAGE_FIELD,
};

#[derive(Debug)]
pub struct MessageFieldTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub field_name: Token<'a>,
    pub type_tag: TypeTagTree<'a>,
    pub tag_number: Option<Token<'a>>,
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
        let message: String = match &self.reason {
            ExpectedColon => expected_got_instead("a colon", token),
            InvalidTypeTag(e) => return e.info(token),
            InvalidTagNumber(e) => return e.info(token),
            ExpectedSemicolon => expected_got_instead("a semicolon", token),
        };
        ErrorInfo {
            code: P_MESSAGE_FIELD,
            header: "invalid message field",
            message,
        }
    }
}

/// Parses an optional message field.
///
/// Returns `Ok(message_field, after_semicolon)`.
/// Returns `Ok((None, c))` if the next non-white token is not a symbol.
pub fn parse_message_field(
    c: ParseContext,
) -> lex::Result<Option<MessageFieldTree>, ParseMessageFieldError> {
    match parse_var(c) {
        Ok((Some(var), after_var)) => match parse_tag_number_optional(after_var) {
            Ok((tag_number, after_tag_number)) => {
                let tree: MessageFieldTree = MessageFieldTree {
                    comments: var.comments,
                    field_name: var.name,
                    type_tag: var.type_tag,
                    tag_number,
                };
                let (_white, after_white) = after_tag_number.white_line_comments();
                match after_white.exact(";") {
                    (Some(_semi), after_semi) => Ok((Some(tree), after_semi)),
                    (None, _) => Err(after_white.to_error(ParseMessageFieldError {
                        field_name: var.name,
                        reason: ExpectedSemicolon,
                    })),
                }
            }
            Err(e) => Err(e.map(|e| ParseMessageFieldError {
                field_name: var.name,
                reason: InvalidTagNumber(e),
            })),
        },
        Ok((None, c)) => Ok((None, c)),
        Err(e) => Err(e.map(|e| ParseMessageFieldError {
            field_name: e.name,
            reason: match e.reason {
                ParseVarErrorReason::ExpectedColon => ExpectedColon,
                ParseVarErrorReason::InvalidTypeTag(e) => InvalidTypeTag(e),
            },
        })),
    }
}
