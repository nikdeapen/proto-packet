use lex::Context;
use proto_packet_parse::TypeDecTree;
use proto_packet_parse::TypeDecTree::*;
use proto_packet_tree::TypeDec;

use crate::InvalidTypeDecError::*;
use crate::{
    validate_message, validate_struct, Error, ErrorInfo, InvalidMessageError, InvalidStructError,
};

#[derive(Debug)]
pub enum InvalidTypeDecError<'a> {
    InvalidStruct(InvalidStructError<'a>),
    InvalidMessage(InvalidMessageError<'a>),
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

impl<'a> Error for InvalidTypeDecError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match self {
            InvalidStruct(e) => e.info(file_name, context),
            InvalidMessage(e) => e.info(file_name, context),
        }
    }
}

pub fn validate_type_dec<'a>(
    tree: &'a TypeDecTree<'a>,
) -> Result<TypeDec, InvalidTypeDecError<'a>> {
    match tree {
        StructDec(structure) => Ok(validate_struct(structure)?.into()),
        MessageDec(message) => Ok(validate_message(message)?.into()),
    }
}
