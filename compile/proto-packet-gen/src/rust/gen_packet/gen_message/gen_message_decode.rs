use crate::rust::GenRust;
use code_gen::rust::{MatchCase, MatchStatement, WhileLoop};
use code_gen::{Literal, Source, WithStatements};
use proto_packet_tree::{
    Message, ModPathRef, TypeTag, WithFieldName, WithTagNumber, WithTypeName, WithTypeTag,
};

impl GenRust {
    //! Gen Message: Decode

    pub(in crate::rust::gen_packet::gen_message) fn gen_message_decode(
        &self,
        _mod_path: ModPathRef,
        m: &Message,
    ) -> Source {
        Source::default()
            .with_empty_line()
            .with_statement(
                self.gen_decode_from_read_impl(m.type_name(), self.gen_decode_message_source(m)),
            )
            .with_empty_line()
            .with_semi(format!(
                "enc::impl_decode_from_read_prefix_length_prefixed!({})",
                self.naming.type_name(m)
            ))
    }

    fn gen_decode_message_source(&self, m: &Message) -> Source {
        let mut source: Source = Source::default();
        if m.fields().is_empty() {
            source.add_semi("use proto_packet::io::FieldHeader");
        } else {
            source.add_semi("use proto_packet::io::{Decoder, FieldHeader}");
        }
        if m.fields()
            .iter()
            .any(|f| matches!(f.type_tag(), TypeTag::Named(_)))
        {
            source.add_semi("use proto_packet::Packet");
        }
        source.add_semi("use enc::DecodeFromReadPrefix");
        source.add_empty_line();
        source.add_semi("let mut result: Self = Self::default()");
        source.add_empty_line();

        source.add_empty_line();
        let mut while_loop: WhileLoop = WhileLoop::from(Literal::from(
            "let Some(first) = enc::read_optional_byte(r)?",
        ));
        while_loop.add_semi(format!(
            "let header: FieldHeader = FieldHeader::{}(r, first)?",
            "decode_from_read_prefix_with_first_byte"
        ));
        let mut match_statement: MatchStatement = MatchStatement::from("header.tag().value()");
        for field in m.fields() {
            let match_case: MatchCase = MatchCase::from(field.tag().to_string())
                .with_statement(self.gen_decode_value(
                    "value",
                    field.field_name(),
                    field.type_tag(),
                    false,
                    Some(field.tag()),
                ))
                .with_semi(format!(
                    "result.set_{}(value)",
                    self.naming.field_name(field)
                ));
            match_statement.add_match_case(match_case);
        }
        match_statement.add_match_case(MatchCase::from("_").with_semi("todo!()"));
        while_loop.add_statement(match_statement);
        source.add_statement(while_loop);

        source.add_literal("Ok(result)");

        source
    }
}
