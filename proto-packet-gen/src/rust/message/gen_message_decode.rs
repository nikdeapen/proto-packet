use code_gen::rust::{ImplBlock, MatchCase, MatchStatement, VarInit, WhileLoop};
use code_gen::{EmptyLine, Literal, Source, WithStatements};

use proto_packet_tree::{
    Message, PrimitiveType, SpecialType, TypeTag, WithFieldName, WithTagNumberOptional,
    WithTypeName, WithTypeTag,
};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Decode

    /// Generates the decoding trait implementations for the `message`.
    pub fn gen_message_decode(&self, message: &Message) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_message_decode_from_read(message));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_message_decode_from_read_prefix(message));

        source
    }
}

impl GenRust {
    //! `enc::DecodeFromRead`

    pub fn gen_message_decode_from_read(&self, message: &Message) -> ImplBlock {
        let mut source: Source = Source::default();
        source.add_semi("let mut result: Self = Self::default()");
        source.add_statement(EmptyLine::default());

        let mut while_loop: WhileLoop = WhileLoop::from(Literal::from(
            "let Some(first) = enc::read_optional_byte(r)?",
        ));
        while_loop.add_semi("use enc::DecodeFromReadPrefix");
        while_loop.add_semi(format!(
            "let field_header: {} = {}::decode_from_read_prefix_with_first_byte(first, r)?",
            "FieldHeader", "FieldHeader"
        ));
        while_loop.add_semi("let tag_number: u32 = field_header.tag_number().tag_number()");

        let mut match_tag: MatchStatement = MatchStatement::from("tag_number");
        for field in message.fields() {
            if let Some(tag_number) = field.tag_number() {
                let mut match_case: MatchCase = MatchCase::from(tag_number.to_string());

                let decode_tag: &str = match field.type_tag() {
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
                    TypeTag::Slice(_) => "slice_u8",
                };

                match_case.add_statement(VarInit::from((
                    ("value", self.typing.field_type(field.type_tag())),
                    Literal::from(format!(
                        "proto_packet::io::decode_{}(field_header.wire_type(), r)?",
                        decode_tag
                    )),
                )));
                match_case.add_semi(format!(
                    "result.set_{}(value)",
                    self.naming.field_name(field.field_name())
                ));

                match_tag.add_match_case(match_case);
            }
        }
        let default_case: MatchCase = MatchCase::from("_");
        match_tag.add_match_case(default_case);

        while_loop.add_statement(match_tag);
        source.add_statement(while_loop);

        source.add_statement(EmptyLine::default());
        source.add_literal("Ok(result)");
        self.gen_decode_from_read_impl(message.type_name(), source)
    }
}

impl GenRust {
    //! `enc::DecodeFromReadPrefix`

    pub fn gen_message_decode_from_read_prefix(&self, message: &Message) -> ImplBlock {
        self.gen_decode_from_read_prefix_impl(
            message.type_name(),
            Source::default()
                .with_semi("use DecodeFromRead")
                .with_literal("Self::decode_from_read_length_prefixed_with_first_byte(first, r)"),
        )
    }
}
