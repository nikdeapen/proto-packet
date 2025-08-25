use crate::ParseTypeDecError::*;
use crate::TypeDecTree::*;
use crate::{
    parse_enum, parse_message, parse_struct, parse_variant, EnumTree, Error, ErrorInfo,
    MessageTree, ParseEnumError, ParseMessageError, ParseStructError, ParseVariantError,
    StructTree, VariantTree,
};
use lex::{ParseContext, ParseResult, Token};

#[derive(Debug)]
pub enum TypeDecTree<'a> {
    StructDec(StructTree<'a>),
    MessageDec(MessageTree<'a>),
    EnumDec(EnumTree<'a>),
    VariantDec(VariantTree<'a>),
}

impl<'a> TypeDecTree<'a> {
    //! Properties

    /// Gets the type name token.
    pub fn type_name_token(&self) -> Token<'_> {
        match self {
            StructDec(structure) => structure.struct_name,
            MessageDec(message) => message.message_name,
            EnumDec(enom) => enom.enum_name,
            VariantDec(variant) => variant.variant_name,
        }
    }
}

#[derive(Debug)]
pub enum ParseTypeDecError<'a> {
    InvalidStruct(ParseStructError<'a>),
    InvalidMessage(ParseMessageError<'a>),
    InvalidEnum(ParseEnumError<'a>),
    InvalidVariant(ParseVariantError<'a>),
}

impl<'a> Error for ParseTypeDecError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        match self {
            InvalidStruct(e) => e.info(token),
            InvalidMessage(e) => e.info(token),
            InvalidEnum(e) => e.info(token),
            InvalidVariant(e) => e.info(token),
        }
    }
}

/// Parses an optional type dec tree.
///
/// Returns `(Some(type_dec), after_type_dec)`.
/// Returns `(None, c)` if the next non-white token does not start a valid type dec.
pub fn parse_type_dec(c: ParseContext) -> ParseResult<Option<TypeDecTree>, ParseTypeDecError> {
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
        Ok((Some(mut enom), after_enum)) => {
            enom.comments = comments;
            return Ok((Some(EnumDec(enom)), after_enum));
        }
        Err(e) => return Err(e.map(|e| InvalidEnum(e))),
        _ => {}
    }

    match parse_variant(after_white) {
        Ok((Some(mut variant), after_variant)) => {
            variant.comments = comments;
            return Ok((Some(VariantDec(variant)), after_variant));
        }
        Err(e) => return Err(e.map(|e| InvalidVariant(e))),
        _ => {}
    }

    Ok((None, c))
}
