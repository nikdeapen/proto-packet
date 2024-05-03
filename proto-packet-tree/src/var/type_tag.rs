use std::fmt::{Display, Formatter};

use crate::var::special_type::SpecialType;
use crate::PrimitiveType;

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),

    /// A special type.
    Special(SpecialType),

    /// A named type.
    Named(String),
}

impl From<PrimitiveType> for TypeTag {
    fn from(primitive: PrimitiveType) -> Self {
        Self::Primitive(primitive)
    }
}

impl From<SpecialType> for TypeTag {
    fn from(special: SpecialType) -> Self {
        Self::Special(special)
    }
}

impl From<String> for TypeTag {
    fn from(name: String) -> Self {
        Self::Named(name)
    }
}

impl From<&str> for TypeTag {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

impl Display for TypeTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(primitive) => write!(f, "{}", primitive),
            Self::Special(special) => write!(f, "{}", special),
            Self::Named(name) => write!(f, "{}", name),
        }
    }
}
