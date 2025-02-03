use lex::ParseContext;

use proto_packet_parse::TypeDecTree;
use proto_packet_parse::TypeDecTree::*;
use proto_packet_tree::TypeDec;

use crate::InvalidTypeDecError::*;
use crate::{validate_message, Error, ErrorInfo, InvalidMessageError};

#[derive(Debug)]
pub enum InvalidTypeDecError<'a> {
    InvalidMessage(InvalidMessageError<'a>),
}

impl<'a> Error for InvalidTypeDecError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        match self {
            InvalidMessage(e) => e.info(file_name, context),
        }
    }
}

pub fn validate_type_dec<'a>(
    tree: &'a TypeDecTree<'a>,
) -> Result<TypeDec, InvalidTypeDecError<'a>> {
    match tree {
        MessageDec(message) => validate_message(message)
            .map(|message| TypeDec::from(message))
            .map_err(|e| InvalidMessage(e)),
    }
}
