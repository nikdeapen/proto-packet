use custom_string::custom_string;

use crate::naming::name::validate_name;

// todo -- doc comments

// The name of a module.
custom_string!(ModName, ModNameRef, |s| validate_mod_name(s));

/// Validates the `mod_name`.
pub fn validate_mod_name(mod_name: &str) -> Result<(), &'static str> {
    validate_name(mod_name)?;
    if !mod_name.as_bytes()[0].is_ascii_lowercase() {
        Err("mod names must start with a lowercase letter")
    } else {
        Ok(())
    }
}

/// An element with a mod name.
pub trait WithModName {
    /// Gets the mod name.
    fn mod_name(&self) -> ModNameRef;
}
