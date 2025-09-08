use crate::ParseServiceCallErrorReason::*;
use crate::{parse_type_tag, Error, ErrorInfo, ParseTypeTagError, TypeTagTree, P_SERVICE_CALL};
use lex::{ParseContext, ParseResult, Token};

#[derive(Debug)]
pub struct ServiceCallTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub service_call_name: Token<'a>,
    pub input_type: TypeTagTree<'a>,
    pub output_type: TypeTagTree<'a>,
}

#[derive(Debug)]
pub struct ParseServiceCallError<'a> {
    pub service_call_name: Token<'a>,
    pub reason: ParseServiceCallErrorReason,
}

#[derive(Debug)]
pub enum ParseServiceCallErrorReason {
    ExpectedColon,
    InvalidInputType(ParseTypeTagError),
    ExpectedReturns,
    InvalidOutputType(ParseTypeTagError),
    ExpectedSemicolon,
}

impl<'a> Error for ParseServiceCallError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self.reason {
            ExpectedColon => "a colon",
            InvalidInputType(e) => return e.info(token),
            ExpectedReturns => "a 'returns' token",
            InvalidOutputType(e) => return e.info(token),
            ExpectedSemicolon => "a semicolon",
        };
        P_SERVICE_CALL.expected_got_instead(expected, token)
    }
}

/// Parses an optional service call.
///
/// Returns `Ok(service_call, after_semicolon)`.
/// Returns `Ok((None, c))` if the next non-white token is not a symbol.
pub fn parse_service_call(
    c: ParseContext,
) -> ParseResult<Option<ServiceCallTree>, ParseServiceCallError> {
    let (comments, after_comments) = c.line_comment_block();
    if let (Some(call_name), after_call_name) = after_comments.symbol() {
        let (tree, after_semi) =
            parse_with_service_call_name(comments, call_name, after_call_name)?;
        Ok((Some(tree), after_semi))
    } else {
        Ok((None, c))
    }
}

fn parse_with_service_call_name<'a>(
    comments: Vec<Token<'a>>,
    service_call_name: Token<'a>,
    c: ParseContext<'a>,
) -> ParseResult<'a, ServiceCallTree<'a>, ParseServiceCallError<'a>> {
    let (_white, after_white) = c.whitespace();
    if let (Some(_colon), after_colon) = after_white.exact(":") {
        let (input_type, after_input_type) = parse_type_tag(after_colon).map_err(|e| {
            e.map(|e| ParseServiceCallError {
                service_call_name,
                reason: InvalidInputType(e),
            })
        })?;
        let (_white, after_white) = after_input_type.whitespace();
        if let (Some(_returns), after_returns) = after_white.exact("returns") {
            let (output_type, after_output_type) = parse_type_tag(after_returns).map_err(|e| {
                e.map(|e| ParseServiceCallError {
                    service_call_name,
                    reason: InvalidOutputType(e),
                })
            })?;
            let (_white, after_white) = after_output_type.whitespace();
            if let (Some(_semi), after_semi) = after_white.exact(";") {
                let tree: ServiceCallTree = ServiceCallTree {
                    comments,
                    service_call_name,
                    input_type,
                    output_type,
                };
                Ok((tree, after_semi))
            } else {
                Err(after_white.to_error(ParseServiceCallError {
                    service_call_name,
                    reason: ExpectedSemicolon,
                }))
            }
        } else {
            Err(after_white.to_error(ParseServiceCallError {
                service_call_name,
                reason: ExpectedReturns,
            }))
        }
    } else {
        Err(after_white.to_error(ParseServiceCallError {
            service_call_name,
            reason: ExpectedColon,
        }))
    }
}
