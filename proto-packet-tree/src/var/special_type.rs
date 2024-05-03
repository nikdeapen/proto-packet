use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::var::type_tag::TypeTag;
use crate::WithName;

/// A special type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SpecialType {
    /// A string type.
    String,

    /// A 16-byte UUID type.
    UniqueIdentifier,
}

impl WithName for SpecialType {
    fn name(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::UniqueIdentifier => "uuid",
        }
    }
}

impl SpecialType {
    //! Type Tags

    /// Converts the primitive type to a type tag.
    pub fn to_type_tag(&self) -> TypeTag {
        TypeTag::Special(*self)
    }
}

impl Display for SpecialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for SpecialType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "string" => Self::String,
            "uuid" => Self::UniqueIdentifier,
            _ => return Err(()),
        })
    }
}
