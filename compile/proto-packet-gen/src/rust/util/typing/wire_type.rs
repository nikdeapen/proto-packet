use crate::rust::Typing;
use proto_packet::io::WireType;
use proto_packet::io::WireType::{
    Fixed16Byte, Fixed1Byte, Fixed2Byte, Fixed4Byte, Fixed8Byte, LengthPrefixed, List, VarInt,
};
use proto_packet_tree::{PrimitiveType, SpecialType, TypeTag};

impl Typing {
    //! Wire Type

    /// Gets the `WireType` expression for the `declared` type.
    pub fn wire_type_exp(&self, declared: &TypeTag, fixed: bool) -> String {
        if let Some(wire_type) = self.wire_type(declared, fixed) {
            format!("WireType::{}", wire_type)
        } else if let TypeTag::Named(name) = declared {
            format!("{}::wire_type()", self.rust_name(name.to_ref()))
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
                SpecialType::Date => {
                    if fixed {
                        Fixed8Byte
                    } else {
                        VarInt
                    }
                }
            },
            TypeTag::Named(_) => return None,
            TypeTag::Slice(base) => match base.as_ref() {
                TypeTag::Primitive(primitive) => match primitive {
                    PrimitiveType::UnsignedInt8 => LengthPrefixed,
                    _ => List,
                },
                _ => List,
            },
        })
    }
}
