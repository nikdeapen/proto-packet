use crate::ParseEnumError::*;
use crate::{parse_enum_case, EnumCaseTree, Error, ErrorInfo, ParseEnumCaseError, P_ENUM};
use lex::parse::IntParser;
use lex::{Context, ParseResult, Token};
use proto_packet::io::TagNumber;
use proto_packet_tree::CaseNameRef;

#[derive(Debug)]
pub struct EnumTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub enum_name: Token<'a>,
    pub cases: Vec<EnumCaseTree<'a>>,
}

impl<'a> EnumTree<'a> {
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
pub enum ParseEnumError {
    ExpectedWhitespace,
    ExpectedEnumName,
    ExpectedOpenCurly,
    InvalidEnumCase(ParseEnumCaseError),
    ExpectedCloseCurly,
}

impl Error for ParseEnumError {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self {
            ExpectedWhitespace => "whitespace",
            ExpectedEnumName => "an enum name symbol",
            ExpectedOpenCurly => "an opening curly bracket '{'",
            InvalidEnumCase(e) => return e.info(token),
            ExpectedCloseCurly => "a closing curly bracket '}'",
        };
        P_ENUM.expected_got_instead(expected, token)
    }
}

/// Parses an optional enum.
///
/// Returns `Ok(enum, after_close_curly)`.
/// Returns `Ok(None, c)` if the next token is not `enum`.
pub fn parse_enum(c: Context) -> ParseResult<Option<EnumTree>, ParseEnumError> {
    match c.exact_symbol("enum") {
        (Some(_enum), after_enum) => match after_enum.white_line_comments() {
            (Some(_white), after_white) => match after_white.symbol() {
                (Some(enum_name), after_enum_name) => parse_enum_block(enum_name, after_enum_name),
                (None, _) => Err(after_white.to_error(ExpectedEnumName)),
            },
            (None, _) => Err(after_enum.to_error(ExpectedWhitespace)),
        },
        _ => Ok((None, c)),
    }
}

fn parse_enum_block<'a>(
    enum_name: Token<'a>,
    c: Context<'a>,
) -> ParseResult<'a, Option<EnumTree<'a>>, ParseEnumError> {
    let (_white, after_white) = c.white_line_comments();
    match after_white.exact("{") {
        (Some(_open), after_open) => {
            let (cases, after_cases) = parse_enum_cases(after_open)?;
            let (_white, after_white) = after_cases.white_line_comments();
            match after_white.exact("}") {
                (Some(_close), after_close) => {
                    let tree: EnumTree = EnumTree {
                        comments: vec![],
                        enum_name,
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

fn parse_enum_cases(mut c: Context) -> ParseResult<Vec<EnumCaseTree>, ParseEnumError> {
    let mut cases: Vec<EnumCaseTree> = Vec::default();
    while let (Some(case), after_case) =
        parse_enum_case(c).map_err(|e| e.map(|e| InvalidEnumCase(e)))?
    {
        cases.push(case);
        c = after_case;
    }
    Ok((cases, c))
}
