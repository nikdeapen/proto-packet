/// Validates the `name`.
///
/// This function only validates the common properties of name types.
pub(in crate::naming) fn validate_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        Err("names cannot be empty")
    } else if name
        .as_bytes()
        .iter()
        .any(|c| !c.is_ascii_alphanumeric() && *c != b'_')
    {
        Err("names must only contain: [a-zA-Z0-9_]")
    } else if name.as_bytes()[0] == b'_' {
        Err("names cannot start with an underscore")
    } else if name.as_bytes()[name.len() - 1] == b'_' {
        Err("names cannot end with an underscore")
    } else if name.contains("__") {
        Err("names cannot contain a double underscore")
    } else {
        Ok(())
    }
}
