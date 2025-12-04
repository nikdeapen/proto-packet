use crate::ParseVariantCaseError::*;
use crate::{
    parse_tag_number, parse_type_tag, Error, ErrorInfo, ParseTagNumberError, ParseTypeTagError,
    TypeTagTree, P_VARIANT_CASE,
};
use lex::{Context, ParseResult, Token};

#[derive(Debug)]
pub struct VariantCaseTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub case_name: Token<'a>,
    pub type_tag: TypeTagTree<'a>,
    pub tag_number: Token<'a>,
}

#[derive(Debug)]
pub enum ParseVariantCaseError {
    ExpectedColon,
    InvalidTypeTag(ParseTypeTagError),
    MissingTagNumber,
    InvalidTagNumber(ParseTagNumberError),
    ExpectedSemicolon,
}

impl Error for ParseVariantCaseError {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self {
            ExpectedColon => "a colon",
            InvalidTypeTag(e) => return e.info(token),
            MissingTagNumber => "a tag number",
            InvalidTagNumber(e) => return e.info(token),
            ExpectedSemicolon => "a semicolon",
        };
        P_VARIANT_CASE.expected_got_instead(expected, token)
    }
}

/// Parses an optional variant case.
///
/// Returns `Ok(variant_case, after_semicolon)`.
/// Returns `Ok((None, c))` if the next non-white token is not a symbol.
pub fn parse_variant_case(
    c: Context,
) -> ParseResult<Option<VariantCaseTree>, ParseVariantCaseError> {
    let (comments, after_comments) = c.line_comment_block();
    if let (Some(case_name), after_case_name) = after_comments.symbol() {
        let (_white, after_white) = after_case_name.whitespace();
        if let (Some(_colon), after_colon) = after_white.exact(":") {
            let (_white, after_white) = after_colon.whitespace();
            let (type_tag, after_type_tag) =
                parse_type_tag(after_white).map_err(|e| e.map(|e| InvalidTypeTag(e)))?;
            let (tag_number, after_tag_number) =
                parse_tag_number(after_type_tag).map_err(|e| e.map(|e| InvalidTagNumber(e)))?;
            let tree: VariantCaseTree = VariantCaseTree {
                comments,
                case_name,
                type_tag,
                tag_number,
            };
            let (_white, after_white) = after_tag_number.white_line_comments();
            match after_white.exact(";") {
                (Some(_semi), after_semi) => Ok((Some(tree), after_semi)),
                (None, _) => Err(after_white.to_error(ExpectedSemicolon)),
            }
        } else {
            Err(after_white.to_error(ExpectedColon))
        }
    } else {
        Ok((None, c))
    }
}
