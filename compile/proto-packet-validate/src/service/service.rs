use crate::InvalidServiceReason::*;
use crate::{validate_service_call, Error, ErrorInfo, InvalidServiceCallError, V_SERVICE};
use lex::{ParseContext, Token};
use proto_packet_parse::ServiceTree;
use proto_packet_tree::{Service, ServiceCall, TypeName, WithComments, WithServiceCallName};

#[derive(Debug)]
pub struct InvalidServiceError<'a> {
    pub service_name: Token<'a>,
    pub reason: InvalidServiceReason<'a>,
}

#[derive(Debug)]
pub enum InvalidServiceReason<'a> {
    InvalidName { error: &'static str },
    InvalidServiceCall(InvalidServiceCallError<'a>),
    DuplicateServiceCallName { call_names: Vec<Token<'a>> },
}

impl<'a> Error for InvalidServiceError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        match &self.reason {
            InvalidName { error } => {
                V_SERVICE.invalid_name(file_name, context, "service", self.service_name, *error)
            }
            InvalidServiceCall(e) => e.info(file_name, context),
            DuplicateServiceCallName { call_names } => {
                V_SERVICE.duplicate_decs(file_name, context, "service names", call_names)
            }
        }
    }
}

pub fn validate_service<'a>(tree: &'a ServiceTree<'a>) -> Result<Service, InvalidServiceError<'a>> {
    let service_name: TypeName =
        TypeName::new(tree.service_name.value()).map_err(|error_service| InvalidServiceError {
            service_name: tree.service_name,
            reason: InvalidName {
                error: error_service,
            },
        })?;
    let mut service: Service = Service::from(service_name);
    for comment in &tree.comments {
        service.add_comment(comment.value().trim_end());
    }

    for call in &tree.service_calls {
        let call: ServiceCall = validate_service_call(&call).map_err(|e| InvalidServiceError {
            service_name: tree.service_name,
            reason: InvalidServiceCall(e),
        })?;

        if service
            .service_call_with_name(call.service_call_name())
            .is_some()
        {
            return Err(InvalidServiceError {
                service_name: tree.service_name,
                reason: DuplicateServiceCallName {
                    call_names: tree.service_call_name_tokens(call.service_call_name()),
                },
            });
        }

        unsafe { service.add_service_call(call) }
    }

    Ok(service)
}
