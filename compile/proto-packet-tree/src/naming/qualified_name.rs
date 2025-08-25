use custom_string::custom_string;

use crate::{validate_mod_path, validate_type_name, ModPathRef, TypeNameRef};

custom_string!(
    #[doc = "A type name with an optional mod path. (ex: the.mod.path.TheTypeName or TheTypeName)"],
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

    /// Checks if `b` is a valid byte in a qualified name.
    pub fn is_valid_byte(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_' || b == b'.'
    }

    /// Checks if `c` is a valid char in a qualified name.
    pub fn is_valid_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || c == '.'
    }

    /// Get the mod path and the type name from the `qualified_name`.
    ///
    /// # Unsafe
    /// The `qualified_name` must be valid.
    unsafe fn mod_path_and_type_name<'a>(
        qualified_name: &'a str,
    ) -> (Option<ModPathRef<'a>>, TypeNameRef<'a>) {
        if let Some(last_dot) = qualified_name.as_bytes().iter().rposition(|c| *c == b'.') {
            let mod_path: &str = &qualified_name[..last_dot];
            let type_name: &str = &qualified_name[(last_dot + 1)..];
            unsafe {
                (
                    Some(ModPathRef::new_unchecked(mod_path)),
                    TypeNameRef::new_unchecked(type_name),
                )
            }
        } else {
            unsafe { (None, TypeNameRef::new_unchecked(qualified_name)) }
        }
    }

    /// Gets the optional mod path.
    pub fn mod_path<'a>(&'a self) -> Option<ModPathRef<'a>> {
        unsafe { Self::mod_path_and_type_name(self.value.as_str()).0 }
    }

    /// Gets the type name.
    pub fn type_name<'a>(&'a self) -> TypeNameRef<'a> {
        unsafe { Self::mod_path_and_type_name(self.value.as_str()).1 }
    }
}

impl<'a> QualifiedNameRef<'a> {
    //! Properties

    /// Gets the optional mod path.
    pub fn mod_path(&'a self) -> Option<ModPathRef<'a>> {
        unsafe { QualifiedName::mod_path_and_type_name(self.value).0 }
    }

    /// Gets the type name.
    pub fn type_name(&'a self) -> TypeNameRef<'a> {
        unsafe { QualifiedName::mod_path_and_type_name(self.value).1 }
    }
}

/// An element with a qualified name.
pub trait WithQualifiedName {
    /// Gets the qualified name.
    fn qualified_name<'a>(&'a self) -> QualifiedNameRef<'a>;
}
