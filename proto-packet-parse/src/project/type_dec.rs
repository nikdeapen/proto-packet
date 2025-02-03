use lex::{ParseContext, Token};

use crate::ParseTypeDecError::*;
use crate::TypeDecTree::*;
use crate::{parse_message, Error, ErrorInfo, MessageTree, ParseMessageError};

#[derive(Debug)]
pub enum TypeDecTree<'a> {
    MessageDec(MessageTree<'a>),
}

impl<'a> TypeDecTree<'a> {
    //! Properties

    /// Gets the type name token.
    pub fn type_name_token(&self) -> Token {
        match self {
            MessageDec(message) => message.message_name,
        }
    }
}

#[derive(Debug)]
pub enum ParseTypeDecError<'a> {
    InvalidMessage(ParseMessageError<'a>),
}

impl<'a> Error for ParseTypeDecError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        match self {
            InvalidMessage(e) => e.info(token),
        }
    }
}

/// Parses an optional type dec tree.
///
/// Returns `(Some(type_dec), after_type_dec)`.
/// Returns `(None, c)` if the next non-white token does not start a valid type dec.
pub fn parse_type_dec(c: ParseContext) -> lex::Result<Option<TypeDecTree>, ParseTypeDecError> {
    let (comments, after_white) = c.line_comment_block();

    match parse_message(comments, after_white) {
        Ok((Some(message), after_message)) => {
            return Ok((Some(MessageDec(message)), after_message))
        }
        Err(e) => return Err(e.map(|e| InvalidMessage(e))),
        _ => {}
    }

    Ok((None, c))
}
