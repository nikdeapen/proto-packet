use crate::{ModPath, ModPathRef, TypeName, TypeNameRef};
use custom_string::custom_string;

custom_string!(
    #[doc = "A qualified name. (ex: TypeName or the.mod.path.TypeName)"],
    QualifiedName,
    QualifiedNameRef,
    WithQualifiedName,
    qualified_name,
    |s| validate_qualified_name(s)
);

/// Validates the `qualified_name`.
pub fn validate_qualified_name(qualified_name: &str) -> Result<(), &'static str> {
    if let Some(last_dot) = qualified_name.as_bytes().iter().rposition(|c| *c == b'.') {
        ModPath::validate(&qualified_name[..last_dot])?;
        TypeName::validate(&qualified_name[(last_dot + 1)..])?;
    } else {
        TypeName::validate(qualified_name)?;
    }
    Ok(())
}

impl QualifiedName {
    //! Properties

    /// Checks if `b` is a valid byte in a qualified name.
    pub fn is_valid_byte(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_' || b == b'.'
    }

    /// Checks if `c` is a valid char in a qualified name.
    pub fn is_valid_char(c: char) -> bool {
        let c: u32 = c as u32;
        c <= 0xFF && Self::is_valid_byte(c as u8)
    }

    /// Get the mod path and the type name from the `qualified_name`.
    ///
    /// # Safety
    /// The `qualified_name` must be valid.
    unsafe fn mod_path_and_type_name(
        qualified_name: &str,
    ) -> (Option<ModPathRef<'_>>, TypeNameRef<'_>) {
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
    pub fn mod_path(&self) -> Option<ModPathRef<'_>> {
        unsafe { Self::mod_path_and_type_name(self.value.as_str()).0 }
    }

    /// Gets the type name.
    pub fn type_name(&self) -> TypeNameRef<'_> {
        unsafe { Self::mod_path_and_type_name(self.value.as_str()).1 }
    }
}

impl<'a> QualifiedNameRef<'a> {
    //! Properties

    /// Gets the optional mod path.
    pub fn mod_path(&self) -> Option<ModPathRef<'_>> {
        unsafe { QualifiedName::mod_path_and_type_name(self.value).0 }
    }

    /// Gets the type name.
    pub fn type_name(&self) -> TypeNameRef<'_> {
        unsafe { QualifiedName::mod_path_and_type_name(self.value).1 }
    }
}
