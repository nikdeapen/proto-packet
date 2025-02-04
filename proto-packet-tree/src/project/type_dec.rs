use crate::{Enum, Message, TypeNameRef, WithTypeName};

/// A type declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeDec {
    /// A message declaration.
    MessageDec(Message),
    /// An enum declaration.
    EnumDec(Enum),
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

impl WithTypeName for TypeDec {
    fn type_name(&self) -> TypeNameRef {
        match self {
            Self::MessageDec(message) => message.type_name(),
            Self::EnumDec(enom) => enom.type_name(),
        }
    }
}
