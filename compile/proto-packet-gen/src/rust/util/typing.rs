use crate::rust::BorrowMethod;
use code_gen::rust::{Reference, RustPrimitive, RustType};
use proto_packet::io::WireType;
use proto_packet::io::WireType::*;
use proto_packet_tree::{PrimitiveType, QualifiedNameRef, SpecialType, TypeTag};

/// Responsible for type conversions.
#[derive(Clone, Debug, Default)]
pub struct Typing {
    _nothing: (),
}

impl Typing {
    //! Borrowing

    /// Gets the borrow method for the `declared` type.
    pub fn borrow_method(&self, declared: &TypeTag) -> BorrowMethod {
        match declared {
            TypeTag::Primitive(_) => BorrowMethod::Copy,
            TypeTag::Special(special) => match special {
                SpecialType::Uuid => BorrowMethod::Copy,
                SpecialType::String => BorrowMethod::Deref,
            },
            TypeTag::Named(_) => BorrowMethod::Ref,
            TypeTag::Slice(_) => BorrowMethod::Deref,
        }
    }
}

impl Typing {
    //! Naming

    /// Maps the `qualified_name` to a qualified name in rust.
    pub fn rust_name(&self, qualified_name: QualifiedNameRef) -> String {
        format!("crate::{}", qualified_name.as_ref().replace(".", "::"))
    }
}

impl Typing {
    //! Wire Types

    /// Gets the `WireType` expression for the `declared`.
    pub fn wire_type_exp(&self, declared: &TypeTag, fixed: bool) -> String {
        if let Some(wire_type) = self.wire_type(declared, fixed) {
            format!("WireType::{}", wire_type)
        } else if let TypeTag::Named(name) = declared {
            format!("{}::wire_type()", self.rust_name(name.to_ref()))
        } else {
            unreachable!()
        }
    }

    /// Gets the optional `WireType` for the `declared`.
    ///
    /// Returns `None` for `Packet` types.
    fn wire_type(&self, declared: &TypeTag, fixed: bool) -> Option<WireType> {
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
            },
            TypeTag::Special(special) => match special {
                SpecialType::Uuid => Fixed16Byte,
                SpecialType::String => LengthPrefixed,
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

impl Typing {
    //! Mappings

    /// Maps the `primitive` type.
    fn primitive(&self, primitive: &PrimitiveType) -> RustType {
        match primitive {
            PrimitiveType::UnsignedInt8 => RustPrimitive::UnsignedInt8,
            PrimitiveType::UnsignedInt16 => RustPrimitive::UnsignedInt16,
            PrimitiveType::UnsignedInt32 => RustPrimitive::UnsignedInt32,
            PrimitiveType::UnsignedInt64 => RustPrimitive::UnsignedInt64,
            PrimitiveType::UnsignedInt128 => RustPrimitive::UnsignedInt128,
        }
        .to_type_tag()
    }

    /// Maps the `special` type.
    fn special(&self, special: &SpecialType) -> RustType {
        match special {
            SpecialType::Uuid => RustType::from("uuid::Uuid"),
            SpecialType::String => RustType::from("String"),
        }
    }

    /// Maps the `declared` type to its owned rust type.
    pub fn owned(&self, declared: &TypeTag) -> RustType {
        match declared {
            TypeTag::Primitive(primitive) => self.primitive(primitive),
            TypeTag::Special(special) => self.special(special),
            TypeTag::Named(name) => RustType::from(self.rust_name(name.to_ref())),
            TypeTag::Slice(base) => self.owned(base).to_vec(),
        }
    }

    /// Maps the `declared` type to its borrwed rust type.
    pub fn borrowed_type(&self, declared: &TypeTag) -> RustType {
        match declared {
            TypeTag::Primitive(_) => self.owned(declared),
            TypeTag::Special(special) => match special {
                SpecialType::String => RustType::from("str").to_ref(Reference::default()),
                _ => self.owned(declared),
            },
            TypeTag::Named(name) => {
                RustType::from(self.rust_name(name.to_ref())).to_ref(Reference::default())
            }
            TypeTag::Slice(base) => self
                .owned(base.as_ref())
                .to_slice()
                .to_ref(Reference::default()),
        }
    }
}

impl Typing {
    //! Field Types

    /// Gets the field type for the `declared` type.
    pub fn field_type(&self, declared: &TypeTag, optional: bool) -> RustType {
        let owned: RustType = self.owned(declared);
        if optional {
            owned.to_option()
        } else {
            owned
        }
    }

    /// Gets the borrowed field type for the `declared` type.
    pub fn borrowed_field_type(&self, declared: &TypeTag, optional: bool) -> RustType {
        let result: RustType = match self.borrow_method(declared) {
            BorrowMethod::Copy => self.owned(declared),
            BorrowMethod::Ref => self.borrowed_type(declared),
            BorrowMethod::Deref => self.borrowed_type(declared),
        };
        if optional {
            result.to_option()
        } else {
            result
        }
    }
}
