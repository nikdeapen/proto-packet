use lex::{ParseContext, Token};

use crate::ParseEnumCaseErrorReason::*;
use crate::{
    expected_got_instead, parse_tag_number, Error, ErrorInfo, ParseTagNumberError,
    ParseTypeTagError, P_ENUM_CASE,
};

#[derive(Debug)]
pub struct EnumCaseTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub case_name: Token<'a>,
    pub tag_number: Token<'a>,
}

#[derive(Debug)]
pub struct ParseEnumCaseError<'a> {
    pub case_name: Token<'a>,
    pub reason: ParseEnumCaseErrorReason,
}

#[derive(Debug)]
pub enum ParseEnumCaseErrorReason {
    InvalidTypeTag(ParseTypeTagError),
    InvalidTagNumber(ParseTagNumberError),
    ExpectedSemicolon,
}

impl<'a> Error for ParseEnumCaseError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let message: String = match &self.reason {
            InvalidTypeTag(e) => return e.info(token),
            InvalidTagNumber(e) => return e.info(token),
            ExpectedSemicolon => expected_got_instead("a semicolon", token),
        };
        ErrorInfo {
            code: P_ENUM_CASE,
            header: "invalid enum case",
            message,
        }
    }
}

/// Parses an optional enum case.
///
/// Returns `Ok(enum_case, after_semicolon)`.
/// Returns `Ok((None, c))` if the next non-white token is not a symbol.
pub fn parse_enum_case(c: ParseContext) -> lex::Result<Option<EnumCaseTree>, ParseEnumCaseError> {
    let (comments, after_comments) = c.line_comment_block();
    if let (Some(case_name), after_name) = after_comments.symbol() {
        let (tag_number, after_tag_number) = parse_tag_number(after_name).map_err(|e| {
            e.map(|e| ParseEnumCaseError {
                case_name,
                reason: InvalidTagNumber(e),
            })
        })?;
        if let (Some(_semi), after_semi) = after_tag_number.white_mark(';') {
            let tree: EnumCaseTree = EnumCaseTree {
                comments,
                case_name,
                tag_number,
            };
            Ok((Some(tree), after_semi))
        } else {
            Err(after_tag_number.to_error(ParseEnumCaseError {
                case_name,
                reason: ExpectedSemicolon,
            }))
        }
    } else {
        Ok((None, c))
    }
}
