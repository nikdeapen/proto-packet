use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::TypeTag;

/// A special type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SpecialType {
    /// A 16-byte identifier.
    Uuid,

    /// A sequence of chars.
    String,
}

impl SpecialType {
    //! Type Tag

    /// Converts the special type to a type tag.
    pub fn to_type_tag(&self) -> TypeTag {
        TypeTag::Special(*self)
    }
}

impl AsRef<str> for SpecialType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Uuid => "uuid",
            Self::String => "string",
        }
    }
}

impl Debug for SpecialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for SpecialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl FromStr for SpecialType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "uuid" => Self::Uuid,
            "string" => Self::String,
            _ => return Err(()),
        })
    }
}
