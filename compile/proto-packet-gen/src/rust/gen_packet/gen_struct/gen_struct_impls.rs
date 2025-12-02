use crate::rust::GenRust;
use code_gen::rust::ImplBlock;
use code_gen::{Source, WithStatements};
use proto_packet::io::WireType::LengthPrefixed;
use proto_packet_tree::Struct;

impl GenRust {
    //! Gen Struct: Impls

    /// Generates the trait impls for the struct `s`.
    pub(in crate::rust::gen_packet::gen_struct) fn gen_struct_impls(&self, s: &Struct) -> Source {
        Source::default()
            .with_empty_line()
            .with_statement(self.gen_packet_impl(s, LengthPrefixed))
            .with_empty_line()
            .with_statement(self.gen_struct_impl(s))
    }

    fn gen_struct_impl(&self, s: &Struct) -> ImplBlock {
        ImplBlock::from(self.naming.type_name(s)).with_for_trait("proto_packet::Struct")
    }
}
