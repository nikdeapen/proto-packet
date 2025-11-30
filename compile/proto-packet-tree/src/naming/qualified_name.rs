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
