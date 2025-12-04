use crate::ParseVariantError::*;
use crate::{
    parse_variant_case, Error, ErrorInfo, ParseVariantCaseError, VariantCaseTree, P_VARIANT,
};
use lex::parse::IntParser;
use lex::{Context, ParseResult, Token};
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
pub enum ParseVariantError {
    ExpectedWhitespace,
    ExpectedVariantName,
    ExpectedOpenCurly,
    InvalidVariantCase(ParseVariantCaseError),
    ExpectedCloseCurly,
}

impl Error for ParseVariantError {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self {
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
pub fn parse_variant(c: Context) -> ParseResult<Option<VariantTree>, ParseVariantError> {
    match c.exact_symbol("variant") {
        (Some(_variant), after_variant) => match after_variant.white_line_comments() {
            (Some(_white), after_white) => match after_white.symbol() {
                (Some(variant_name), after_variant_name) => {
                    let (_white, after_white) = after_variant_name.white_line_comments();
                    match after_white.exact("{") {
                        (Some(_open), after_open) => {
                            let (cases, after_cases) = parse_variant_cases(after_open)?;
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
                                _ => Err(after_white.to_error(ExpectedCloseCurly)),
                            }
                        }
                        _ => Err(after_white.to_error(ExpectedOpenCurly)),
                    }
                }
                (None, _) => Err(after_white.to_error(ExpectedVariantName)),
            },
            (None, _) => Err(after_variant.to_error(ExpectedWhitespace)),
        },
        _ => Ok((None, c)),
    }
}

fn parse_variant_cases(mut c: Context) -> ParseResult<Vec<VariantCaseTree>, ParseVariantError> {
    let mut cases: Vec<VariantCaseTree> = Vec::default();
    while let (Some(case), after_case) =
        parse_variant_case(c).map_err(|e| e.map(|e| InvalidVariantCase(e)))?
    {
        cases.push(case);
        c = after_case;
    }
    Ok((cases, c))
}
