use proto_packet_tree::{PrimitiveType, SpecialType, TypeTag};

use crate::rust::GenRust;

impl GenRust {
    //! Fields

    /// Gets the field expression for the field with the `tag_number` and `field_type`.
    pub fn field_exp(&self, field_type: &TypeTag, fixed: bool) -> Option<String> {
        let (type_name, constructor_name) = match field_type {
            TypeTag::Primitive(primitive) => match primitive {
                PrimitiveType::UnsignedInt8 => ("Fixed1Byte", "u8"),
                PrimitiveType::UnsignedInt16 => {
                    (if fixed { "Fixed2Byte" } else { "VarInt16" }, "u16")
                }
                PrimitiveType::UnsignedInt32 => {
                    (if fixed { "Fixed4Byte" } else { "VarInt32" }, "u32")
                }
                PrimitiveType::UnsignedInt64 => {
                    (if fixed { "Fixed8Byte" } else { "VarInt64" }, "u64")
                }
                PrimitiveType::UnsignedInt128 => {
                    (if fixed { "Fixed16Byte" } else { "VarInt128" }, "u128")
                }
            },
            TypeTag::Special(special) => match special {
                SpecialType::UUID => ("Fixed16Byte", "uuid"),
                SpecialType::String => ("Bytes", "string"),
            },
            TypeTag::Named(_) => ("Packet", "packet"),
        };
        Some(format!(
            "proto_packet::io::{}Field::from_{}(tag_number, value)",
            type_name, constructor_name
        ))
    }
}
