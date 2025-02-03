use custom_string::custom_string;

use crate::{validate_mod_name, ModNameRef, QualifiedName, TypeNameRef};

// A mod path.
custom_string!(ModPath, ModPathRef, |s| validate_mod_path(s));

/// Validates the `mod_path`.
pub fn validate_mod_path(mod_path: &str) -> Result<(), &'static str> {
    for mod_name in mod_path.split(".") {
        validate_mod_name(mod_name)?;
    }
    Ok(())
}

impl ModPath {
    //! Conversions

    /// Appends the `mod_name`.
    pub fn with_appended(mut self, mod_name: ModNameRef) -> Self {
        self.value.reserve(1 + mod_name.len());
        self.value.push('.');
        self.value.push_str(mod_name.as_ref());

        self
    }
}

impl<'a> ModPathRef<'a> {
    //! Conversions

    /// Converts the mod path to a qualified name by appending the type name.
    pub fn to_qualified_name(&self, type_name: TypeNameRef) -> QualifiedName {
        unsafe { QualifiedName::new_unchecked(format!("{}.{}", self, type_name)) }
    }
}

/// An element with a mod path.
pub trait WithModPath {
    /// Gets the mod path.
    fn mod_path(&self) -> ModPathRef;
}
