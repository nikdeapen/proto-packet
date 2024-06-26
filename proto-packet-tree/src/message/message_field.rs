use std::fmt::{Display, Formatter};

use crate::{TypeTag, Var, WithComments, WithName, WithTypeTag, WithVar};

/// A message field.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MessageField {
    comments: Vec<String>,
    var: Var,
    field_number: Option<u32>,
}

impl<V: Into<Var>> From<V> for MessageField {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            var: var.into(),
            field_number: None,
        }
    }
}

impl WithComments for MessageField {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
        where
            S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl WithName for MessageField {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithTypeTag for MessageField {
    fn type_tag(&self) -> &TypeTag {
        self.var.type_tag()
    }
}

impl WithVar for MessageField {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl MessageField {
    //! Field Number

    /// Gets the field number.
    pub fn field_number(&self) -> Option<u32> {
        self.field_number
    }

    // Sets the field number.
    pub fn with_field_number(mut self, field_number: u32) -> Self {
        self.set_field_number(field_number);
        self
    }

    // Sets the field number.
    pub fn set_field_number(&mut self, field_number: u32) {
        self.field_number = Some(field_number.into());
    }
}

impl Display for MessageField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for comment in self.comments() {
            write!(f, "// {}\n", comment)?;
        }
        write!(f, "{}", self.var)?;
        if let Some(field_number) = self.field_number {
            write!(f, " = {}", field_number)?;
        }
        write!(f, ";")
    }
}