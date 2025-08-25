use crate::ParseVariantErrorReason::*;
use crate::{
    parse_variant_case, Error, ErrorInfo, ParseVariantCaseError, VariantCaseTree, P_VARIANT,
};
use lex::parse::IntParser;
use lex::{ParseContext, ParseResult, Token};
use proto_packet::io::TagNumber;
use proto_packet_tree::CaseNameRef;

#[derive(Debug)]
pub struct VariantTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub variant_name: Token<'a>,
    pub cases: Vec<VariantCaseTree<'a>>,
}

impl<'a> VariantTree<'a> {
    //! Cases

    /// Gets the matching `case_name` tokens.
    pub fn case_name_tokens(&self, case_name: CaseNameRef) -> Vec<Token<'_>> {
        self.cases
            .iter()
            .map(|case| case.case_name)
            .filter(|name| case_name.as_ref() == name.value())
            .collect()
    }

    /// Gets the matching `tag_number` tokens.
    pub fn tag_number_tokens(&self, tag_number: TagNumber, parser: &IntParser) -> Vec<Token<'_>> {
        self.cases
            .iter()
            .map(|case| case.tag_number)
            .flat_map(|number| parser.parse_u32(number.value()).ok().map(|n| (number, n)))
            .flat_map(|(token, number)| TagNumber::new(number).map(|n| (token, n)))
            .filter(|(_, number)| tag_number == *number)
            .map(|(token, _)| token)
            .collect()
    }
}

#[derive(Debug)]
pub struct ParseVariantError<'a> {
    pub variant_name: Option<Token<'a>>,
    pub reason: ParseVariantErrorReason<'a>,
}

#[derive(Debug)]
pub enum ParseVariantErrorReason<'a> {
    ExpectedWhitespace,
    ExpectedVariantName,
    ExpectedOpenCurly,
    InvalidVariantCase(ParseVariantCaseError<'a>),
    ExpectedCloseCurly,
}

impl<'a> Error for ParseVariantError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self.reason {
            ExpectedWhitespace => "whitespace",
            ExpectedVariantName => "a variant name",
            ExpectedOpenCurly => "an opening curly bracket '{'",
            InvalidVariantCase(e) => return e.info(token),
            ExpectedCloseCurly => "a closing curly bracket '}'",
        };
        P_VARIANT.expected_got_instead(expected, token)
    }
}

/// Parses an optional variant.
///
/// Returns `Ok(variant, after_close_curly)`.
/// Returns `Ok(None, c)` if the next token is not `variant`.
pub fn parse_variant(c: ParseContext) -> ParseResult<Option<VariantTree>, ParseVariantError> {
    match c.exact_symbol("variant") {
        (Some(_variant), after_variant) => match after_variant.white_line_comments() {
            (Some(_white), after_white) => match after_white.symbol() {
                (Some(variant_name), after_variant_name) => {
                    parse_variant_block(variant_name, after_variant_name)
                }
                (None, _) => Err(after_white.to_error(ParseVariantError {
                    variant_name: None,
                    reason: ExpectedVariantName,
                })),
            },
            (None, _) => Err(after_variant.to_error(ParseVariantError {
                variant_name: None,
                reason: ExpectedWhitespace,
            })),
        },
        _ => Ok((None, c)),
    }
}

fn parse_variant_block<'a>(
    variant_name: Token<'a>,
    c: ParseContext<'a>,
) -> ParseResult<'a, Option<VariantTree<'a>>, ParseVariantError<'a>> {
    let (_white, after_white) = c.white_line_comments();
    match after_white.exact("{") {
        (Some(_open), after_open) => {
            let (cases, after_cases) = parse_variant_cases(variant_name, after_open)?;
            let (_white, after_white) = after_cases.white_line_comments();
            match after_white.exact("}") {
                (Some(_close), after_close) => {
                    let tree: VariantTree = VariantTree {
                        comments: vec![],
                        variant_name,
                        cases,
                    };
                    Ok((Some(tree), after_close))
                }
                _ => Err(after_white.to_error(ParseVariantError {
                    variant_name: Some(variant_name),
                    reason: ExpectedCloseCurly,
                })),
            }
        }
        _ => Err(after_white.to_error(ParseVariantError {
            variant_name: Some(variant_name),
            reason: ExpectedOpenCurly,
        })),
    }
}

fn parse_variant_cases<'a>(
    variant_name: Token<'a>,
    mut c: ParseContext<'a>,
) -> ParseResult<'a, Vec<VariantCaseTree<'a>>, ParseVariantError<'a>> {
    let mut cases: Vec<VariantCaseTree> = Vec::default();
    while let (Some(case), after_case) = parse_variant_case(c).map_err(|e| {
        e.map(|e| ParseVariantError {
            variant_name: Some(variant_name),
            reason: InvalidVariantCase(e),
        })
    })? {
        cases.push(case);
        c = after_case;
    }
    Ok((cases, c))
}
