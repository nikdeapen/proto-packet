use crate::{ModPath, TypeName};
use custom_string::custom_string;

custom_string!(
    #[doc = "The mod path. (ex: the.mod.path)"],
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
}
