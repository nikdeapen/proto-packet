use crate::rust::util::Typing;
use proto_packet::io::WireType;
use proto_packet::io::WireType::{
    Fixed16Byte, Fixed1Byte, Fixed2Byte, Fixed4Byte, Fixed8Byte, LengthPrefixed, VarInt,
};
use proto_packet_tree::{PrimitiveType, SpecialType, TypeTag};

impl Typing {
    //! Wire Type

    /// Gets the `WireType` expression for the `declared` type.
    pub fn wire_type_exp(&self, declared: &TypeTag, fixed: bool) -> String {
        if let Some(wire_type) = self.wire_type(declared, fixed) {
            format!("proto_packet::io::WireType::{}", wire_type)
        } else if let TypeTag::Named(name) = declared {
            format!("{}::wire_type()", self.rust_name(name.to_ref()))
        } else if let TypeTag::List(base) = declared {
            match base.as_ref() {
                TypeTag::Primitive(base) => match base {
                    PrimitiveType::UnsignedInt8 => {
                        "proto_packet::io::WireType::LengthPrefixed".to_string()
                    }
                    _ => "proto_packet::io::WireType::List".to_string(),
                },
                _ => "proto_packet::io::WireType::List".to_string(),
            }
        } else {
            unreachable!()
        }
    }

    /// Gets the optional `WireType` for the `declared` type.
    ///
    /// Returns `None` for `Packet` types.
    pub(in crate::rust) fn wire_type(&self, declared: &TypeTag, fixed: bool) -> Option<WireType> {
        Some(match declared {
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
            },
            TypeTag::Special(special) => match special {
                SpecialType::Uuid => Fixed16Byte,
                SpecialType::String => LengthPrefixed,
            },
            TypeTag::Named(_) => return None,
            TypeTag::List(_) => return None,
        })
    }
}
