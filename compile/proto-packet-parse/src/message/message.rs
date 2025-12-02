use crate::ParseMessageError::*;
use crate::{
    parse_message_field, Error, ErrorInfo, MessageFieldTree, ParseMessageFieldError, P_MESSAGE,
};
use lex::parse::IntParser;
use lex::{Context, ParseResult, Token};
use proto_packet::io::TagNumber;
use proto_packet_tree::FieldNameRef;

#[derive(Debug)]
pub struct MessageTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub message_name: Token<'a>,
    pub fields: Vec<MessageFieldTree<'a>>,
}

impl<'a> MessageTree<'a> {
    //! Fields

    /// Gets the matching `field_name` tokens.
    pub fn field_name_tokens(&self, field_name: FieldNameRef) -> Vec<Token<'_>> {
        self.fields
            .iter()
            .map(|field| field.field_name)
            .filter(|name| field_name.as_ref() == name.value())
            .collect()
    }

    /// Gets the matching `tag_number` tokens.
    pub fn tag_number_tokens(&self, tag_number: TagNumber, parser: &IntParser) -> Vec<Token<'_>> {
        self.fields
            .iter()
            .map(|field| field.tag_number)
            .flat_map(|number| parser.parse_u32(number.value()).ok().map(|n| (number, n)))
            .flat_map(|(token, number)| TagNumber::new(number).map(|n| (token, n)))
            .filter(|(_, number)| tag_number == *number)
            .map(|(token, _)| token)
            .collect()
    }
}

#[derive(Debug)]
pub enum ParseMessageError {
    ExpectedWhitespace,
    ExpectedMessageName,
    ExpectedOpenCurly,
    InvalidMessageField(ParseMessageFieldError),
    ExpectedCloseCurly,
}

impl Error for ParseMessageError {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self {
            ExpectedWhitespace => "whitespace",
            ExpectedMessageName => "a message name",
            ExpectedOpenCurly => "an opening curly bracket '{'",
            InvalidMessageField(e) => return e.info(token),
            ExpectedCloseCurly => "a closing curly bracket '}'",
        };
        P_MESSAGE.expected_got_instead(expected, token)
    }
}

/// Parses an optional message.
///
/// Returns `Ok(message, after_close_curly)`.
/// Returns `Ok(None, c)` if the next token is not `message`.
pub fn parse_message(c: Context) -> ParseResult<Option<MessageTree>, ParseMessageError> {
    match c.exact_symbol("message") {
        (Some(_message), after_message) => match after_message.whitespace() {
            (Some(_white), after_white) => match after_white.symbol() {
                (Some(message_name), after_message_name) => {
                    parse_message_block(message_name, after_message_name)
                }
                (None, _) => Err(after_white.to_error(ExpectedMessageName)),
            },
            (None, _) => Err(after_message.to_error(ExpectedWhitespace)),
        },
        _ => Ok((None, c)),
    }
}

fn parse_message_block<'a>(
    message_name: Token<'a>,
    c: Context<'a>,
) -> ParseResult<'a, Option<MessageTree<'a>>, ParseMessageError> {
    let (_white, after_white) = c.whitespace();
    match after_white.exact("{") {
        (Some(_open), after_open) => {
            let (fields, after_fields) = parse_message_fields(after_open)?;
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
                _ => Err(after_white.to_error(ExpectedCloseCurly)),
            }
        }
        _ => Err(after_white.to_error(ExpectedOpenCurly)),
    }
}

fn parse_message_fields(mut c: Context) -> ParseResult<Vec<MessageFieldTree>, ParseMessageError> {
    let mut fields: Vec<MessageFieldTree> = Vec::default();
    while let (Some(field), after_field) =
        parse_message_field(c).map_err(|e| e.map(|e| InvalidMessageField(e)))?
    {
        fields.push(field);
        c = after_field;
    }
    Ok((fields, c))
}
