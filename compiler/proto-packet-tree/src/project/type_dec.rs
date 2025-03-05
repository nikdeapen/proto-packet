use crate::{Message, TypeNameRef, WithTypeName};

/// A type declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeDec {
    /// A message declaration.
    MessageDec(Message),
}

impl From<Message> for TypeDec {
    fn from(message: Message) -> Self {
        Self::MessageDec(message)
    }
}

impl WithTypeName for TypeDec {
    fn type_name(&self) -> TypeNameRef {
        match self {
            Self::MessageDec(message) => message.type_name(),
        }
    }
}
