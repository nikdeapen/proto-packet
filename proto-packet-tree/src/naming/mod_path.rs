use custom_string::custom_string;

use crate::validate_mod_name;

// A mod path.
custom_string!(ModPath, ModPathRef, |s| validate_mod_path(s));

/// Validates the `mod_path`.
pub fn validate_mod_path(mod_path: &str) -> Result<(), &'static str> {
    for mod_name in mod_path.split(".") {
        validate_mod_name(mod_name)?;
    }
    Ok(())
}

/// An element with a mod path.
pub trait WithModPath {
    /// Gets the mod path.
    fn mod_path(&self) -> ModPathRef;
}
