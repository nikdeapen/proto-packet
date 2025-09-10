use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet_tree::{ModPathRef, Service};

impl GenRust {
    //! Gen Service

    /// Generates the source code for the `service`.
    pub(in crate::rust) fn gen_service(&self, mod_path: ModPathRef, service: &Service) -> Source {
        Source::default()
            .with_statement(self.gen_service_imports(service))
            .with_statement(self.gen_service_trait(service))
            .with_statement(self.gen_service_service(mod_path, service))
    }
}
