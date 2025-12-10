use crate::rust::{GenRust, Naming};
use code_gen::rust::{
    Function, ImplBlock, MatchCase, MatchStatement, Signature, WithFunctions, WithResult,
    WithVarParams,
};
use code_gen::{Source, WithStatements};
use proto_packet_tree::{Enum, WithTagNumber};

impl GenRust {
    //! Gen Enum: Constructors

    /// Generates the constructors for the enum `e`.
    pub(in crate::rust::gen_packet::gen_enum) fn gen_enum_constructors(&self, e: &Enum) -> Source {
        Source::default().with_statement(self.gen_enum_from_tag_number(e))
    }

    fn gen_enum_from_tag_number(&self, e: &Enum) -> Source {
        let mut match_statement: MatchStatement = MatchStatement::from("tag.value()");
        for case in e.cases() {
            match_statement.add_match_case(
                MatchCase::from(case.tag().to_string())
                    .with_literal(format!("Self::{}", self.naming.case_name(case))),
            );
        }
        match_statement.add_match_case(MatchCase::from("_").with_literal(format!(
            "Self::{}({}{} {{ tag }} )",
            Naming::UNRECOGNIZED_CASE_NAME,
            self.naming.type_name(e),
            Naming::UNRECOGNIZED_CASE_NAME,
        )));

        Source::default().with_empty_line().with_statement(
            ImplBlock::from(self.naming.type_name(e))
                .with_for_trait("From<proto_packet::io::TagNumber>")
                .with_function(
                    Function::from(
                        Signature::from("from")
                            .with_param(("tag", "proto_packet::io::TagNumber"))
                            .with_result("Self"),
                    )
                    .with_statement(match_statement),
                ),
        )
    }
}
