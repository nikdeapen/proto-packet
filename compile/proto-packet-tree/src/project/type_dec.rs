use crate::{Enum, Message, Service, Struct, TypeNameRef, Variant, WithTypeName};

/// A type declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeDec {
    /// A struct declaration.
    StructDec(Struct),

    /// A message declaration.
    MessageDec(Message),

    /// An enum declaration.
    EnumDec(Enum),

    /// A variant declaration.
    VariantDec(Variant),

    /// A service declaration.
    ServiceDec(Service),
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
    fn from(enom: Enum) -> Self {
        Self::EnumDec(enom)
    }
}

impl From<Variant> for TypeDec {
    fn from(variant: Variant) -> Self {
        Self::VariantDec(variant)
    }
}

impl From<Service> for TypeDec {
    fn from(service: Service) -> Self {
        Self::ServiceDec(service)
    }
}

impl WithTypeName for TypeDec {
    fn type_name(&self) -> TypeNameRef<'_> {
        match self {
            Self::StructDec(structure) => structure.type_name(),
            Self::MessageDec(message) => message.type_name(),
            Self::EnumDec(message) => message.type_name(),
            Self::VariantDec(variant) => variant.type_name(),
            Self::ServiceDec(service) => service.type_name(),
        }
    }
}
