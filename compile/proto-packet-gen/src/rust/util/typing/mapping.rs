use crate::rust::util::Typing;
use crate::rust::BorrowMethod;
use code_gen::rust::{Reference, RustPrimitive, RustType};
use proto_packet_tree::{PrimitiveType, SpecialType, TypeTag};

impl Typing {
    //! Owned Type

    /// Maps the `declared` type to its owned rust type.
    pub fn owned(&self, declared: &TypeTag) -> RustType {
        match declared {
            TypeTag::Primitive(primitive) => self.owned_primitive(primitive),
            TypeTag::Special(special) => self.owned_special(special),
            TypeTag::Named(name) => RustType::from(self.rust_name(name.to_ref())),
            TypeTag::List(base) => RustType::from("Vec").with_generic(self.owned(base)),
        }
    }

    /// Maps the `primitive` type to its owned rust type.
    fn owned_primitive(&self, primitive: &PrimitiveType) -> RustType {
        match primitive {
            PrimitiveType::UnsignedInt8 => RustPrimitive::UnsignedInt8,
            PrimitiveType::UnsignedInt16 => RustPrimitive::UnsignedInt16,
            PrimitiveType::UnsignedInt32 => RustPrimitive::UnsignedInt32,
            PrimitiveType::UnsignedInt64 => RustPrimitive::UnsignedInt64,
            PrimitiveType::UnsignedInt128 => RustPrimitive::UnsignedInt128,
            PrimitiveType::SignedInt8 => RustPrimitive::SignedInt8,
            PrimitiveType::SignedInt16 => RustPrimitive::SignedInt16,
            PrimitiveType::SignedInt32 => RustPrimitive::SignedInt32,
            PrimitiveType::SignedInt64 => RustPrimitive::SignedInt64,
            PrimitiveType::SignedInt128 => RustPrimitive::SignedInt128,
        }
        .to_type_tag()
    }

    /// Maps the `special` type to its owned rust type.
    fn owned_special(&self, special: &SpecialType) -> RustType {
        match special {
            SpecialType::Uuid => RustType::from("uuid::Uuid"),
            SpecialType::String => RustType::from("String"),
        }
    }
}

impl Typing {
    //! Borrowed Type

    /// Maps the `declared` type to its borrowed rust type.
    pub fn borrowed_type(&self, declared: &TypeTag) -> RustType {
        match declared {
            TypeTag::Primitive(_) => self.owned(declared),
            TypeTag::Special(special) => match special {
                SpecialType::String => RustType::from("&str"),
                _ => self.owned(declared),
            },
            TypeTag::Named(name) => {
                RustType::from(self.rust_name(name.to_ref())).to_ref(Reference::default())
            }
            TypeTag::List(base) => self.owned(base).to_slice().to_ref(Reference::default()),
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

    /// Gets the borrow method for the `declared` type.
    pub fn borrow_method(&self, declared: &TypeTag) -> BorrowMethod {
        match declared {
            TypeTag::Primitive(_) => BorrowMethod::Copy,
            TypeTag::Special(special) => match special {
                SpecialType::Uuid => BorrowMethod::Copy,
                SpecialType::String => BorrowMethod::Deref,
            },
            TypeTag::Named(_) => BorrowMethod::Ref,
            TypeTag::List(_) => BorrowMethod::Deref,
        }
    }
}

impl Typing {
    //! Field Type

    /// Maps the `declared` type to its field type.
    pub fn field_type(&self, declared: &TypeTag, optional: bool) -> RustType {
        let owned: RustType = self.owned(declared);
        if optional {
            owned.to_option()
        } else {
            owned
        }
    }
}
