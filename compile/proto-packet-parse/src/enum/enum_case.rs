use crate::ParseEnumCaseError::*;
use crate::{parse_tag_number, Error, ErrorInfo, ParseTagNumberError, P_ENUM_CASE};
use lex::{Context, ParseResult, Token};

#[derive(Debug)]
pub struct EnumCaseTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub case_name: Token<'a>,
    pub tag_number: Token<'a>,
}

#[derive(Debug)]
pub enum ParseEnumCaseError {
    InvalidTagNumber(ParseTagNumberError),
    ExpectedSemicolon,
}

impl Error for ParseEnumCaseError {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self {
            InvalidTagNumber(e) => return e.info(token),
            ExpectedSemicolon => "a semicolon",
        };
        P_ENUM_CASE.expected_got_instead(expected, token)
    }
}

/// Parses an optional enum case.
///
/// Returns `Ok(enum_case, after_semicolon)`.
/// Returns `Ok((None, c))` if the next non-white token is not a symbol.
pub fn parse_enum_case(c: Context) -> ParseResult<Option<EnumCaseTree>, ParseEnumCaseError> {
    let (comments, after_comments) = c.line_comment_block();
    if let (Some(case_name), after_name) = after_comments.symbol() {
        let (_white, after_white) = after_name.whitespace();
        let (tag_number, after_tag_number) =
            parse_tag_number(after_white).map_err(|e| e.map(|e| InvalidTagNumber(e)))?;
        if let (Some(_semi), after_semi) = after_tag_number.exact(";") {
            let tree: EnumCaseTree = EnumCaseTree {
                comments,
                case_name,
                tag_number,
            };
            Ok((Some(tree), after_semi))
        } else {
            Err(after_tag_number.to_error(ExpectedSemicolon))
        }
    } else {
        Ok((None, c))
    }
}
