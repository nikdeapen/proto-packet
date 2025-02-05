use code_gen::rust::{
    Function, ImplBlock, MatchCase, MatchStatement, Reference, RustType, Signature, WithFunctions,
    WithResult, WithTypeDecs, WithVarParams,
};
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::{Enum, WithCaseName, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Enum FromStr

    /// Generates the `Packet` and `Enum` impls for the `enom`.
    pub(in crate::rust::enums) fn gen_enum_from_str(&self, enom: &Enum) -> Source {
        let from_str: Function = Function::from(
            Signature::from("from_str")
                .with_param(("s", RustType::from("str").to_ref_type(Reference::default())))
                .with_result(
                    RustType::from("Result")
                        .with_generic(RustType::SelfType)
                        .with_generic(RustType::from("Self::Err")),
                ),
        )
        .with_statement(self.gen_enum_from_str_impl(enom));
        let block: ImplBlock = ImplBlock::from(self.naming.type_name(enom.type_name()))
            .with_for_trait("FromStr")
            .with_type_dec(("Err", RustType::from("()")))
            .with_function(from_str);
        Source::default()
            .with_statement(EmptyLine::default())
            .with_statement(block)
    }

    fn gen_enum_from_str_impl(&self, enom: &Enum) -> Source {
        let mut match_statement: MatchStatement = MatchStatement::from("s");
        for case in enom.cases() {
            let case_name: String = self.naming.case_name(case.case_name());
            let match_case: MatchCase = MatchCase::from(format!("\"{}\"", case_name))
                .with_literal(format!("Ok(Self::{})", case_name));
            match_statement.add_match_case(match_case);
        }
        match_statement.add_match_case(MatchCase::from("_").with_literal("Err(())"));
        Source::default().with_statement(match_statement)
    }
}
