use crate::{Struct, TypeNameRef, WithTypeName};

/// A type declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeDec {
    /// A struct declaration.
    StructDec(Struct),
}

impl From<Struct> for TypeDec {
    fn from(structure: Struct) -> Self {
        Self::StructDec(structure)
    }
}

impl WithTypeName for TypeDec {
    fn type_name(&self) -> TypeNameRef<'_> {
        match self {
            Self::StructDec(structure) => structure.type_name(),
        }
    }
}
