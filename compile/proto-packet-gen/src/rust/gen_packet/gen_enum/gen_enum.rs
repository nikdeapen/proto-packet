use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet_tree::Enum;

impl GenRust {
    //! Gen Enum

    pub(in crate::rust) fn gen_enum(&self, e: &Enum) -> Source {
        Source::default()
            .with_statement(self.gen_enum_dec(e))
            .with_statement(self.gen_enum_constructors(e))
            .with_statement(self.gen_enum_impls(e))
            .with_statement(self.gen_enum_encode(e))
            .with_statement(self.gen_enum_decode(e))
            .with_statement(self.gen_enum_unrecognized(e))
    }
}
