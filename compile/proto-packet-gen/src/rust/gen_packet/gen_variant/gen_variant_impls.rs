use crate::rust::{GenRust, Naming};
use code_gen::rust::{
    Function, ImplBlock, MatchCase, MatchStatement, Receiver, RustPrimitive, Signature,
    WithFunctions, WithReceiver, WithResult,
};
use code_gen::{Source, WithStatements};
use proto_packet::io::WireType::LengthPrefixed;
use proto_packet_tree::{Variant, WithTagNumber};

impl GenRust {
    //! Gen Variant: Implementations

    pub(in crate::rust::gen_packet::gen_variant) fn gen_variant_impls(
        &self,
        e: &Variant,
    ) -> Source {
        Source::default()
            .with_empty_line()
            .with_statement(self.gen_packet_impl(e, LengthPrefixed))
            .with_empty_line()
            .with_statement(self.gen_variant_impl_variant(e))
            .with_empty_line()
            .with_statement(self.gen_variant_impl_with_tag_number(e))
    }

    fn gen_variant_impl_variant(&self, e: &Variant) -> ImplBlock {
        ImplBlock::from(self.naming.type_name(e)).with_for_trait("proto_packet::Variant")
    }

    fn gen_variant_impl_with_tag_number(&self, e: &Variant) -> ImplBlock {
        let mut match_statement: MatchStatement =
            MatchStatement::from("self").with_assignment(("tag", RustPrimitive::UnsignedInt32));
        match_statement.add_match_case(
            MatchCase::from(format!("Self::{}(u)", Naming::UNRECOGNIZED_CASE_NAME))
                .with_semi("use enc::DecodeFromReadPrefix")
                .with_semi("use std::io::Cursor")
                .with_semi("let mut serial: Cursor<&[u8]> = Cursor::new(u.serial.as_slice())")
                .with_semi(format!(
                    "let header: {} = {}::decode_from_read_prefix(&mut serial).expect({})",
                    Naming::FIELD_HEADER,
                    Naming::FIELD_HEADER,
                    "\"'serial' must start with a valid field header\""
                ))
                .with_literal("header.tag().tag()"),
        );
        for case in e.cases() {
            match_statement.add_match_case(
                MatchCase::from(format!("Self::{}(_)", self.naming.case_name(case)))
                    .with_literal(case.tag().to_string()),
            )
        }

        ImplBlock::from(self.naming.type_name(e))
            .with_for_trait("proto_packet::io::WithTagNumber")
            .with_function(
                Function::from(
                    Signature::from("tag")
                        .with_receiver(Receiver::Borrowed)
                        .with_result("proto_packet::io::TagNumber"),
                )
                .with_statement(match_statement)
                .with_literal("unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }"),
            )
    }
}
