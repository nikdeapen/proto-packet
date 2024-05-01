use crate::var::type_tag::TypeTag;
use crate::WithName;
use std::fmt::{Display, Formatter};

/// A primitive type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PrimitiveType {
    /// An unsigned 8-bit integer.
    UnsignedInt8,

    /// An unsigned 16-bit integer.
    UnsignedInt16,

    /// An unsigned 32-bit integer.
    UnsignedInt32,

    /// An unsigned 64-bit integer.
    UnsignedInt64,

    /// An unsigned 128-bit integer.
    UnsignedInt128,
}

impl WithName for PrimitiveType {
    fn name(&self) -> &'static str {
        match self {
            Self::UnsignedInt8 => "u8",
            Self::UnsignedInt16 => "u16",
            Self::UnsignedInt32 => "u32",
            Self::UnsignedInt64 => "u64",
            Self::UnsignedInt128 => "u128",
        }
    }
}

impl PrimitiveType {
    //! Type Tags

    /// Converts the primitive type to a type tag.
    pub fn to_type_tag(&self) -> TypeTag {
        TypeTag::Primitive(*self)
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
