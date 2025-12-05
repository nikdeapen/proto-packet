use crate::rust::{GenRust, Naming};
use code_gen::rust::{MatchCase, MatchStatement};
use code_gen::{Source, WithStatements};
use proto_packet_tree::{TypeTag, Variant, WithCaseName, WithTagNumber, WithTypeName, WithTypeTag};

impl GenRust {
    //! Gen Variant: Decode

    pub(in crate::rust::gen_packet::gen_variant) fn gen_variant_decode(
        &self,
        v: &Variant,
    ) -> Source {
        Source::default()
            .with_empty_line()
            .with_statement(
                self.gen_decode_from_read_impl(v.type_name(), self.gen_decode_variant_source(v)),
            )
            .with_empty_line()
            .with_semi(format!(
                "enc::impl_decode_from_read_prefix_length_prefixed!({})",
                self.naming.type_name(v)
            ))
    }

    fn gen_decode_variant_source(&self, v: &Variant) -> Source {
        let mut source: Source = Source::default();
        if v.cases()
            .iter()
            .any(|c| matches!(c.type_tag(), TypeTag::Named(_)))
        {
            source.add_semi("use proto_packet::Packet;");
        }
        source.add_semi("use enc::DecodeFromReadPrefix;");
        if !v.cases().is_empty() {
            source.add_semi("use proto_packet::io::Decoder;");
        }
        source.add_empty_line();

        source.add_semi(format!(
            "let header: {} = {}::decode_from_read_prefix(r)?",
            Naming::FIELD_HEADER,
            Naming::FIELD_HEADER
        ));
        let mut match_statement: MatchStatement = MatchStatement::from("header.tag().value()");
        for case in v.cases() {
            match_statement.add_match_case(
                MatchCase::from(case.tag_number().to_string())
                    .with_statement(self.gen_decode_value(
                        "value",
                        case.case_name(),
                        case.type_tag(),
                        false,
                        Some(case.tag_number()),
                    ))
                    .with_literal(format!("Ok(Self::{}(value))", self.naming.case_name(case))),
            )
        }
        match_statement.add_match_case(
            MatchCase::from("_").with_statement(self.gen_decode_variant_unrecognized(v)),
        );
        source.add_statement(match_statement);

        source
    }

    fn gen_decode_variant_unrecognized(&self, v: &Variant) -> Source {
        Source::default()
            .with_semi("use std::io::Cursor")
            .with_semi("use enc::EncodeToWrite")
            .with_empty_line()
            .with_semi("let serial: Vec<u8> = Vec::default()")
            .with_semi("let mut serial: Cursor<Vec<u8>> = Cursor::new(serial)")
            .with_semi("header.encode_to_write(&mut serial)?")
            .with_semi("header.wire().transfer(r, &mut serial)?")
            .with_semi("let serial: Vec<u8> = serial.into_inner()")
            .with_literal(format!(
                "Ok(Self::{}({} {{ serial }}))",
                Naming::UNRECOGNIZED_CASE_NAME,
                self.naming.unrecognized_case_type_name(v)
            ))
    }
}
