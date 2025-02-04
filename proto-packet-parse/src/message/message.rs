use lex::parse::IntParser;
use lex::{ParseContext, Token};

use proto_packet::io::TagNumber;
use proto_packet_tree::FieldNameRef;

use crate::ParseMessageErrorReason::*;
use crate::{
    expected_got_instead, parse_message_field, Error, ErrorInfo, MessageFieldTree,
    ParseMessageFieldError, P_MESSAGE,
};

#[derive(Debug)]
pub struct MessageTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub message_name: Token<'a>,
    pub fields: Vec<MessageFieldTree<'a>>,
}

impl<'a> MessageTree<'a> {
    //! Fields

    /// Gets the matching `field_name` tokens.
    pub fn field_name_tokens(&self, field_name: FieldNameRef) -> Vec<Token> {
        self.fields
            .iter()
            .map(|field| field.field_name)
            .filter(|name| field_name.as_ref() == name.value())
            .collect()
    }

    /// Gets the matching `tag_number` tokens.
    pub fn tag_number_tokens(&self, tag_number: TagNumber, parser: &IntParser) -> Vec<Token> {
        self.fields
            .iter()
            .flat_map(|field| field.tag_number)
            .flat_map(|number| parser.parse_u32(number.value()).ok().map(|n| (number, n)))
            .flat_map(|(token, number)| TagNumber::new(number).map(|n| (token, n)))
            .filter(|(_, number)| tag_number == *number)
            .map(|(token, _)| token)
            .collect()
    }
}

#[derive(Debug)]
pub struct ParseMessageError<'a> {
    pub message_name: Option<Token<'a>>,
    pub reason: ParseMessageErrorReason<'a>,
}

#[derive(Debug)]
pub enum ParseMessageErrorReason<'a> {
    ExpectedWhitespace,
    ExpectedMessageName,
    ExpectedOpenCurly,
    InvalidMessageField(ParseMessageFieldError<'a>),
    ExpectedCloseCurly,
}

impl<'a> Error for ParseMessageError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let message: String = match &self.reason {
            ExpectedWhitespace => expected_got_instead("whitespace", token),
            ExpectedMessageName => expected_got_instead("a message name", token),
            ExpectedOpenCurly => expected_got_instead("an opening curly bracket `{`", token),
            InvalidMessageField(e) => return e.info(token),
            ExpectedCloseCurly => expected_got_instead("a closing curly bracket `}`", token),
        };
        ErrorInfo {
            code: P_MESSAGE,
            header: "invalid message declaration",
            message,
        }
    }
}

/// Parses an optional message.
///
/// Returns `Ok(message, after_close_curly)`.
/// Returns `Ok(None, c)` if the next token is not `message`.
pub fn parse_message(c: ParseContext) -> lex::Result<Option<MessageTree>, ParseMessageError> {
    match c.exact_symbol("message") {
        (Some(_message), after_message) => match after_message.white_line_comments() {
            (Some(_white), after_white) => match after_white.symbol() {
                (Some(message_name), after_message_name) => {
                    parse_message_block(message_name, after_message_name)
                }
                (None, _) => Err(after_white.to_error(ParseMessageError {
                    message_name: None,
                    reason: ExpectedWhitespace,
                })),
            },
            (None, _) => Err(after_message.to_error(ParseMessageError {
                message_name: None,
                reason: ExpectedMessageName,
            })),
        },
        _ => Ok((None, c)),
    }
}

fn parse_message_block<'a>(
    message_name: Token<'a>,
    c: ParseContext<'a>,
) -> lex::Result<'a, Option<MessageTree<'a>>, ParseMessageError<'a>> {
    let (_white, after_white) = c.white_line_comments();
    match after_white.exact("{") {
        (Some(_open), after_open) => {
            let (fields, after_fields) = parse_message_fields(message_name, after_open)?;
            let (_white, after_white) = after_fields.white_line_comments();
            match after_white.exact("}") {
                (Some(_close), after_close) => {
                    let tree: MessageTree = MessageTree {
                        comments: vec![],
                        message_name,
                        fields,
                    };
                    Ok((Some(tree), after_close))
                }
                _ => Err(after_white.to_error(ParseMessageError {
                    message_name: Some(message_name),
                    reason: ExpectedCloseCurly,
                })),
            }
        }
        _ => Err(after_white.to_error(ParseMessageError {
            message_name: Some(message_name),
            reason: ExpectedOpenCurly,
        })),
    }
}

fn parse_message_fields<'a>(
    message_name: Token<'a>,
    mut c: ParseContext<'a>,
) -> lex::Result<'a, Vec<MessageFieldTree<'a>>, ParseMessageError<'a>> {
    let mut fields: Vec<MessageFieldTree> = Vec::default();
    while let (Some(field), after_field) = parse_message_field(c).map_err(|e| {
        e.map(|e| ParseMessageError {
            message_name: Some(message_name),
            reason: InvalidMessageField(e),
        })
    })? {
        fields.push(field);
        c = after_field;
    }
    Ok((fields, c))
}
