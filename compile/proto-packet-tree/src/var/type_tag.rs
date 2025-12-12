use crate::{PrimitiveType, QualifiedName, SpecialType};
use std::fmt::{Debug, Display, Formatter};

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),

    /// A special type.
    Special(SpecialType),

    /// A named type.
    Named(QualifiedName),

    /// A list type.
    List(Box<TypeTag>),
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

impl From<QualifiedName> for TypeTag {
    fn from(name: QualifiedName) -> Self {
        Self::Named(name)
    }
}

impl TypeTag {
    //! Lists

    /// Converts the type tag to a list of itself.
    pub fn to_list(self) -> Self {
        Self::List(Box::new(self))
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
            Self::List(base) => write!(f, "[]{}", base),
        }
    }
}
