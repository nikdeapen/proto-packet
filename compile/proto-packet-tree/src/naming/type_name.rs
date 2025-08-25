use custom_string::custom_string;

use crate::naming::validate_name::validate_name;

custom_string!(
    #[doc = "The name of a type."],
    TypeName,
    TypeNameRef,
    |s| validate_type_name(s)
);

/// Validates the `type_name`.
pub fn validate_type_name(type_name: &str) -> Result<(), &'static str> {
    validate_name(type_name)?;

    if !type_name.as_bytes()[0].is_ascii_uppercase() {
        Err("type names must start with an uppercase letter")
    } else {
        Ok(())
    }
}

/// An element with a type name.
pub trait WithTypeName {
    /// Gets the type name.
    fn type_name(&self) -> TypeNameRef<'_>;
}
