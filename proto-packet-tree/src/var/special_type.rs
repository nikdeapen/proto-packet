use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::TypeTag;

/// A special type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SpecialType {
    /// A 16-byte identifier.
    UUID,

    /// A sequence of characters.
    String,
}

impl SpecialType {
    //! Type Tags

    /// Converts the special type to a type tag.
    pub fn to_type_tag(&self) -> TypeTag {
        TypeTag::Special(*self)
    }
}

impl Display for SpecialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            SpecialType::UUID => "uuid",
            SpecialType::String => "string",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for SpecialType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "uuid" => Self::UUID,
            "string" => Self::String,
            _ => return Err(()),
        })
    }
}
