use code_gen::rust::{Function, ImplBlock, RustType, Signature, WithFunctions, WithResult};
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::{Message, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! `proto_packet::Packet`

    /// Generates the `Packet` and `Message` impls for the `message`.
    pub(in crate::rust::message) fn gen_message_impls(&self, message: &Message) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_message_packet_impl(message));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_message_message_impl(message));

        source
    }

    fn gen_message_message_impl(&self, message: &Message) -> ImplBlock {
        ImplBlock::from(self.naming.type_name(message.type_name())).with_for_trait("Message")
    }

    fn gen_message_packet_impl(&self, message: &Message) -> ImplBlock {
        let signature: Signature =
            Signature::from("wire_type").with_result(RustType::from("WireType"));
        let wire_type: Function =
            Function::from(signature).with_literal("WireType::LengthPrefixed");
        ImplBlock::from(self.naming.type_name(message.type_name()))
            .with_for_trait("Packet")
            .with_function(wire_type)
    }
}
