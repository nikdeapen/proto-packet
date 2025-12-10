use crate::rust::util::EncodeOp;
use crate::rust::GenRust;
use code_gen::{Source, WithStatements};
use proto_packet::io::WireType::{
    Fixed16Byte, Fixed1Byte, Fixed2Byte, Fixed4Byte, Fixed8Byte, LengthPrefixed, VarInt,
};
use proto_packet::io::{TagNumber, WireType};
use proto_packet_tree::{PrimitiveType, SpecialType, TypeTag};

impl GenRust {
    //! Gen Encode Field

    /// Generates the source to encode a field.
    ///
    /// # Expects in Scope
    ///   `encoded_len: usize`
    ///   `target: &mut [u8]`         (if `op == EncodeToSlice`)
    ///   `w: &mut std::io::Write`    (if `op == EncodeToWrite`)
    pub(in crate::rust) fn gen_encode_field(
        &self,
        value_exp: &str,
        type_tag: &TypeTag,
        fixed: bool,
        tag_number: TagNumber,
        op: EncodeOp,
    ) -> Source {
        Source::default()
            .with_semi(format!(
                "let tag: proto_packet::io::TagNumber = unsafe {{ proto_packet::io::TagNumber::new_unchecked({}) }}",
                tag_number
            ))
            .with_semi(format!(
                "let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new({}, {})",
                self.gen_wire_type_exp(type_tag, fixed),
                "tag"
            ))
            .with_semi(format!("encoded_len += header.{}?", op.encode_call()))
            .with_statement(self.gen_encode_value(value_exp, type_tag, fixed, op))
    }

    pub fn gen_wire_type_exp(&self, type_tag: &TypeTag, fixed: bool) -> String {
        let wire_type: WireType = match type_tag {
            TypeTag::Primitive(primitive) => match primitive {
                PrimitiveType::UnsignedInt8 => Fixed1Byte,
                PrimitiveType::UnsignedInt16 => {
                    if fixed {
                        Fixed2Byte
                    } else {
                        VarInt
                    }
                }
                PrimitiveType::UnsignedInt32 => {
                    if fixed {
                        Fixed4Byte
                    } else {
                        VarInt
                    }
                }
                PrimitiveType::UnsignedInt64 => {
                    if fixed {
                        Fixed8Byte
                    } else {
                        VarInt
                    }
                }
                PrimitiveType::UnsignedInt128 => {
                    if fixed {
                        Fixed16Byte
                    } else {
                        VarInt
                    }
                }
            },
            TypeTag::Special(special) => match special {
                SpecialType::Uuid => Fixed16Byte,
                SpecialType::String => LengthPrefixed,
            },
            TypeTag::Named(name) => {
                return format!("{}::wire_type()", self.typing.rust_name(name.to_ref()))
            }
        };
        format!("proto_packet::io::WireType::{}", wire_type)
    }
}
