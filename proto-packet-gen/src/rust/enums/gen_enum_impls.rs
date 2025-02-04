use code_gen::rust::{Function, ImplBlock, RustType, Signature, WithFunctions, WithResult};
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::{Enum, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! `proto_packet::Packet`

    /// Generates the `Packet` and `Enum` impls for the `enom`.
    pub(in crate::rust::enums) fn gen_enum_impls(&self, enom: &Enum) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_enum_packet_impl(enom));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_enum_enum_impl(enom));

        source
    }

    fn gen_enum_enum_impl(&self, enom: &Enum) -> ImplBlock {
        ImplBlock::from(self.naming.type_name(enom.type_name())).with_for_trait("Enum")
    }

    fn gen_enum_packet_impl(&self, enom: &Enum) -> ImplBlock {
        let signature: Signature =
            Signature::from("wire_type").with_result(RustType::from("WireType"));
        let wire_type: Function = Function::from(signature).with_literal("WireType::VarInt");
        ImplBlock::from(self.naming.type_name(enom.type_name()))
            .with_for_trait("Packet")
            .with_function(wire_type)
    }
}
