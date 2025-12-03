use crate::rust::{EncodeOp, GenRust};
use code_gen::rust::IfStatement;
use code_gen::{Source, WithStatements};
use proto_packet_tree::{Message, TypeTag, WithTagNumber, WithTypeTag};

impl GenRust {
    //! Gen Message: Encode

    pub(in crate::rust::gen_packet) fn gen_message_encode(&self, m: &Message) -> Source {
        self.gen_encode(m, m.fields().is_empty(), |m, op| {
            self.gen_encode_message_source(m, op)
        })
    }

    pub(in crate::rust) fn gen_encode_message_source(&self, m: &Message, op: EncodeOp) -> Source {
        if m.fields().is_empty() {
            return Source::default().with_literal("Ok(0)");
        }

        let mut source: Source = Source::default();
        if m.fields()
            .iter()
            .any(|f| matches!(f.type_tag(), TypeTag::Named(_)))
        {
            source.add_semi("use proto_packet::Packet");
        }
        source.add_empty_line();
        source.add_semi("let mut encoded_len: usize = 0");
        source.add_empty_line();

        for field in m.fields() {
            source.add_empty_line();
            source.add_statement(
                IfStatement::from(format!(
                    "let Some(value) = &self.{}",
                    self.naming.field_name(field)
                ))
                .with_success_statements(self.gen_encode_field(
                    "value",
                    field.type_tag(),
                    false, // todo -- fixed
                    field.tag_number(),
                    op,
                )),
            );
        }

        source.add_empty_line();
        source.add_literal("Ok(encoded_len)");
        source
    }
}
