use crate::{Enum, Message, Struct, TypeNameRef, Variant, WithTypeName};

/// A type declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeDec {
    StructDec(Struct),
    MessageDec(Message),
    EnumDec(Enum),
    VariantDec(Variant),
}

impl From<Struct> for TypeDec {
    fn from(structure: Struct) -> Self {
        Self::StructDec(structure)
    }
}

impl From<Message> for TypeDec {
    fn from(message: Message) -> Self {
        Self::MessageDec(message)
    }
}

impl From<Enum> for TypeDec {
    fn from(enumeration: Enum) -> Self {
        Self::EnumDec(enumeration)
    }
}

impl From<Variant> for TypeDec {
    fn from(enumeration: Variant) -> Self {
        Self::VariantDec(enumeration)
    }
}

impl WithTypeName for TypeDec {
    fn type_name(&self) -> TypeNameRef<'_> {
        match self {
            Self::StructDec(structure) => structure.type_name(),
            Self::MessageDec(message) => message.type_name(),
            Self::EnumDec(enumeration) => enumeration.type_name(),
            Self::VariantDec(variant) => variant.type_name(),
        }
    }
}
