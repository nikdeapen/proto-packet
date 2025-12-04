use crate::rust::{GenRust, Naming};
use code_gen::rust::{
    Function, ImplBlock, MatchCase, MatchStatement, Receiver, RustPrimitive, Signature,
    WithFunctions, WithReceiver, WithResult,
};
use code_gen::{Source, WithStatements};
use proto_packet::io::WireType::VarInt;
use proto_packet_tree::{Enum, WithTagNumber};

impl GenRust {
    //! Gen Enum: Implementations

    pub(in crate::rust::gen_packet::gen_enum) fn gen_enum_impls(&self, e: &Enum) -> Source {
        Source::default()
            .with_statement(self.gen_packet_impl(e, VarInt))
            .with_empty_line()
            .with_statement(self.gen_enum_impl_enum(e))
            .with_empty_line()
            .with_statement(self.gen_enum_impl_with_tag_number(e))
    }

    fn gen_enum_impl_enum(&self, e: &Enum) -> ImplBlock {
        ImplBlock::from(self.naming.type_name(e)).with_for_trait("proto_packet::Enum")
    }

    fn gen_enum_impl_with_tag_number(&self, e: &Enum) -> ImplBlock {
        let mut match_statement: MatchStatement = MatchStatement::from("self")
            .with_assignment(("tag_number", RustPrimitive::UnsignedInt32));
        match_statement.add_match_case(
            MatchCase::from(format!("Self::{}(u)", Naming::UNRECOGNIZED_ENUM_CASE_NAME))
                .with_literal("u.tag.value()"),
        );
        for case in e.cases() {
            match_statement.add_match_case(
                MatchCase::from(format!("Self::{}", self.naming.case_name(case)))
                    .with_literal(case.tag_number().to_string()),
            )
        }

        ImplBlock::from(self.naming.type_name(e))
            .with_for_trait("proto_packet::io::WithTagNumber")
            .with_function(
                Function::from(
                    Signature::from("tag_number")
                        .with_receiver(Receiver::Borrowed)
                        .with_result("proto_packet::io::TagNumber"),
                )
                .with_statement(match_statement)
                .with_literal("unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }"),
            )
    }
}
