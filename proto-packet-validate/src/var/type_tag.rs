use colored::Colorize;
use lex::{ParseContext, Token};

use proto_packet_parse::TypeTagTree;
use proto_packet_tree::{QualifiedName, TypeTag};

use crate::error::{Error, ErrorInfo};
use crate::InvalidTypeTagError::InvalidQualifiedName;

#[derive(Debug)]
pub enum InvalidTypeTagError<'a> {
    InvalidQualifiedName {
        name: Token<'a>,
        error_message: String,
    },
}

impl<'a> Error for InvalidTypeTagError<'a> {
    fn info(&self, _file_name: &str, _context: ParseContext) -> ErrorInfo {
        let code: &str = "V_TYPE_TAG";
        match self {
            InvalidQualifiedName {
                name,
                error_message,
            } => ErrorInfo {
                code,
                header: format!("invalid named type: {}", name),
                info: vec![error_message.normal()],
            },
        }
    }
}

pub fn validate_type_tag<'a>(tree: &'a TypeTagTree) -> Result<TypeTag, InvalidTypeTagError<'a>> {
    match tree {
        TypeTagTree::Primitive { primitive, .. } => Ok(primitive.to_type_tag()),
        TypeTagTree::Special { special, .. } => Ok(special.to_type_tag()),
        TypeTagTree::Named { name } => {
            let name: QualifiedName =
                QualifiedName::new(name.value()).map_err(|e| InvalidQualifiedName {
                    name: *name,
                    error_message: e.to_string(),
                })?;
            Ok(TypeTag::from(name))
        }
        TypeTagTree::Slice { base } => Ok(validate_type_tag(base)?.to_slice()),
    }
}
