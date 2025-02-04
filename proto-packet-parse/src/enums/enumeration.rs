use lex::parse::IntParser;
use lex::{ParseContext, Token};

use proto_packet::io::TagNumber;
use proto_packet_tree::CaseNameRef;

use crate::ParseEnumErrorReason::*;
use crate::{
    expected_got_instead, parse_enum_case, EnumCaseTree, Error, ErrorInfo, ParseEnumCaseError,
    P_ENUM,
};

#[derive(Debug)]
pub struct EnumTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub enum_name: Token<'a>,
    pub cases: Vec<EnumCaseTree<'a>>,
}

impl<'a> EnumTree<'a> {
    //! Cases

    /// Gets the matching `case_name` tokens.
    pub fn case_name_tokens(&self, case_name: CaseNameRef) -> Vec<Token> {
        self.cases
            .iter()
            .map(|case| case.case_name)
            .filter(|name| case_name.as_ref() == name.value())
            .collect()
    }

    /// Gets the matching `tag_number` tokens.
    pub fn tag_number_tokens(&self, tag_number: TagNumber, parser: &IntParser) -> Vec<Token> {
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
pub struct ParseEnumError<'a> {
    pub enum_name: Option<Token<'a>>,
    pub reason: ParseEnumErrorReason<'a>,
}

#[derive(Debug)]
pub enum ParseEnumErrorReason<'a> {
    ExpectedWhitespace,
    ExpectedEnumName,
    ExpectedOpenCurly,
    InvalidEnumCase(ParseEnumCaseError<'a>),
    ExpectedCloseCurly,
}

impl<'a> Error for ParseEnumError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let message: String = match &self.reason {
            ExpectedWhitespace => expected_got_instead("whitespace", token),
            ExpectedEnumName => expected_got_instead("a enum name", token),
            ExpectedOpenCurly => expected_got_instead("an opening curly bracket `{`", token),
            InvalidEnumCase(e) => return e.info(token),
            ExpectedCloseCurly => expected_got_instead("a closing curly bracket `}`", token),
        };
        ErrorInfo {
            code: P_ENUM,
            header: "invalid enum declaration",
            message,
        }
    }
}

/// Parses an optional enum.
///
/// Returns `Ok(enum, after_close_curly)`.
/// Returns `Ok(None, c)` if the next token is not `enum`.
pub fn parse_enum<'a>(
    c: ParseContext<'a>,
) -> lex::Result<'a, Option<EnumTree<'a>>, ParseEnumError<'a>> {
    match c.exact_symbol("enum") {
        (Some(_enum), after_enum) => match after_enum.white_line_comments() {
            (Some(_white), after_white) => match after_white.symbol() {
                (Some(enum_name), after_enum_name) => parse_enum_block(enum_name, after_enum_name),
                (None, _) => Err(after_white.to_error(ParseEnumError {
                    enum_name: None,
                    reason: ExpectedWhitespace,
                })),
            },
            (None, _) => Err(after_enum.to_error(ParseEnumError {
                enum_name: None,
                reason: ExpectedEnumName,
            })),
        },
        _ => Ok((None, c)),
    }
}

fn parse_enum_block<'a>(
    enum_name: Token<'a>,
    c: ParseContext<'a>,
) -> lex::Result<'a, Option<EnumTree<'a>>, ParseEnumError<'a>> {
    let (_white, after_white) = c.white_line_comments();
    match after_white.exact("{") {
        (Some(_open), after_open) => {
            let (cases, after_cases) = parse_enum_cases(enum_name, after_open)?;
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
                _ => Err(after_white.to_error(ParseEnumError {
                    enum_name: Some(enum_name),
                    reason: ExpectedCloseCurly,
                })),
            }
        }
        _ => Err(after_white.to_error(ParseEnumError {
            enum_name: Some(enum_name),
            reason: ExpectedOpenCurly,
        })),
    }
}

fn parse_enum_cases<'a>(
    enum_name: Token<'a>,
    mut c: ParseContext<'a>,
) -> lex::Result<'a, Vec<EnumCaseTree<'a>>, ParseEnumError<'a>> {
    let mut cases: Vec<EnumCaseTree> = Vec::default();
    while let (Some(case), after_case) = parse_enum_case(c).map_err(|e| {
        e.map(|e| ParseEnumError {
            enum_name: Some(enum_name),
            reason: InvalidEnumCase(e),
        })
    })? {
        cases.push(case);
        c = after_case;
    }
    Ok((cases, c))
}
