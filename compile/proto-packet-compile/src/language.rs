use crate::Language::Rust;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A target language.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Language {
    Rust,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Rust => "rust",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Rust),
            _ => Err(()),
        }
    }
}
