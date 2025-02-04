use lex::{ParseContext, Token};

use crate::ParseVariantCaseErrorReason::*;
use crate::{
    expected_got_instead, parse_tag_number_optional, parse_var, Error, ErrorInfo,
    ParseTagNumberError, ParseTypeTagError, ParseVarErrorReason, TypeTagTree,
};

#[derive(Debug)]
pub struct VariantCaseTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub case_name: Token<'a>,
    pub type_tag: TypeTagTree<'a>,
    pub tag_number: Token<'a>,
}

#[derive(Debug)]
pub struct ParseVariantCaseError<'a> {
    pub case_name: Token<'a>,
    pub reason: ParseVariantCaseErrorReason,
}

#[derive(Debug)]
pub enum ParseVariantCaseErrorReason {
    ExpectedColon,
    MissingTagNumber,
    InvalidTypeTag(ParseTypeTagError),
    InvalidTagNumber(ParseTagNumberError),
    ExpectedSemicolon,
}

impl ParseVariantCaseErrorReason {
    pub fn to_error(self, case_name: Token) -> ParseVariantCaseError {
        ParseVariantCaseError {
            case_name,
            reason: self,
        }
    }
}

impl<'a> Error for ParseVariantCaseError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let message: String = match &self.reason {
            ExpectedColon => expected_got_instead("a colon", token),
            MissingTagNumber => expected_got_instead("an '= [tag_number]'", token),
            InvalidTypeTag(e) => return e.info(token),
            InvalidTagNumber(e) => return e.info(token),
            ExpectedSemicolon => expected_got_instead("a semicolon", token),
        };
        ErrorInfo {
            code: "P_VARIANT_CASE",
            header: "invalid variant case",
            message,
        }
    }
}

/// Parses an optional variant case.
///
/// Returns `Ok(variant_case, after_semicolon)`.
/// Returns `Ok((None, c))` if the next non-white token is not a symbol.
pub fn parse_variant_case(
    c: ParseContext,
) -> lex::Result<Option<VariantCaseTree>, ParseVariantCaseError> {
    match parse_var(c) {
        Ok((Some(var), after_var)) => match parse_tag_number_optional(after_var) {
            Ok((tag_number, after_tag_number)) => {
                let tag_number: Token = if let Some(tag_number) = tag_number {
                    tag_number
                } else {
                    return Err(after_var.to_error(MissingTagNumber.to_error(var.name)));
                };
                let tree: VariantCaseTree = VariantCaseTree {
                    comments: var.comments,
                    case_name: var.name,
                    type_tag: var.type_tag,
                    tag_number,
                };
                let (_white, after_white) = after_tag_number.white_line_comments();
                match after_white.exact(";") {
                    (Some(_semi), after_semi) => Ok((Some(tree), after_semi)),
                    (None, _) => Err(after_white.to_error(ParseVariantCaseError {
                        case_name: var.name,
                        reason: ExpectedSemicolon,
                    })),
                }
            }
            Err(e) => Err(e.map(|e| ParseVariantCaseError {
                case_name: var.name,
                reason: InvalidTagNumber(e),
            })),
        },
        Ok((None, c)) => Ok((None, c)),
        Err(e) => Err(e.map(|e| ParseVariantCaseError {
            case_name: e.name,
            reason: match e.reason {
                ParseVarErrorReason::ExpectedColon => ExpectedColon,
                ParseVarErrorReason::InvalidTypeTag(e) => InvalidTypeTag(e),
            },
        })),
    }
}
