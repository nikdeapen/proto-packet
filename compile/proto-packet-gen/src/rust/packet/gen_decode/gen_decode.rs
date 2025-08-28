use crate::rust::GenRust;
use code_gen::rust::{MatchCase, MatchStatement, WhileLoop};
use code_gen::{EmptyLine, Literal, Source, WithStatements};
use proto_packet_tree::{
    Enum, Message, ModPathRef, Struct, Variant, WithCaseName, WithFieldName, WithTagNumber,
    WithTypeName, WithTypeTag,
};

impl GenRust {
    //! Gen Decode

    pub(in crate::rust) fn gen_decode_struct(&self, mod_path: ModPathRef, s: &Struct) -> Source {
        self.gen_decode(
            s,
            self.gen_decode_from_read_source_struct(mod_path, s),
            self.gen_decode_from_read_prefix_by_length_prefix_source(),
        )
    }

    pub(in crate::rust) fn gen_decode_message(&self, mod_path: ModPathRef, m: &Message) -> Source {
        self.gen_decode(
            m,
            self.gen_decode_from_read_source_message(mod_path, m),
            self.gen_decode_from_read_prefix_by_length_prefix_source(),
        )
    }

    pub(in crate::rust) fn gen_decode_enum(&self, mod_path: ModPathRef, e: &Enum) -> Source {
        self.gen_decode(
            e,
            self.gen_decode_from_read_source_enum(),
            self.gen_decode_from_read_prefix_source_enum(mod_path, e),
        )
    }

    pub(in crate::rust) fn gen_decode_variant(&self, mod_path: ModPathRef, v: &Variant) -> Source {
        self.gen_decode(
            v,
            self.gen_decode_from_read_source_variant(mod_path, v),
            self.gen_decode_from_read_prefix_by_length_prefix_source(),
        )
    }
}

impl GenRust {
    //! Gen Decode: Struct

    fn gen_decode_from_read_source_struct(&self, _mod_path: ModPathRef, s: &Struct) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());

        for field in s.fields() {
            source.add_statement(EmptyLine::default());
            source.add_statement(self.gen_decode_value(
                format!("decoded_{}", self.naming.field_name(field.field_name())).as_str(),
                field.field_name(),
                field.type_tag(),
                false,
                None,
            ));
        }

        source.add_statement(EmptyLine::default());
        source.add_semi("debug_assert!(enc::read_optional_byte(r)?.is_none())");

        source.add_statement(EmptyLine::default());
        source.add_literal("Ok(Self {");
        for field in s.fields() {
            source.add_literal(format!(
                "    {}: decoded_{},",
                self.naming.field_name(field.field_name()),
                self.naming.field_name(field.field_name()),
            ));
        }
        source.add_literal("})");

        source
    }
}

impl GenRust {
    //! Gen Decode: Message

    fn gen_decode_from_read_source_message(&self, _mod_path: ModPathRef, m: &Message) -> Source {
        let mut source: Source = Source::default();
        source.add_semi("let mut result: Self = Self::default()");
        source.add_statement(EmptyLine::default());

        source.add_statement(EmptyLine::default());
        let mut while_loop: WhileLoop = WhileLoop::from(Literal::from(
            "let Some(first) = enc::read_optional_byte(r)?",
        ));
        while_loop.add_semi(format!(
            "let header: {} = {}::{}(r, first)?",
            self.naming.field_header_type_name,
            self.naming.field_header_type_name,
            "decode_from_read_prefix_with_first_byte"
        ));
        let mut match_statement: MatchStatement =
            MatchStatement::from("header.tag_number().value()");
        for field in m.fields() {
            let match_case: MatchCase = MatchCase::from(field.tag_number().to_string())
                .with_statement(self.gen_decode_value(
                    "value",
                    field.field_name(),
                    field.type_tag(),
                    false,
                    Some(field.tag_number()),
                ))
                .with_semi(format!(
                    "result.set_{}(value)",
                    self.naming.field_name(field.field_name())
                ));
            match_statement.add_match_case(match_case);
        }
        match_statement.add_match_case(
            MatchCase::from("_")
                .with_semi(format!(
                    "let mut w: {}<&mut Vec<u8>> = {}::new(&mut result.{})",
                    self.naming.cursor_type_name,
                    self.naming.cursor_type_name,
                    self.naming.unrecognized_fields_name,
                ))
                .with_semi("header.encode_to_write(&mut w)?")
                .with_semi("header.wire_type().transfer(r, &mut w)?"),
        );
        while_loop.add_statement(match_statement);
        source.add_statement(while_loop);

        source.add_literal("Ok(result)");

        source
    }
}

impl GenRust {
    //! Gen Decode: Enum

    fn gen_decode_from_read_source_enum(&self) -> Source {
        Source::default()
            .with_semi("let first: u8 = enc::read_single_byte(r)?")
            .with_semi("let value: Self = Self::decode_from_read_prefix_with_first_byte(r, first)?")
            .with_semi("debug_assert!(enc::read_optional_byte(r)?.is_none())")
            .with_literal("Ok(value)")
    }

    fn gen_decode_from_read_prefix_source_enum(&self, _mod_path: ModPathRef, e: &Enum) -> Source {
        let mut match_statement: MatchStatement = MatchStatement::from("tag_number");
        for case in e.cases() {
            let match_case: MatchCase = MatchCase::from(case.tag_number().to_string())
                .with_literal(format!(
                    "Ok(Self::{})",
                    self.naming.case_name(case.case_name())
                ));
            match_statement.add_match_case(match_case);
        }
        match_statement.add_match_case(MatchCase::from("_").with_literal("todo!()"));

        Source::default()
            .with_semi(format!(
                "let tag_number: u32 = {}::{}(r, first)?.value()",
                "enc::var_int::VarInt32", "decode_from_read_prefix_with_first_byte"
            ))
            .with_statement(match_statement)
    }
}

impl GenRust {
    //! Gen Decode: Variant

    fn gen_decode_from_read_source_variant(&self, _mod_path: ModPathRef, v: &Variant) -> Source {
        let mut match_statement: MatchStatement =
            MatchStatement::from("header.tag_number().value()");
        for case in v.cases() {
            let match_case: MatchCase = MatchCase::from(case.tag_number().to_string())
                .with_statement(self.gen_decode_value(
                    "value",
                    case.case_name(),
                    case.type_tag(),
                    false,
                    Some(case.tag_number()),
                ))
                .with_literal(format!(
                    "Ok(Self::{}(value))",
                    self.naming.case_name(case.case_name())
                ));
            match_statement.add_match_case(match_case)
        }
        match_statement.add_match_case(MatchCase::from("_").with_literal("todo!()"));

        Source::default()
            .with_semi("let first: u8 = enc::read_single_byte(r)?")
            .with_semi(format!(
                "let header: {} = {}::decode_from_read_prefix_with_first_byte(r, first)?",
                self.naming.field_header_type_name, self.naming.field_header_type_name
            ))
            .with_statement(match_statement)
    }
}

impl GenRust {
    //! Gen Decode: Utils

    fn gen_decode<T>(&self, element: &T, from_read: Source, from_prefix: Source) -> Source
    where
        T: WithTypeName,
    {
        Source::default()
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_decode_from_read_impl(element.type_name(), from_read))
            .with_statement(EmptyLine::default())
            .with_statement(self.gen_decode_from_read_prefix_impl(element.type_name(), from_prefix))
    }

    fn gen_decode_from_read_prefix_by_length_prefix_source(&self) -> Source {
        Source::default()
            .with_literal("Self::decode_from_read_length_prefixed_with_first_byte(r, first)")
    }
}
