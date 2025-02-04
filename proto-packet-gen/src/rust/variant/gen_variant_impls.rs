use code_gen::rust::{Function, ImplBlock, RustType, Signature, WithFunctions, WithResult};
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::{Variant, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! `proto_packet::Packet`

    /// Generates the `Packet` and `Variant` impls for the `variant`.
    pub(in crate::rust::variant) fn gen_variant_impls(&self, variant: &Variant) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_variant_packet_impl(variant));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_variant_variant_impl(variant));

        source
    }

    fn gen_variant_variant_impl(&self, variant: &Variant) -> ImplBlock {
        ImplBlock::from(self.naming.type_name(variant.type_name())).with_for_trait("Variant")
    }

    fn gen_variant_packet_impl(&self, variant: &Variant) -> ImplBlock {
        let signature: Signature =
            Signature::from("wire_type").with_result(RustType::from("WireType"));
        let wire_type: Function =
            Function::from(signature).with_literal("WireType::LengthPrefixed");
        ImplBlock::from(self.naming.type_name(variant.type_name()))
            .with_for_trait("Packet")
            .with_function(wire_type)
    }
}
