use crate::ParseServiceErrorReason::*;
use crate::{
    parse_service_call, Error, ErrorInfo, ParseServiceCallError, ServiceCallTree, P_SERVICE,
};
use lex::{ParseContext, ParseResult, Token};
use proto_packet_tree::ServiceCallNameRef;

#[derive(Debug)]
pub struct ServiceTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub service_name: Token<'a>,
    pub service_calls: Vec<ServiceCallTree<'a>>,
}

impl<'a> ServiceTree<'a> {
    //! Service Calls

    /// Gets the matching `service_call_name` tokens.
    pub fn service_call_name_tokens(
        &self,
        service_call_name: ServiceCallNameRef,
    ) -> Vec<Token<'_>> {
        self.service_calls
            .iter()
            .map(|service_call| service_call.service_call_name)
            .filter(|name| service_call_name.as_ref() == name.value())
            .collect()
    }
}

#[derive(Debug)]
pub struct ParseServiceError<'a> {
    pub service_name: Option<Token<'a>>,
    pub reason: ParseServiceErrorReason<'a>,
}

#[derive(Debug)]
pub enum ParseServiceErrorReason<'a> {
    ExpectedWhitespace,
    ExpectedServiceName,
    ExpectedOpenCurly,
    InvalidServiceCall(ParseServiceCallError<'a>),
    ExpectedCloseCurly,
}

impl<'a> Error for ParseServiceError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self.reason {
            ExpectedWhitespace => "whitespace",
            ExpectedServiceName => "a service name",
            ExpectedOpenCurly => "an opening curly bracket '{'",
            InvalidServiceCall(e) => return e.info(token),
            ExpectedCloseCurly => "a closing curly bracket '}'",
        };
        P_SERVICE.expected_got_instead(expected, token)
    }
}

/// Parses an optional service.
///
/// Returns `Ok(service, after_close_curly)`.
/// Returns `Ok(None, c)` if the next token is not `service`.
pub fn parse_service(
    c: ParseContext<'_>,
) -> ParseResult<'_, Option<ServiceTree<'_>>, ParseServiceError<'_>> {
    match c.exact_symbol("service") {
        (Some(_service), after_service) => match after_service.whitespace() {
            (Some(_white), after_white) => match after_white.symbol() {
                (Some(service_name), after_service_name) => {
                    parse_service_block(service_name, after_service_name)
                }
                (None, _) => Err(after_white.to_error(ParseServiceError {
                    service_name: None,
                    reason: ExpectedServiceName,
                })),
            },
            (None, _) => Err(after_service.to_error(ParseServiceError {
                service_name: None,
                reason: ExpectedWhitespace,
            })),
        },
        _ => Ok((None, c)),
    }
}

fn parse_service_block<'a>(
    service_name: Token<'a>,
    c: ParseContext<'a>,
) -> ParseResult<'a, Option<ServiceTree<'a>>, ParseServiceError<'a>> {
    let (_white, after_white) = c.whitespace();
    match after_white.exact("{") {
        (Some(_open), after_open) => {
            let (service_calls, after_service_calls) =
                parse_service_service_calls(service_name, after_open)?;
            let (_white, after_white) = after_service_calls.white_line_comments();
            match after_white.exact("}") {
                (Some(_close), after_close) => {
                    let tree: ServiceTree = ServiceTree {
                        comments: vec![],
                        service_name,
                        service_calls,
                    };
                    Ok((Some(tree), after_close))
                }
                _ => Err(after_white.to_error(ParseServiceError {
                    service_name: Some(service_name),
                    reason: ExpectedCloseCurly,
                })),
            }
        }
        _ => Err(after_white.to_error(ParseServiceError {
            service_name: Some(service_name),
            reason: ExpectedOpenCurly,
        })),
    }
}

fn parse_service_service_calls<'a>(
    service_name: Token<'a>,
    mut c: ParseContext<'a>,
) -> ParseResult<'a, Vec<ServiceCallTree<'a>>, ParseServiceError<'a>> {
    let mut service_calls: Vec<ServiceCallTree> = Vec::default();
    while let (Some(service_call), after_service_call) = parse_service_call(c).map_err(|e| {
        e.map(|e| ParseServiceError {
            service_name: Some(service_name),
            reason: InvalidServiceCall(e),
        })
    })? {
        service_calls.push(service_call);
        c = after_service_call;
    }
    Ok((service_calls, c))
}
