use lex::{ParseContext, Token};

use crate::common::tag_number::ParseTagNumberError::*;
use crate::{expected_got_instead, Error, ErrorInfo, P_TAG_NUMBER};

#[derive(Debug)]
pub enum ParseTagNumberError {
    ExpectedTagNumberEq,
    ExpectedTagNumberSymbol,
}

impl Error for ParseTagNumberError {
    fn info(&self, token: &str) -> ErrorInfo {
        match self {
            ExpectedTagNumberEq => ErrorInfo {
                code: P_TAG_NUMBER,
                header: "expected a tag number",
                message: expected_got_instead("an '=' starting a tag number", token),
            },
            ExpectedTagNumberSymbol => ErrorInfo {
                code: P_TAG_NUMBER,
                header: "expected a tag number symbol",
                message: expected_got_instead("a tag number symbol", token),
            },
        }
    }
}

/// Parses a tag number.
///
/// Returns `Ok(tag_number, after_tag_number)`.
pub fn parse_tag_number(c: ParseContext) -> lex::Result<Token, ParseTagNumberError> {
    let (_white, after_white) = c.white_line_comments();
    if let (Some(tag_number), after_tag_number) = parse_tag_number_optional(after_white)? {
        Ok((tag_number, after_tag_number))
    } else {
        Err(after_white.to_error(ExpectedTagNumberEq))
    }
}

/// Parses an optional tag number.
///
/// Returns `Ok(Some(tag_number), after tag_number)`.
/// Returns `Ok(None, c)` if the next non-white token is not `=`.
pub fn parse_tag_number_optional(
    c: ParseContext,
) -> lex::Result<Option<Token>, ParseTagNumberError> {
    let (_white, after_white) = c.white_line_comments();
    if let (Some(_eq), after_eq) = after_white.mark('=') {
        let (_white, after_white) = after_eq.white_line_comments();
        if let (Some(tag_number), after_tag_number) = after_white.symbol() {
            Ok((Some(tag_number), after_tag_number))
        } else {
            Err(after_white.to_error(ExpectedTagNumberSymbol))
        }
    } else {
        Ok((None, c))
    }
}
