use code_gen::rust::{ImplBlock, MatchCase, MatchStatement};
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::{Variant, WithCaseName, WithTypeName, WithTypeTag};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Variant Encode

    /// Generates the encoding trait implementations for the `variant`.
    pub fn gen_variant_encode(&self, variant: &Variant) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_variant_encoded_len(variant));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_variant_encode_to_slice(variant));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_variant_encode_to_write(variant));

        source
    }
}

impl GenRust {
    //! `enc::EncodedLen`

    fn gen_variant_encoded_len(&self, variant: &Variant) -> ImplBlock {
        let mut match_statement: MatchStatement = MatchStatement::from("self");
        for case in variant.cases() {
            let mut match_case: MatchCase = MatchCase::from(format!(
                "Self::{}(value)",
                self.naming.case_name(case.case_name())
            ));
            match_case.add_semi(format!(
                "let tag_number: {} = unsafe {{ {}::new_unchecked({}) }}",
                "proto_packet::io::TagNumber",
                "proto_packet::io::TagNumber",
                case.tag_number()
            ));
            if let Some(field_exp) = self.field_exp(case.type_tag(), false) {
                match_case.add_literal(format!("{}.encoded_len()", field_exp))
            } else {
                unimplemented!()
            }
            match_statement.add_match_case(match_case);
        }
        self.gen_encoded_len_impl(
            variant.type_name(),
            Source::default().with_statement(match_statement),
        )
    }
}

impl GenRust {
    //! `enc::EncodeToSlice`

    fn gen_variant_encode_to_slice(&self, variant: &Variant) -> ImplBlock {
        let mut match_statement: MatchStatement = MatchStatement::from("self");
        for case in variant.cases() {
            let mut match_case: MatchCase = MatchCase::from(format!(
                "Self::{}(value)",
                self.naming.case_name(case.case_name())
            ));
            match_case.add_semi(format!(
                "let tag_number: {} = unsafe {{ {}::new_unchecked({}) }}",
                "proto_packet::io::TagNumber",
                "proto_packet::io::TagNumber",
                case.tag_number()
            ));
            if let Some(field_exp) = self.field_exp(case.type_tag(), false) {
                match_case.add_literal(format!("{}.encode_to_slice_unchecked(target)", field_exp))
            } else {
                unimplemented!()
            }
            match_statement.add_match_case(match_case);
        }
        self.gen_encode_to_slice_impl(
            variant.type_name(),
            Source::default().with_statement(match_statement),
        )
    }
}

impl GenRust {
    //! `enc::EncodeToWrite`

    fn gen_variant_encode_to_write(&self, variant: &Variant) -> ImplBlock {
        let mut match_statement: MatchStatement = MatchStatement::from("self");
        for case in variant.cases() {
            let mut match_case: MatchCase = MatchCase::from(format!(
                "Self::{}(value)",
                self.naming.case_name(case.case_name())
            ));
            match_case.add_semi(format!(
                "let tag_number: {} = unsafe {{ {}::new_unchecked({}) }}",
                "proto_packet::io::TagNumber",
                "proto_packet::io::TagNumber",
                case.tag_number()
            ));
            if let Some(field_exp) = self.field_exp(case.type_tag(), false) {
                match_case.add_literal(format!("{}.encode_to_write(w)", field_exp))
            } else {
                unimplemented!()
            }
            match_statement.add_match_case(match_case);
        }
        self.gen_encode_to_write_impl(
            variant.type_name(),
            Source::default().with_statement(match_statement),
        )
    }
}
