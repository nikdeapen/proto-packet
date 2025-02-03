use lex::parse::IntParser;
use lex::{ParseContext, Token};

use proto_packet_parse::MessageTree;
use proto_packet_tree::{
    Message, MessageField, TypeName, WithComments, WithFieldName, WithTagNumberOptional,
};

use crate::error::{duplicate_decs, invalid_name};
use crate::message::message::InvalidMessageReason::InvalidName;
use crate::InvalidMessageReason::{DuplicateFieldName, DuplicateFieldNumber, InvalidField};
use crate::{validate_message_field, Error, ErrorInfo, InvalidMessageFieldError, V_MESSAGE};

#[derive(Debug)]
pub struct InvalidMessageError<'a> {
    pub message_name: Token<'a>,
    pub reason: InvalidMessageReason<'a>,
}

#[derive(Debug)]
pub enum InvalidMessageReason<'a> {
    InvalidName { error_message: &'static str },
    InvalidField(InvalidMessageFieldError<'a>),
    DuplicateFieldName { field_names: Vec<Token<'a>> },
    DuplicateFieldNumber { tag_numbers: Vec<Token<'a>> },
}

impl<'a> Error for InvalidMessageError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        match &self.reason {
            InvalidName { error_message } => invalid_name(
                file_name,
                context,
                "message",
                self.message_name,
                *error_message,
                V_MESSAGE,
            ),
            InvalidField(e) => e.info(file_name, context),
            DuplicateFieldName { field_names } => {
                duplicate_decs(file_name, context, V_MESSAGE, "message names", field_names)
            }
            DuplicateFieldNumber { tag_numbers } => {
                duplicate_decs(file_name, context, V_MESSAGE, "field num bers", tag_numbers)
            }
        }
    }
}

pub fn validate_message<'a>(tree: &'a MessageTree<'a>) -> Result<Message, InvalidMessageError<'a>> {
    let message_name: TypeName =
        TypeName::new(tree.message_name.value()).map_err(|error_message| InvalidMessageError {
            message_name: tree.message_name,
            reason: InvalidName { error_message },
        })?;
    let mut message: Message = Message::from(message_name);

    for comment in &tree.comments {
        message.add_comment(comment.value().trim_end());
    }

    for field in &tree.fields {
        let field: MessageField =
            validate_message_field(&field).map_err(|e| InvalidMessageError {
                message_name: tree.message_name,
                reason: InvalidField(e),
            })?;

        if message.field_with_name(field.field_name()).is_some() {
            return Err(InvalidMessageError {
                message_name: tree.message_name,
                reason: DuplicateFieldName {
                    field_names: tree.field_name_tokens(field.field_name()),
                },
            });
        }

        if let Some(tag_number) = field.tag_number() {
            if message.field_with_number(tag_number).is_some() {
                return Err(InvalidMessageError {
                    message_name: tree.message_name,
                    reason: DuplicateFieldNumber {
                        tag_numbers: tree.tag_number_tokens(tag_number, &IntParser::default()),
                    },
                });
            }
        }

        unsafe { message.add_field(field) }
    }

    Ok(message)
}
