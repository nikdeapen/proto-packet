use crate::error::V_STRUCT_FIELD;
use crate::InvalidStructFieldReason::*;
use crate::{validate_type_tag, Error, ErrorInfo, InvalidTypeTagError};
use lex::{Context, Token};
use proto_packet_parse::StructFieldTree;
use proto_packet_tree::{FieldName, StructField, TypeTag, WithComments};

#[derive(Debug)]
pub struct InvalidStructFieldError<'a> {
    pub field_name: Token<'a>,
    pub reason: InvalidStructFieldReason<'a>,
}

#[derive(Debug)]
pub enum InvalidStructFieldReason<'a> {
    InvalidFieldName { error: &'static str },
    InvalidTypeTag(InvalidTypeTagError<'a>),
}

impl<'a> Error for InvalidStructFieldError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match &self.reason {
            InvalidFieldName { error } => V_STRUCT_FIELD.invalid_name(
                file_name,
                context,
                "struct field",
                self.field_name,
                error,
            ),
            InvalidTypeTag(e) => e.info(file_name, context),
        }
    }
}

pub fn validate_struct_field<'a>(
    tree: &'a StructFieldTree,
) -> Result<StructField, InvalidStructFieldError<'a>> {
    let field_name: FieldName =
        FieldName::new(tree.field_name.value()).map_err(|error_message| {
            InvalidStructFieldError {
                field_name: tree.field_name,
                reason: InvalidFieldName {
                    error: error_message,
                },
            }
        })?;
    let type_tag: TypeTag =
        validate_type_tag(&tree.type_tag).map_err(|e| InvalidStructFieldError {
            field_name: tree.field_name,
            reason: InvalidTypeTag(e),
        })?;

    let mut field: StructField = StructField::new(field_name, type_tag);
    for comment in &tree.comments {
        field.add_comment(comment.value());
    }

    Ok(field)
}
