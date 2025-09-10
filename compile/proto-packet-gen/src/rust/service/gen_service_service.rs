use crate::rust::GenRust;
use code_gen::rust::{
    Access, Function, MatchCase, MatchStatement, RustType, Signature, WithAccess, WithFnGenerics,
    WithResult, WithVarParams,
};
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet_tree::{ModPathRef, Service, WithServiceCallName, WithTypeName};

impl GenRust {
    //! Gen Service: Service

    /// Generates the service.
    pub fn gen_service_service(&self, mod_path: ModPathRef, service: &Service) -> Source {
        Source::default()
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_service_service_packet_fn(service))
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_service_service_fn(mod_path, service))
    }

    fn gen_service_service_packet_fn(&self, service: &Service) -> Function {
        let signature: Signature = Signature::from("service_packet")
            .with_generic(("S", self.naming.type_name(service)))
            .with_generic(("W", "std::io::Write"))
            .with_param(("service", "&S"))
            .with_param(("service_call_name", "&str"))
            .with_param(("input", "actix_web::web::Bytes"))
            .with_param(("output", "&mut W"))
            .with_result(
                RustType::from("Result")
                    .with_generic("usize")
                    .with_generic("ServiceError"),
            );

        let mut match_statement: MatchStatement = MatchStatement::from("service_call_name");
        for service_call in service.service_calls() {
            let match_case: MatchCase = MatchCase::from(format!(
                "\"{}\"",
                self.naming
                    .service_call_name(service_call.service_call_name())
            ))
            .with_literal(format!(
                "{}(input, output, |input| service.{}(input))",
                "proto_packet::service::service_packet",
                self.naming
                    .service_call_name(service_call.service_call_name())
            ));

            match_statement.add_match_case(match_case);
        }
        match_statement.add_match_case(MatchCase::from("_").with_literal("todo!()"));

        Function::from(signature).with_statement(match_statement)
    }

    fn gen_service_service_fn(&self, mod_path: ModPathRef, service: &Service) -> Function {
        let signature: Signature = Signature::from("service")
            .with_generic((
                "S",
                format!("'static + {} + Sync + Send", self.naming.type_name(service)),
            ))
            .with_param(("service", "S"))
            .with_result("actix_web::Scope");

        Function::from(signature)
            .with_access(Access::Public)
            .with_literal(format!(
                "{}(service, \"{}\", |{}| service_packet({}))",
                "proto_packet::service::service",
                self.typing
                    .rust_name(mod_path.to_qualified_name(service.type_name()).to_ref()),
                "service, call, input, output",
                "service, call, input, output",
            ))
    }
}
