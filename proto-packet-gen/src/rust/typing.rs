use code_gen::rust::{PrimitiveType as RustPrimitive, Reference, TypeTag as RustType};

use proto_packet_tree::{Message, PrimitiveType, SpecialType, TypeTag, WithTypeTag};

use crate::GenError;

/// Responsible for type conversions & type utilities.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Typing {
    _nothing: (),
}

impl Typing {
    //! Type Properties

    /// Checks if the declared type is converted to a rust `Copy` type.
    pub fn is_copy(&self, declared: &TypeTag) -> Result<bool, GenError> {
        let is_copy: bool = match declared {
            TypeTag::Primitive(primitive) => match primitive {
                PrimitiveType::UnsignedInt8 => true,
                PrimitiveType::UnsignedInt16 => true,
                PrimitiveType::UnsignedInt32 => true,
                PrimitiveType::UnsignedInt64 => true,
                PrimitiveType::UnsignedInt128 => true,
            },
            TypeTag::Special(special) => match special {
                SpecialType::String => false,
            },
        };
        Ok(is_copy)
    }

    /// Checks if all the fields in the message convert to a rust `Copy` type.
    pub fn all_copy(&self, message: &Message) -> Result<bool, GenError> {
        for field in message.fields() {
            if !self.is_copy(field.type_tag())? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl Typing {
    //! Fields

    /// Gets the field type for the declared type. (the non-optional type)
    pub fn field_type(&self, declared: &TypeTag) -> Result<RustType, GenError> {
        match declared {
            TypeTag::Primitive(primitive) => self.primitive_field_type(*primitive),
            TypeTag::Special(special) => match special {
                SpecialType::String => Ok(RustType::Named("String".to_string())),
            },
        }
    }

    /// Gets the field type for the declared primitive type.
    pub fn primitive_field_type(
        &self,
        primitive_type: PrimitiveType,
    ) -> Result<RustType, GenError> {
        let tag: RustType = match primitive_type {
            PrimitiveType::UnsignedInt8 => RustPrimitive::UnsignedInt8.to_type_tag(),
            PrimitiveType::UnsignedInt16 => RustPrimitive::UnsignedInt16.to_type_tag(),
            PrimitiveType::UnsignedInt32 => RustPrimitive::UnsignedInt32.to_type_tag(),
            PrimitiveType::UnsignedInt64 => RustPrimitive::UnsignedInt64.to_type_tag(),
            PrimitiveType::UnsignedInt128 => RustPrimitive::UnsignedInt128.to_type_tag(),
        };
        Ok(tag)
    }
}

impl Typing {
    //! Reference Types

    /// Generates the non-optional borrowed type for the declared type.
    pub fn borrowed_type(&self, declared: &TypeTag) -> Result<RustType, GenError> {
        let rust_type: RustType = match declared {
            TypeTag::Primitive(primitive) => match primitive {
                PrimitiveType::UnsignedInt8 => RustPrimitive::UnsignedInt8.to_type_tag(),
                PrimitiveType::UnsignedInt16 => RustPrimitive::UnsignedInt16.to_type_tag(),
                PrimitiveType::UnsignedInt32 => RustPrimitive::UnsignedInt32.to_type_tag(),
                PrimitiveType::UnsignedInt64 => RustPrimitive::UnsignedInt64.to_type_tag(),
                PrimitiveType::UnsignedInt128 => RustPrimitive::UnsignedInt128.to_type_tag(),
            },
            TypeTag::Special(special) => match special {
                SpecialType::String => {
                    RustType::Named("str".to_string()).to_ref_type(Reference::default())
                }
            },
        };
        Ok(rust_type)
    }
}
