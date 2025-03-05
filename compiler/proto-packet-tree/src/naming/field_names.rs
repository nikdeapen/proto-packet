use custom_string::custom_string;

use crate::naming::name::validate_name;

// todo -- doc comments

// The name of a field.
custom_string!(FieldName, FieldNameRef, |s| validate_field_name(s));

/// Validates the `field_name`.
pub fn validate_field_name(field_name: &str) -> Result<(), &'static str> {
    validate_name(field_name)?;
    if !field_name.as_bytes()[0].is_ascii_lowercase() {
        Err("field names must start with a lowercase letter")
    } else {
        Ok(())
    }
}

/// An element with a field name.
pub trait WithFieldName {
    /// Gets the field name.
    fn field_name(&self) -> FieldNameRef;
}
