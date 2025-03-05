use custom_string::custom_string;

use crate::{validate_mod_path, validate_type_name, ModPathRef, TypeNameRef};

// todo -- doc comments

// A type name with an optional mod path. (ex: the.mod.path.TheTypeName or TheTypeName)
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

impl<'a> QualifiedNameRef<'a> {
    //! Properties

    /// Gets the type name.
    pub fn type_name(&self) -> TypeNameRef {
        if let Some(last_dot) = self.value.as_bytes().iter().rposition(|c| *c == b'.') {
            let type_name: &str = &self.value[(last_dot + 1)..];
            unsafe { TypeNameRef::new_unchecked(type_name) }
        } else {
            let type_name: &str = self.value;
            unsafe { TypeNameRef::new_unchecked(type_name) }
        }
    }

    /// Gets the optional mod path.
    pub fn mod_path(&self) -> Option<ModPathRef> {
        if let Some(last_dot) = self.value.as_bytes().iter().rposition(|c| *c == b'.') {
            let mod_path: &str = &self.value[..last_dot];
            let mod_path: ModPathRef = unsafe { ModPathRef::new_unchecked(mod_path) };
            Some(mod_path)
        } else {
            None
        }
    }
}

impl QualifiedName {
    //! Properties

    /// Gets the type name.
    pub fn type_name(&self) -> TypeNameRef {
        // todo -- duplicate code
        if let Some(last_dot) = self.value.as_bytes().iter().rposition(|c| *c == b'.') {
            let type_name: &str = &self.value[(last_dot + 1)..];
            unsafe { TypeNameRef::new_unchecked(type_name) }
        } else {
            let type_name: &str = self.value.as_str();
            unsafe { TypeNameRef::new_unchecked(type_name) }
        }
    }

    /// Gets the optional mod path.
    pub fn mod_path(&self) -> Option<ModPathRef> {
        // todo -- duplicate code
        if let Some(last_dot) = self.value.as_bytes().iter().rposition(|c| *c == b'.') {
            let mod_path: &str = &self.value[..last_dot];
            let mod_path: ModPathRef = unsafe { ModPathRef::new_unchecked(mod_path) };
            Some(mod_path)
        } else {
            None
        }
    }
}

/// An element with a qualified name.
pub trait WithQualifiedName {
    /// Gets the qualified name.
    fn qualified_name(&self) -> QualifiedNameRef;
}
