use crate::ParseTypeDecError::*;
use crate::TypeDecTree::*;
use crate::{
    parse_enum, parse_message, parse_struct, EnumTree, Error, ErrorInfo, MessageTree,
    ParseEnumError, ParseMessageError, ParseStructError, StructTree,
};
use lex::{Context, ParseResult, Token};

#[derive(Debug)]
pub enum TypeDecTree<'a> {
    StructDec(StructTree<'a>),
    MessageDec(MessageTree<'a>),
    EnumDec(EnumTree<'a>),
}

impl<'a> TypeDecTree<'a> {
    //! Properties

    /// Gets the type name token.
    pub fn type_name_token(&self) -> Token<'_> {
        match self {
            StructDec(s) => s.struct_name,
            MessageDec(m) => m.message_name,
            EnumDec(e) => e.enum_name,
        }
    }
}

#[derive(Debug)]
pub enum ParseTypeDecError {
    InvalidStruct(ParseStructError),
    InvalidMessage(ParseMessageError),
    InvalidEnum(ParseEnumError),
}

impl Error for ParseTypeDecError {
    fn info(&self, token: &str) -> ErrorInfo {
        match self {
            InvalidStruct(e) => e.info(token),
            InvalidMessage(e) => e.info(token),
            InvalidEnum(e) => e.info(token),
        }
    }
}

/// Parses an optional type dec.
///
/// Returns `(Some(type_dec), after_type_dec)`.
/// Returns `(None, c)` if the next non-white token does not start a valid type dec.
pub fn parse_type_dec(c: Context) -> ParseResult<Option<TypeDecTree>, ParseTypeDecError> {
    let (comments, after_white) = c.line_comment_block();

    match parse_struct(after_white) {
        Ok((Some(mut structure), after_struct)) => {
            structure.comments = comments;
            return Ok((Some(StructDec(structure)), after_struct));
        }
        Err(e) => return Err(e.map(|e| InvalidStruct(e))),
        _ => {}
    }

    match parse_message(after_white) {
        Ok((Some(mut message), after_message)) => {
            message.comments = comments;
            return Ok((Some(MessageDec(message)), after_message));
        }
        Err(e) => return Err(e.map(|e| InvalidMessage(e))),
        _ => {}
    }

    match parse_enum(after_white) {
        Ok((Some(mut enumeration), after_enumeration)) => {
            enumeration.comments = comments;
            return Ok((Some(EnumDec(enumeration)), after_enumeration));
        }
        Err(e) => return Err(e.map(|e| InvalidEnum(e))),
        _ => {}
    }

    Ok((None, c))
}
