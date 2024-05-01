use std::fmt::{Display, Formatter};

use crate::{TypeTag, WithName, WithTypeTag};

/// A name with an associated type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    tag: TypeTag,
}

impl<S: Into<String>, T: Into<TypeTag>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            tag: tuple.1.into(),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithTypeTag for Var {
    fn type_tag(&self) -> &TypeTag {
        &self.tag
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.tag)
    }
}
