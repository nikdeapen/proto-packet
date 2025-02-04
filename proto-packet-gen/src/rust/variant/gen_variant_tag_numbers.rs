use code_gen::rust::Access::Public;
use code_gen::rust::{
    Function, ImplBlock, MatchCase, MatchStatement, Receiver, RustPrimitive, Signature, WithAccess,
    WithComments, WithFunctions, WithReceiver, WithResult,
};
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet_tree::{Variant, WithCaseName, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Variant Tag Number

    pub(in crate::rust::variant) fn gen_variant_tag_number(&self, variant: &Variant) -> Source {
        Source::default()
            .with_statement(EmptyLine::default())
            .with_statement(
                ImplBlock::from(variant.type_name())
                    .with_comment(" Tag Number")
                    .with_function(self.gen_variant_tag_number_get(variant)),
            )
    }

    fn gen_variant_tag_number_get(&self, variant: &Variant) -> Function {
        let signature: Signature = Signature::from("tag_number")
            .with_receiver(Receiver::Borrowed)
            .with_result("proto_packet::io::TagNumber");

        let mut match_statement: MatchStatement = MatchStatement::from("self")
            .with_assignment(("tag_number", RustPrimitive::UnsignedInt32));
        for case in variant.cases() {
            match_statement.add_match_case(
                MatchCase::from(format!(
                    "Self::{}(_)",
                    self.naming.case_name(case.case_name())
                ))
                .with_literal(case.tag_number().to_string()),
            );
        }

        Function::from(signature)
            .with_access(Public)
            .with_comment(" Gets the tag number.")
            .with_statement(match_statement)
            .with_literal(format!(
                "unsafe {{ proto_packet::io::TagNumber::new_unchecked(tag_number) }}"
            ))
    }
}
