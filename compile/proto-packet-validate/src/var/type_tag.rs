use crate::error::{Error, ErrorInfo};
use crate::InvalidTypeTagError::InvalidQualifiedName;
use crate::V_TYPE_TAG;
use lex::{Context, Token};
use proto_packet_parse::TypeTagTree;
use proto_packet_tree::{QualifiedName, TypeTag};

#[derive(Debug)]
pub enum InvalidTypeTagError<'a> {
    InvalidQualifiedName {
        name: Token<'a>,
        error_message: String,
    },
}

impl<'a> Error for InvalidTypeTagError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match self {
            InvalidQualifiedName {
                name,
                error_message,
            } => V_TYPE_TAG.info("invalid qualified name").with_token_info(
                file_name,
                context,
                *name,
                error_message.as_str(),
            ),
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
    }
}
