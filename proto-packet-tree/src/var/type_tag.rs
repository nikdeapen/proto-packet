use std::fmt::{Display, Formatter};

use crate::PrimitiveType;

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),
}

impl From<PrimitiveType> for TypeTag {
    fn from(primitive: PrimitiveType) -> Self {
        Self::Primitive(primitive)
    }
}

impl Display for TypeTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(primitive) => write!(f, "{}", primitive),
        }
    }
}
