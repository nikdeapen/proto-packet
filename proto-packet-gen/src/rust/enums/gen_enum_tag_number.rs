use code_gen::rust::Access::Public;
use code_gen::rust::{
    Function, ImplBlock, MatchCase, MatchStatement, Receiver, RustPrimitive, RustType, Signature,
    WithAccess, WithComments, WithFunctions, WithReceiver, WithResult, WithVarParams,
};
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::{Enum, WithCaseName, WithTagNumber, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Enum Tag Number

    pub(in crate::rust::enums) fn gen_enum_tag_number(&self, enom: &Enum) -> Source {
        Source::default()
            .with_statement(EmptyLine::default())
            .with_statement(
                ImplBlock::from(enom.type_name())
                    .with_comment(" Tag Numbers")
                    .with_function(self.gen_enum_tag_number_get(enom)),
            )
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_enum_from_tag_number(enom))
    }

    fn gen_enum_tag_number_get(&self, enom: &Enum) -> Function {
        let signature: Signature = Signature::from("tag_number")
            .with_receiver(Receiver::Borrowed)
            .with_result("proto_packet::io::TagNumber");

        let mut match_statement: MatchStatement = MatchStatement::from("self")
            .with_assignment(("tag_number", RustPrimitive::UnsignedInt32));
        for case in enom.cases() {
            match_statement.add_match_case(
                MatchCase::from(format!("Self::{}", self.naming.case_name(case.case_name())))
                    .with_literal(case.tag_number().to_string()),
            );
        }
        match_statement.add_match_case(
            MatchCase::from("Self::Unrecognized(tag_number)")
                .with_literal("tag_number.tag_number()"),
        );

        Function::from(signature)
            .with_access(Public)
            .with_comment(" Gets the tag number.")
            .with_statement(match_statement)
            .with_literal(format!(
                "unsafe {{ proto_packet::io::TagNumber::new_unchecked(tag_number) }}"
            ))
    }

    fn gen_enum_from_tag_number(&self, enom: &Enum) -> ImplBlock {
        let mut match_statement: MatchStatement = MatchStatement::from("tag_number.tag_number()");
        for case in enom.cases() {
            match_statement.add_match_case(
                MatchCase::from(case.tag_number().to_string())
                    .with_literal(format!("Self::{}", self.naming.case_name(case.case_name()))),
            );
        }
        match_statement
            .add_match_case(MatchCase::from("_").with_literal("Self::Unrecognized(tag_number)"));

        let signature: Signature = Signature::from("from")
            .with_param(("tag_number", "proto_packet::io::TagNumber"))
            .with_result(RustType::SelfType);
        let from: Function = Function::from(signature).with_statement(match_statement);
        ImplBlock::from(RustType::from(self.naming.type_name(enom.type_name())))
            .with_for_trait(RustType::from("From").with_generic("proto_packet::io::TagNumber"))
            .with_function(from)
    }
}
