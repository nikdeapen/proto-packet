use custom_string::custom_string;

use crate::{validate_mod_path, validate_type_name, TypeNameRef};

// An optional mod path with a type name. (ex: the.mod.path.TheTypeName or TheTypeName)
custom_string!(
    QualifiedName,
    QualifiedNameRef,
    |s| validate_qualified_name(s)
);

/// Validates the `qualified_name`.
pub fn validate_qualified_name(qualified_name: &str) -> Result<(), &'static str> {
    if let Some(last_dot) = qualified_name.as_bytes().iter().rposition(|c| *c == b'.') {
        validate_mod_path(&qualified_name[..last_dot])?;
        validate_type_name(&qualified_name[(last_dot + 1)..])
    } else {
        validate_type_name(qualified_name)
    }
}

impl QualifiedName {
    //! Properties

    /// Gets the type name.
    pub fn type_name(&self) -> TypeNameRef {
        if let Some(dot) = self.value.as_bytes().iter().position(|c| *c == b'.') {
            unsafe { TypeNameRef::new_unchecked(&self.value()[dot + 1..]) }
        } else {
            unsafe { TypeNameRef::new_unchecked(self.value()) }
        }
    }
}

/// An element with a qualified name.
pub trait WithQualifiedName {
    /// Gets the qualified name.
    fn qualified_name(&self) -> QualifiedNameRef;
}
