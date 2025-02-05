use code_gen::rust::{ImplBlock, MatchCase, MatchStatement, VarInit};
use code_gen::{EmptyLine, Literal, Source, WithStatements};

use proto_packet_tree::{
    PrimitiveType, SpecialType, TypeTag, Variant, WithCaseName, WithTypeName, WithTypeTag,
};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Variant Decode

    /// Generates the decoding trait implementations for the `variant`.
    pub fn gen_variant_decode(&self, variant: &Variant) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_variant_decode_from_read(variant));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_variant_decode_from_read_prefix(variant));

        source
    }
}

impl GenRust {
    //! `enc::DecodeFromRead`

    pub fn gen_variant_decode_from_read(&self, variant: &Variant) -> ImplBlock {
        let mut source: Source = Source::default();

        source.add_semi("let field_header: FieldHeader = FieldHeader::decode_from_read_prefix(r)?");

        let mut match_statement: MatchStatement =
            MatchStatement::from("field_header.tag_number().tag_number()");
        for case in variant.cases() {
            let mut match_case: MatchCase = MatchCase::from(case.tag_number().to_string());

            let decode_tag: &str = match case.type_tag() {
                TypeTag::Primitive(primitive) => match primitive {
                    PrimitiveType::UnsignedInt8 => "u8",
                    PrimitiveType::UnsignedInt16 => "u16",
                    PrimitiveType::UnsignedInt32 => "u32",
                    PrimitiveType::UnsignedInt64 => "u64",
                    PrimitiveType::UnsignedInt128 => "u128",
                },
                TypeTag::Special(special) => match special {
                    SpecialType::UUID => "uuid",
                    SpecialType::String => "string",
                },
                TypeTag::Named(_) => "packet",
                TypeTag::Slice(_base) => unimplemented!(),
            };

            match_case.add_statement(VarInit::from((
                ("value", self.typing.field_type(case.type_tag())),
                Literal::from(format!(
                    "proto_packet::io::decode_{}(field_header.wire_type(), r)?",
                    decode_tag
                )),
            )));

            match_case.add_literal(format!(
                "Ok(Self::{}(value))",
                self.naming.case_name(case.case_name()),
            ));

            match_statement.add_match_case(match_case);
        }

        match_statement.add_match_case(MatchCase::from("_").with_literal("unimplemented!()"));

        source.add_statement(match_statement);

        self.gen_decode_from_read_impl(variant.type_name(), source)
    }
}

impl GenRust {
    //! `enc::DecodeFromReadPrefix`

    pub fn gen_variant_decode_from_read_prefix(&self, variant: &Variant) -> ImplBlock {
        self.gen_decode_from_read_prefix_impl(
            variant.type_name(),
            Source::default()
                .with_semi("use enc::DecodeFromRead")
                .with_literal("Self::decode_from_read_length_prefixed_with_first_byte(first, r)"),
        )
    }
}
