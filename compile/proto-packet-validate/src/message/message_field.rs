use crate::InvalidMessageFieldReason::*;
use crate::{
    validate_tag_number, validate_type_tag, Error, ErrorInfo, InvalidTagNumberError,
    InvalidTypeTagError, V_MESSAGE_FIELD,
};
use lex::{Context, Token};
use proto_packet::io::TagNumber;
use proto_packet_parse::MessageFieldTree;
use proto_packet_tree::{FieldName, MessageField, TypeTag, WithComments};

#[derive(Debug)]
pub struct InvalidMessageFieldError<'a> {
    pub field_name: Token<'a>,
    pub reason: InvalidMessageFieldReason<'a>,
}

#[derive(Debug)]
pub enum InvalidMessageFieldReason<'a> {
    InvalidFieldName { error: &'static str },
    InvalidTypeTag(InvalidTypeTagError<'a>),
    InvalidTagNumber(InvalidTagNumberError<'a>),
}

impl<'a> Error for InvalidMessageFieldError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match &self.reason {
            InvalidFieldName { error } => V_MESSAGE_FIELD.invalid_name(
                file_name,
                context,
                "message field",
                self.field_name,
                *error,
            ),
            InvalidTypeTag(e) => e.info(file_name, context),
            InvalidTagNumber(e) => e.info(file_name, context),
        }
    }
}

pub fn validate_message_field<'a>(
    tree: &'a MessageFieldTree,
) -> Result<MessageField, InvalidMessageFieldError<'a>> {
    let field_name: FieldName =
        FieldName::new(tree.field_name.value()).map_err(|error_message| {
            InvalidMessageFieldError {
                field_name: tree.field_name,
                reason: InvalidFieldName {
                    error: error_message,
                },
            }
        })?;
    let type_tag: TypeTag =
        validate_type_tag(&tree.type_tag).map_err(|e| InvalidMessageFieldError {
            field_name: tree.field_name,
            reason: InvalidTypeTag(e),
        })?;
    let tag_number: TagNumber =
        validate_tag_number(tree.tag_number).map_err(|e| InvalidMessageFieldError {
            field_name: tree.field_name,
            reason: InvalidTagNumber(e),
        })?;

    let mut field: MessageField = MessageField::new(field_name, type_tag, tag_number);
    for comment in &tree.comments {
        field.add_comment(comment.value());
    }

    Ok(field)
}
