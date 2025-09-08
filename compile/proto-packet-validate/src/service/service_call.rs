use crate::InvalidServiceCallReason::*;
use crate::{validate_type_tag, Error, ErrorInfo, InvalidTypeTagError, V_SERVICE_CALL};
use lex::{ParseContext, Token};
use proto_packet_parse::ServiceCallTree;
use proto_packet_tree::{ServiceCall, ServiceCallName, TypeTag, WithComments};

#[derive(Debug)]
pub struct InvalidServiceCallError<'a> {
    pub service_call_name: Token<'a>,
    pub reason: InvalidServiceCallReason<'a>,
}

#[derive(Debug)]
pub enum InvalidServiceCallReason<'a> {
    InvalidServiceCallName { error: &'static str },
    InvalidInputType(InvalidTypeTagError<'a>),
    InvalidOutputType(InvalidTypeTagError<'a>),
}

impl<'a> Error for InvalidServiceCallError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        match &self.reason {
            InvalidServiceCallName { error } => V_SERVICE_CALL.invalid_name(
                file_name,
                context,
                "service call",
                self.service_call_name,
                *error,
            ),
            InvalidInputType(e) => e.info(file_name, context),
            InvalidOutputType(e) => e.info(file_name, context),
        }
    }
}

pub fn validate_service_call<'a>(
    tree: &'a ServiceCallTree,
) -> Result<ServiceCall, InvalidServiceCallError<'a>> {
    let call_name: ServiceCallName =
        ServiceCallName::new(tree.service_call_name.value()).map_err(|error_service| {
            InvalidServiceCallError {
                service_call_name: tree.service_call_name,
                reason: InvalidServiceCallName {
                    error: error_service,
                },
            }
        })?;
    let input_type: TypeTag =
        validate_type_tag(&tree.input_type).map_err(|e| InvalidServiceCallError {
            service_call_name: tree.service_call_name,
            reason: InvalidInputType(e),
        })?;
    let output_type: TypeTag =
        validate_type_tag(&tree.output_type).map_err(|e| InvalidServiceCallError {
            service_call_name: tree.service_call_name,
            reason: InvalidOutputType(e),
        })?;

    let mut call: ServiceCall = ServiceCall::new(call_name, input_type, output_type);
    for comment in &tree.comments {
        call.add_comment(comment.value());
    }

    Ok(call)
}
