use lex::ParseContext;
use proto_packet_parse::TypeDecTree;
use proto_packet_parse::TypeDecTree::*;
use proto_packet_tree::TypeDec;

use crate::InvalidTypeDecError::*;
use crate::{
    validate_enum, validate_message, validate_service, validate_struct, validate_variant, Error,
    ErrorInfo, InvalidEnumError, InvalidMessageError, InvalidServiceError, InvalidStructError,
    InvalidVariantError,
};

#[derive(Debug)]
pub enum InvalidTypeDecError<'a> {
    InvalidStruct(InvalidStructError<'a>),
    InvalidMessage(InvalidMessageError<'a>),
    InvalidEnum(InvalidEnumError<'a>),
    InvalidVariant(InvalidVariantError<'a>),
    InvalidService(InvalidServiceError<'a>),
}

impl<'a> From<InvalidStructError<'a>> for InvalidTypeDecError<'a> {
    fn from(error: InvalidStructError<'a>) -> Self {
        InvalidStruct(error)
    }
}

impl<'a> From<InvalidMessageError<'a>> for InvalidTypeDecError<'a> {
    fn from(error: InvalidMessageError<'a>) -> Self {
        InvalidMessage(error)
    }
}

impl<'a> From<InvalidEnumError<'a>> for InvalidTypeDecError<'a> {
    fn from(error: InvalidEnumError<'a>) -> Self {
        InvalidEnum(error)
    }
}

impl<'a> From<InvalidVariantError<'a>> for InvalidTypeDecError<'a> {
    fn from(error: InvalidVariantError<'a>) -> Self {
        InvalidVariant(error)
    }
}

impl<'a> From<InvalidServiceError<'a>> for InvalidTypeDecError<'a> {
    fn from(error: InvalidServiceError<'a>) -> Self {
        InvalidService(error)
    }
}

impl<'a> Error for InvalidTypeDecError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        match self {
            InvalidStruct(e) => e.info(file_name, context),
            InvalidMessage(e) => e.info(file_name, context),
            InvalidEnum(e) => e.info(file_name, context),
            InvalidVariant(e) => e.info(file_name, context),
            InvalidService(e) => e.info(file_name, context),
        }
    }
}

pub fn validate_type_dec<'a>(
    tree: &'a TypeDecTree<'a>,
) -> Result<TypeDec, InvalidTypeDecError<'a>> {
    match tree {
        MessageDec(message) => Ok(validate_message(message)?.into()),
        StructDec(structure) => Ok(validate_struct(structure)?.into()),
        EnumDec(enumeration) => Ok(validate_enum(enumeration)?.into()),
        VariantDec(variant) => Ok(validate_variant(variant)?.into()),
        ServiceDec(service) => Ok(validate_service(service)?.into()),
    }
}
