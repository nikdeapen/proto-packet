use crate::rust::GenRust;
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet_tree::Service;

impl GenRust {
    //! Gen Service: Imports

    /// Generates the imports for the `packet_type`.
    pub(in crate::rust) fn gen_service_imports(&self, _service: &Service) -> Source {
        Source::default()
            .with_semi("use proto_packet::service::ServiceError")
            .with_statement(EmptyLine::default())
    }
}
