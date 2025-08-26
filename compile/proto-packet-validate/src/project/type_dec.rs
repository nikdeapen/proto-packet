use lex::ParseContext;
use proto_packet_parse::TypeDecTree;
use proto_packet_parse::TypeDecTree::*;
use proto_packet_tree::TypeDec;

use crate::InvalidTypeDecError::*;
use crate::{
    validate_enum, validate_message, validate_struct, validate_variant, Error, ErrorInfo,
    InvalidEnumError, InvalidMessageError, InvalidStructError, InvalidVariantError,
};

#[derive(Debug)]
pub enum InvalidTypeDecError<'a> {
    InvalidStruct(InvalidStructError<'a>),
    InvalidMessage(InvalidMessageError<'a>),
    InvalidEnum(InvalidEnumError<'a>),
    InvalidVariant(InvalidVariantError<'a>),
}

impl<'a> Error for InvalidTypeDecError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        match self {
            InvalidStruct(e) => e.info(file_name, context),
            InvalidMessage(e) => e.info(file_name, context),
            InvalidEnum(e) => e.info(file_name, context),
            InvalidVariant(e) => e.info(file_name, context),
        }
    }
}

pub fn validate_type_dec<'a>(
    tree: &'a TypeDecTree<'a>,
) -> Result<TypeDec, InvalidTypeDecError<'a>> {
    match tree {
        StructDec(structure) => validate_struct(structure)
            .map(|structure| TypeDec::from(structure))
            .map_err(|e| InvalidStruct(e)),
        MessageDec(message) => validate_message(message)
            .map(|message| TypeDec::from(message))
            .map_err(|e| InvalidMessage(e)),
        EnumDec(enom) => validate_enum(enom)
            .map(|enom| TypeDec::from(enom))
            .map_err(|e| InvalidEnum(e)),
        VariantDec(variant) => validate_variant(variant)
            .map(|variant| TypeDec::from(variant))
            .map_err(|e| InvalidVariant(e)),
    }
}
