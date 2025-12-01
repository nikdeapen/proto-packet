use lex::Context;
use proto_packet_parse::TypeDecTree;
use proto_packet_parse::TypeDecTree::*;
use proto_packet_tree::TypeDec;

use crate::InvalidTypeDecError::*;
use crate::{validate_struct, Error, ErrorInfo, InvalidStructError};

#[derive(Debug)]
pub enum InvalidTypeDecError<'a> {
    InvalidStruct(InvalidStructError<'a>),
}

impl<'a> From<InvalidStructError<'a>> for InvalidTypeDecError<'a> {
    fn from(error: InvalidStructError<'a>) -> Self {
        InvalidStruct(error)
    }
}

impl<'a> Error for InvalidTypeDecError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match self {
            InvalidStruct(e) => e.info(file_name, context),
        }
    }
}

pub fn validate_type_dec<'a>(
    tree: &'a TypeDecTree<'a>,
) -> Result<TypeDec, InvalidTypeDecError<'a>> {
    match tree {
        StructDec(structure) => Ok(validate_struct(structure)?.into()),
    }
}
