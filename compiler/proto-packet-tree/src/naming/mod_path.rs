use custom_string::custom_string;

use crate::{validate_mod_name, ModNameRef, QualifiedName, TypeNameRef};

// todo -- doc comments

// A module path.
custom_string!(ModPath, ModPathRef, |s| validate_mod_path(s));

/// Validates the `mod_path`.
pub fn validate_mod_path(mod_path: &str) -> Result<(), &'static str> {
    for mod_name in mod_path.split(".") {
        validate_mod_name(mod_name)?;
    }
    Ok(())
}

impl<'a> ModPathRef<'a> {
    //! Properties

    /// Creates an iterator for the mod names.
    pub fn mod_names(&self) -> impl Iterator<Item = ModNameRef> {
        self.value
            .split('.')
            .map(|mod_name| unsafe { ModNameRef::new_unchecked(mod_name) })
    }
}

impl ModPath {
    //! Mutations

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

    /// Converts the mod path to a qualified name given the `type_name`.
    pub fn to_qualified_name(&self, type_name: TypeNameRef) -> QualifiedName {
        let mut s: String = String::with_capacity(self.value.len() + 1 + type_name.len());
        s.push_str(self.value);
        s.push('.');
        s.push_str(type_name.as_ref());
        unsafe { QualifiedName::new_unchecked(s) }
    }
}

impl ModPath {
    //! Conversions

    /// Converts the mod path to a qualified name given the `type_name`.
    pub fn to_qualified_name(self, type_name: TypeNameRef) -> QualifiedName {
        let mut s: String = self.value;
        s.reserve(1 + type_name.len());
        s.push('.');
        s.push_str(type_name.as_ref());
        unsafe { QualifiedName::new_unchecked(s) }
    }
}

/// An element with a mod path.
pub trait WithModPath {
    /// Gets the mod path.
    fn mod_path(&self) -> ModPathRef;
}
