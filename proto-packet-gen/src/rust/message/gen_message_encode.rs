use code_gen::rust::{IfStatement, ImplBlock};
use code_gen::{EmptyLine, Source, WithStatements};

use proto_packet::io::TagNumber;
use proto_packet_tree::{
    Message, MessageField, PrimitiveType, TypeTag, WithFieldName, WithTagNumberOptional,
    WithTypeName, WithTypeTag,
};

use crate::rust::EncodeOp::{EncodeToSlice, EncodeToWrite, EncodedLen};
use crate::rust::{EncodeOp, GenRust};

impl GenRust {
    //! Gen Encode

    /// Generates the encoding trait implementations for the `message`.
    pub fn gen_message_encode(&self, message: &Message) -> Source {
        let mut source: Source = Source::default();

        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_message_encoded_len(message));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_message_encode_to_slice(message));
        source.add_statement(EmptyLine::default());
        source.add_statement(self.gen_message_encode_to_write(message));

        source
    }
}

impl GenRust {
    //! `enc::EncodedLen`

    fn gen_message_encoded_len(&self, message: &Message) -> ImplBlock {
        let mut source: Source = Source::default();
        source.add_semi("let mut encoded_len: usize = 0");

        for field in message.fields() {
            source.add_statement(self.gen_message_encode_field_source(field, EncodedLen))
        }

        source.add_statement(EmptyLine::default());
        source.add_literal("Ok(encoded_len)");
        self.gen_encoded_len_impl(message.type_name(), source)
    }
}

impl GenRust {
    //! `enc::EncodeToSlice`

    fn gen_message_encode_to_slice(&self, message: &Message) -> ImplBlock {
        let mut source: Source = Source::default();
        source.add_semi("let mut encoded_len: usize = 0");
        source.add_statement(EmptyLine::default());

        for field in message.fields().iter() {
            source.add_statement(self.gen_message_encode_field_source(field, EncodeToSlice))
        }

        source.add_statement(EmptyLine::default());
        source.add_literal("Ok(encoded_len)");
        self.gen_encode_to_slice_impl(message.type_name(), source)
    }
}

impl GenRust {
    //! `enc::EncodeToWrite`

    fn gen_message_encode_to_write(&self, message: &Message) -> ImplBlock {
        let mut source: Source = Source::default();
        source.add_semi("let mut encoded_len: usize = 0");

        for field in message.fields().iter() {
            source.add_statement(self.gen_message_encode_field_source(field, EncodeToWrite))
        }

        source.add_statement(EmptyLine::default());
        source.add_literal("Ok(encoded_len)");
        self.gen_encode_to_write_impl(message.type_name(), source)
    }
}

impl GenRust {
    //! Gen Encode Message Field Source

    /// Generates the source code for encoding the message field.
    fn gen_message_encode_field_source(&self, field: &MessageField, op: EncodeOp) -> Source {
        if let Some(tag_number) = field.tag_number() {
            Source::default()
                .with_statement(EmptyLine::default())
                .with_statement(
                    IfStatement::from(format!(
                        "let Some(value) = &self.{}",
                        self.naming.field_name(field.field_name())
                    ))
                    .with_success_statements(
                        self.gen_encode_message_field_with_tag_number_and_value(
                            field, op, tag_number,
                        ),
                    ),
                )
        } else {
            unimplemented!("missing tag numbers is messages not yet supported")
        }
    }

    /// Generates the source code for encoding the message field.
    ///
    /// This function assumes the `field` has a `TagNumber` and a non-optional `value` variable.
    fn gen_encode_message_field_with_tag_number_and_value(
        &self,
        field: &MessageField,
        op: EncodeOp,
        tag_number: TagNumber,
    ) -> Source {
        let mut source: Source = Source::default();

        source.add_semi(format!(
            "let tag_number: TagNumber = unsafe {{ TagNumber::new_unchecked({}) }}",
            tag_number
        ));
        source.add_statement(self.gen_message_encode_field_len(field, op));
        source.add_semi(format!(
            "encoded_len = encoded_len.checked_add(field_len).ok_or({})?",
            "enc::Error::IntegerOverflow"
        ));

        source
    }

    /// Generates the code to increment generate `field_len` variable.
    fn gen_message_encode_field_len(&self, field: &MessageField, op: EncodeOp) -> Source {
        if let Some(field_exp) = self.field_exp(field.type_tag(), false) {
            Source::default().with_semi(format!(
                "let field_len: usize = {}.{}?",
                field_exp,
                op.encode_call()
            ))
        } else {
            match field.type_tag() {
                TypeTag::Slice(base) => self.gen_message_encode_slice_field_len(op, base),
                _ => unimplemented!("no field expression for type: {:?}", field.type_tag()),
            }
        }
    }

    fn gen_message_encode_slice_field_len(&self, op: EncodeOp, base: &TypeTag) -> Source {
        let encode_tag: &str = match base {
            TypeTag::Primitive(primitive) => match primitive {
                PrimitiveType::UnsignedInt8 => "u8",
                _ => unimplemented!(),
            },
            TypeTag::Named(_) => "packet",
            _ => unimplemented!(),
        };
        let list_wire_type: String = match base {
            TypeTag::Primitive(primitive) => match primitive {
                PrimitiveType::UnsignedInt8 => "WireType::Fixed1Byte".to_string(),
                _ => unimplemented!(),
            },
            TypeTag::Named(name) => {
                format!("{}::wire_type()", self.typing.rust_name(name.to_ref()))
            }
            _ => unimplemented!(),
        };
        Source::default()
            .with_semi(format!(
                "let field_header_len: usize = {}.{}?",
                "FieldHeader::new(WireType::List, tag_number)",
                op.encode_call(),
            ))
            .with_semi("encoded_len = encoded_len.checked_add(field_header_len).ok_or(enc::Error::IntegerOverflow)?")
            .with_semi(format!(
                "let list_size_bytes: usize = proto_packet::io::encoded_len_slice_{}(value)?",
                encode_tag,
            ))
            .with_semi(format!(
                "let list_header_len: usize = proto_packet::io::ListHeader::new({}, list_size_bytes).{}?",
                list_wire_type,
                op.encode_call(),
            ))
            .with_semi("encoded_len = encoded_len.checked_add(list_header_len).ok_or(enc::Error::IntegerOverflow)?")
            .with_semi(format!(
                "let also_list_size_bytes: usize = proto_packet::io::{}_slice_{}(value{})?",
                op.encode_tag(),
                encode_tag,
                op.encode_extra_params()
            ))
            .with_semi("debug_assert_eq!(list_size_bytes, also_list_size_bytes)")
            .with_semi("let field_len: usize = list_size_bytes")
    }
}
