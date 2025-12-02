use crate::common::tag_number::ParseTagNumberError::*;
use crate::{Error, ErrorInfo, P_TAG_NUMBER};
use lex::{Context, ParseResult, Token};

#[derive(Debug)]
pub enum ParseTagNumberError {
    ExpectedEq,
    ExpectedTagNumber,
}

impl Error for ParseTagNumberError {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match self {
            ExpectedEq => "an '=' starting a tag number",
            ExpectedTagNumber => "a tag number symbol",
        };
        P_TAG_NUMBER.expected_got_instead(expected, token)
    }
}

/// Parses a tag number.
///
/// Returns `Ok(tag_number, after_tag_number)`.
pub fn parse_tag_number(c: Context) -> ParseResult<Token, ParseTagNumberError> {
    let (_white, after_white) = c.whitespace();
    if let (Some(_eq), after_eq) = after_white.exact("=") {
        let (_white, after_white) = after_eq.whitespace();
        if let (Some(tag_number), after_tag_number) = after_white.symbol() {
            Ok((tag_number, after_tag_number))
        } else {
            Err(after_white.to_error(ExpectedTagNumber))
        }
    } else {
        Err(after_white.to_error(ExpectedEq))
    }
}
