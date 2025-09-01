use crate::rust::{EncodeOp, GenRust};
use code_gen::{Source, WithStatements};
use proto_packet::io::WireType::{
    Fixed16Byte, Fixed1Byte, Fixed2Byte, Fixed4Byte, Fixed8Byte, LengthPrefixed, List, VarInt,
};
use proto_packet::io::{TagNumber, WireType};
use proto_packet_tree::{PrimitiveType, SpecialType, TypeTag};

impl GenRust {
    //! Gen Encode Field

    /// Generates the source to encode a field.
    ///
    /// # Expects in Scope
    ///   `field_exp: &T`
    ///   `encoded_len: usize`
    ///   `target: &mut [u8]`         (if `op == EncodeToSlice`)
    ///   `w: &mut std::io::Write`    (if `op == EncodeToWrite`)
    pub(in crate::rust) fn gen_encode_field(
        &self,
        field_exp: &str,
        type_tag: &TypeTag,
        fixed: bool,
        tag_number: TagNumber,
        op: EncodeOp,
    ) -> Source {
        Source::default()
            .with_literal("encoded_len += {")
            .with_semi(format!(
                "let tag_number: {} = unsafe {{ {}::new_unchecked({}) }}",
                "proto_packet::io::TagNumber", "proto_packet::io::TagNumber", tag_number
            ))
            .with_semi(format!(
                "let header: {} = {}::new({}, {})",
                "proto_packet::io::FieldHeader",
                "proto_packet::io::FieldHeader",
                self.gen_wire_type_exp(type_tag, fixed),
                "tag_number"
            ))
            .with_literal(format!("header.{}?", op.encode_call()))
            .with_semi("}")
            .with_statement(self.gen_encode_value(field_exp, type_tag, fixed, op))
    }

    fn gen_wire_type_exp(&self, type_tag: &TypeTag, fixed: bool) -> String {
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
                PrimitiveType::SignedInt8 => Fixed1Byte,
                PrimitiveType::SignedInt16 => {
                    if fixed {
                        Fixed2Byte
                    } else {
                        VarInt
                    }
                }
                PrimitiveType::SignedInt32 => {
                    if fixed {
                        Fixed4Byte
                    } else {
                        VarInt
                    }
                }
                PrimitiveType::SignedInt64 => {
                    if fixed {
                        Fixed8Byte
                    } else {
                        VarInt
                    }
                }
                PrimitiveType::SignedInt128 => {
                    if fixed {
                        Fixed16Byte
                    } else {
                        VarInt
                    }
                }
                PrimitiveType::Bool => Fixed1Byte,
            },
            TypeTag::Special(special) => match special {
                SpecialType::Uuid => Fixed16Byte,
                SpecialType::String => LengthPrefixed,
                SpecialType::Date => {
                    if fixed {
                        Fixed4Byte
                    } else {
                        VarInt
                    }
                }
            },
            TypeTag::Named(name) => {
                return format!("{}::wire_type()", self.typing.rust_name(name.to_ref()))
            }
            TypeTag::Slice(base) => match base.as_ref() {
                TypeTag::Primitive(primitive) => match primitive {
                    PrimitiveType::UnsignedInt8 => LengthPrefixed,
                    _ => List,
                },
                TypeTag::Special(_) => List,
                TypeTag::Named(_) => List,
                TypeTag::Slice(_) => List,
            },
        };
        format!("WireType::{}", wire_type)
    }
}
