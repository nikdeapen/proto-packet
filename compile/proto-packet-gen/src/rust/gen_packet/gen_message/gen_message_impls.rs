use crate::rust::GenRust;
use code_gen::rust::ImplBlock;
use code_gen::{Source, WithStatements};
use proto_packet::io::WireType::LengthPrefixed;
use proto_packet_tree::Message;

impl GenRust {
    //! Gen Message: Impls

    /// Generates the trait impls for the message `m`.
    pub(in crate::rust::gen_packet::gen_message) fn gen_message_impls(
        &self,
        m: &Message,
    ) -> Source {
        Source::default()
            .with_empty_line()
            .with_statement(self.gen_packet_impl(m, LengthPrefixed))
            .with_empty_line()
            .with_statement(self.gen_message_impl(m))
    }

    fn gen_message_impl(&self, s: &Message) -> ImplBlock {
        ImplBlock::from(self.naming.type_name(s)).with_for_trait("proto_packet::Message")
    }
}
