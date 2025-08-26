use crate::rust::EncodeOp::{EncodeToSlice, EncodeToWrite, EncodedLen};
use crate::rust::{EncodeOp, GenRust};
use code_gen::rust::{IfStatement, MatchCase, MatchStatement, RustPrimitive};
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet_tree::{
    Enum, Message, Struct, TypeNameRef, Variant, WithCaseName, WithFieldName, WithTagNumber,
    WithTypeName, WithTypeTag,
};

impl GenRust {
    //! Gen Encode

    pub(in crate::rust) fn gen_encode_struct(&self, s: &Struct) -> Source {
        self.gen_encode(s, |s, op| self.gen_encode_struct_source(s, op))
    }

    pub(in crate::rust) fn gen_encode_message(&self, m: &Message) -> Source {
        self.gen_encode(m, |m, op| self.gen_encode_message_source(m, op))
    }

    pub(in crate::rust) fn gen_encode_enum(&self, e: &Enum) -> Source {
        self.gen_encode(e, |e, op| self.gen_encode_enum_source(e, op))
    }

    pub(in crate::rust) fn gen_encode_variant(&self, v: &Variant) -> Source {
        self.gen_encode(v, |v, op| self.gen_encode_variant_source(v, op))
    }

    pub(in crate::rust) fn gen_encode<T, F>(&self, element: &T, encode: F) -> Source
    where
        T: WithTypeName,
        F: Fn(&T, EncodeOp) -> Source,
    {
        let name: TypeNameRef = element.type_name();
        Source::default()
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_encoded_len_impl(name, encode(element, EncodedLen)))
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_encode_to_slice_impl(name, encode(element, EncodeToSlice)))
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_encode_to_write_impl(name, encode(element, EncodeToWrite)))
    }
}

impl GenRust {
    //! Gen Encode: Source

    pub(in crate::rust) fn gen_encode_struct_source(&self, s: &Struct, op: EncodeOp) -> Source {
        let mut source: Source = Source::default();
        source.add_semi("let mut encoded_len: usize = 0;");

        for field in s.fields() {
            source.add_statement(EmptyLine::default());
            source.add_statement(self.gen_encode_value(
                format!("&self.{}", self.naming.field_name(field.field_name())).as_str(),
                field.type_tag(),
                false, // todo -- fixed
                op,
            ));
        }

        source.add_statement(EmptyLine::default());
        source.add_literal("Ok(encoded_len)");
        source
    }

    pub(in crate::rust) fn gen_encode_message_source(&self, s: &Message, op: EncodeOp) -> Source {
        let mut source: Source = Source::default();
        source.add_semi("let mut encoded_len: usize = 0;");

        for field in s.fields() {
            source.add_statement(EmptyLine::default());
            source.add_statement(
                IfStatement::from(format!(
                    "let Some(value) = &self.{}",
                    self.naming.field_name(field.field_name())
                ))
                .with_success_statements(Source::default().with_statement(
                    self.gen_encode_field(
                        "value",
                        field.type_tag(),
                        false, // todo -- fixed
                        field.tag_number(),
                        op,
                    ),
                )),
            );
        }

        source.add_statement(EmptyLine::default());
        source.add_literal("Ok(encoded_len)");
        source
    }

    pub(in crate::rust) fn gen_encode_enum_source(&self, e: &Enum, op: EncodeOp) -> Source {
        let mut source: Source = Source::default();
        source.add_semi("let mut encoded_len: usize = 0;");

        let mut match_statement: MatchStatement = MatchStatement::from("self")
            .with_assignment(("tag_number", RustPrimitive::UnsignedInt32));
        for case in e.cases() {
            match_statement.add_match_case(
                MatchCase::from(format!("Self::{}", self.naming.case_name(case.case_name())))
                    .with_literal(case.tag_number().to_string()),
            )
        }
        source.add_statement(match_statement);

        source.add_semi(format!(
            "encoded_len += {}::from(tag_number).{}?",
            "enc::var_int::VarInt32",
            op.encode_call()
        ));
        source.add_literal("Ok(encoded_len)");

        source
    }

    pub(in crate::rust) fn gen_encode_variant_source(&self, v: &Variant, op: EncodeOp) -> Source {
        let mut source: Source = Source::default();
        source.add_semi("let mut encoded_len: usize = 0;");

        let mut match_statement: MatchStatement = MatchStatement::from("self");
        for case in v.cases() {
            match_statement.add_match_case(
                MatchCase::from(format!(
                    "Self::{}(value)",
                    self.naming.case_name(case.case_name())
                ))
                .with_statement(self.gen_encode_field(
                    "value",
                    case.type_tag(),
                    false, // todo -- fixed
                    case.tag_number(),
                    op,
                )),
            )
        }
        source.add_statement(match_statement);

        source.add_literal("Ok(encoded_len)");
        source
    }
}
