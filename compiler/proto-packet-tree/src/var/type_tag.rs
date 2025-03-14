use std::fmt::{Debug, Display, Formatter};

use crate::var::special_type::SpecialType;
use crate::{PrimitiveType, QualifiedName};

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),

    /// A special type.
    Special(SpecialType),

    /// A named type.
    Named(QualifiedName),
}

impl From<PrimitiveType> for TypeTag {
    fn from(primitive: PrimitiveType) -> Self {
        Self::Primitive(primitive)
    }
}

impl From<QualifiedName> for TypeTag {
    fn from(name: QualifiedName) -> Self {
        Self::Named(name)
    }
}

impl Debug for TypeTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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
