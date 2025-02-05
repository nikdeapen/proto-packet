use code_gen::rust::{Reference, RustPrimitive, RustType};

use proto_packet_tree::{PrimitiveType, QualifiedNameRef, SpecialType, TypeTag};

/// Responsible for type conversions & type utilities.
#[derive(Clone, Debug, Default)]
pub struct Typing {
    _nothing: (),
}

impl Typing {
    //! Copy & Clone

    /// Checks if the `declared` type is converted to a rust `Copy` type.
    pub fn is_copy(&self, declared: &TypeTag) -> bool {
        match declared {
            TypeTag::Primitive(_) => true,
            TypeTag::Special(special) => match special {
                SpecialType::UUID => true,
                SpecialType::String => false,
            },
            _ => false,
        }
    }

    /// Checks if the `declared` type is converted to a rust `as_ref` type.
    pub fn is_ref(&self, declared: &TypeTag) -> bool {
        match declared {
            TypeTag::Named(_) => true,
            _ => false,
        }
    }

    /// Checks if the `declared` type is converted to a rust `as_deref` type.
    pub fn is_deref(&self, declared: &TypeTag) -> bool {
        match declared {
            TypeTag::Special(special) => match special {
                SpecialType::UUID => false,
                SpecialType::String => true,
            },
            TypeTag::Slice(_) => true,
            _ => false,
        }
    }
}

impl Typing {
    //! Rust Name

    /// Gets the fully qualified rust name for the `qualified_name`.
    pub fn rust_name(&self, qualified_name: QualifiedNameRef) -> String {
        format!("crate::{}", qualified_name.as_ref().replace(".", "::"))
    }
}

impl Typing {
    //! Fields

    /// Gets the non-optional field type for the declared `type_tag`.
    pub fn field_type(&self, type_tag: &TypeTag) -> RustType {
        match type_tag {
            TypeTag::Primitive(primitive) => self.primitive_field_type(*primitive),
            TypeTag::Special(special) => self.special_field_type(*special),
            TypeTag::Named(name) => RustType::from(self.rust_name(name.to_ref())),
            TypeTag::Slice(base) => RustType::from("Vec").with_generic(self.field_type(base)),
        }
    }

    /// Gets the non-optional field type for the declared `primitive` type.
    pub fn primitive_field_type(&self, primitive: PrimitiveType) -> RustType {
        match primitive {
            PrimitiveType::UnsignedInt8 => RustPrimitive::UnsignedInt8.to_type_tag(),
            PrimitiveType::UnsignedInt16 => RustPrimitive::UnsignedInt16.to_type_tag(),
            PrimitiveType::UnsignedInt32 => RustPrimitive::UnsignedInt32.to_type_tag(),
            PrimitiveType::UnsignedInt64 => RustPrimitive::UnsignedInt64.to_type_tag(),
            PrimitiveType::UnsignedInt128 => RustPrimitive::UnsignedInt128.to_type_tag(),
        }
    }

    /// Gets the non-optional field type for the declared `special` type.
    pub fn special_field_type(&self, special: SpecialType) -> RustType {
        match special {
            SpecialType::UUID => RustType::from("uuid::Uuid"),
            SpecialType::String => RustType::from("String"),
        }
    }
}

impl Typing {
    //! Borrowed

    /// Gets the non-optional borrowed type for the declared `type_tag`.
    pub fn borrowed_type(&self, type_tag: &TypeTag) -> RustType {
        match type_tag {
            TypeTag::Primitive(primitive) => self.primitive_field_type(*primitive),
            TypeTag::Special(special) => match special {
                SpecialType::UUID => self.special_field_type(*special),
                SpecialType::String => RustType::from("str").to_ref_type(Reference::default()),
            },
            TypeTag::Named(name) => {
                RustType::from(self.rust_name(name.to_ref())).to_ref_type(Reference::default())
            }
            TypeTag::Slice(base) => self
                .field_type(base)
                .to_slice()
                .to_ref_type(Reference::default()),
        }
    }
}
