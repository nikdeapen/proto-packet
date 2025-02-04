use code_gen::rust::{IfStatement, ImplBlock};
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet_tree::{Enum, WithTypeName};

use crate::rust::GenRust;

impl GenRust {
    //! Gen Decode

    /// Generates the decoding trait implementations for the `enom`.
    pub fn gen_enum_decode(&self, enom: &Enum) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_enum_decode_from_read(enom));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_enum_decode_from_read_prefix(enom));

        source
    }
}

impl GenRust {
    //! `enc::DecodeFromRead`

    pub fn gen_enum_decode_from_read(&self, enom: &Enum) -> ImplBlock {
        let source: Source = Source::default()
            .with_semi("use enc::DecodeFromReadPrefix")
            .with_literal("Self::decode_from_read_prefix(r)");
        self.gen_decode_from_read_impl(enom.type_name(), source)
    }
}

impl GenRust {
    //! `enc::DecodeFromReadPrefix`

    pub fn gen_enum_decode_from_read_prefix(&self, enom: &Enum) -> ImplBlock {
        self.gen_decode_from_read_prefix_impl(
            enom.type_name(),
            Source::default()
                .with_semi(
                    "let tag_number: u32 = enc::var_int::VarInt32::decode_from_read_prefix_with_first_byte(first, r)?.value",
                )
                .with_statement(
                    IfStatement::from(
                        "let Some(tag_number) = proto_packet::io::TagNumber::new(tag_number)",
                    )
                        .with_success_statements(
                            Source::default().with_literal("Ok(Self::from(tag_number))"),
                        )
                        .with_else_statements(Source::default().with_literal(format!(
                            "Err(std::io::Error::new({}, {}))",
                            "std::io::ErrorKind::InvalidData", "\"invalid tag number\""
                        ))),
                ),
        )
    }
}
