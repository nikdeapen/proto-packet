use crate::InvalidStructReason::InvalidName;
use crate::InvalidStructReason::{DuplicateFieldName, InvalidField};
use crate::{validate_struct_field, Error, ErrorInfo, InvalidStructFieldError, V_STRUCT};
use lex::{Context, Token};
use proto_packet_parse::StructTree;
use proto_packet_tree::{Struct, StructField, TypeName, WithComments, WithFieldName};

#[derive(Debug)]
pub struct InvalidStructError<'a> {
    pub struct_name: Token<'a>,
    pub reason: InvalidStructReason<'a>,
}

#[derive(Debug)]
pub enum InvalidStructReason<'a> {
    InvalidName { error: &'static str },
    InvalidField(InvalidStructFieldError<'a>),
    DuplicateFieldName { field_names: Vec<Token<'a>> },
}

impl<'a> Error for InvalidStructError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match &self.reason {
            InvalidName { error } => {
                V_STRUCT.invalid_name(file_name, context, "struct", self.struct_name, error)
            }
            InvalidField(e) => e.info(file_name, context),
            DuplicateFieldName { field_names } => {
                V_STRUCT.duplicate_decs(file_name, context, "struct", field_names)
            }
        }
    }
}

pub fn validate_struct<'a>(tree: &'a StructTree<'a>) -> Result<Struct, InvalidStructError<'a>> {
    let struct_name: TypeName =
        TypeName::new(tree.struct_name.value()).map_err(|error| InvalidStructError {
            struct_name: tree.struct_name,
            reason: InvalidName { error },
        })?;
    let mut structure: Struct = Struct::from(struct_name);

    for comment in &tree.comments {
        structure.add_comment(comment.value().trim_end());
    }

    for field in &tree.fields {
        let field: StructField = validate_struct_field(&field).map_err(|e| InvalidStructError {
            struct_name: tree.struct_name,
            reason: InvalidField(e),
        })?;

        if structure.field_with_name(field.field_name()).is_some() {
            return Err(InvalidStructError {
                struct_name: tree.struct_name,
                reason: DuplicateFieldName {
                    field_names: tree.field_name_tokens(field.field_name()),
                },
            });
        }

        unsafe { structure.add_field(field) }
    }

    Ok(structure)
}
