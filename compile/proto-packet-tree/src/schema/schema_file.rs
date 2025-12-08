use crate::{Import, TypeDec, WithTypeName};

/// A schema file.
///
/// # Invariant
/// 1. No two imports can have the same effective name.
/// 2. No two type declarations can have the same type name.
/// 3. No import can have the same effective name as a type declaration's type name.
#[derive(Clone, Debug, Default)]
pub struct SchemaFile {
    imports: Vec<Import>,
    type_decs: Vec<TypeDec>,
}

impl SchemaFile {
    //! Imports

    /// Gets the imports.
    pub fn imports(&self) -> &[Import] {
        self.imports.as_slice()
    }

    /// Gets the optional import by the `effective_name`.
    pub fn import_by_effective_name<S>(&self, effective_name: S) -> Option<&Import>
    where
        S: AsRef<str>,
    {
        self.imports
            .iter()
            .find(|i| i.effective_name() == effective_name.as_ref())
    }

    /// Checks if the `import` can be added.
    pub fn can_add_import(&self, import: &Import) -> bool {
        self.import_by_effective_name(import.effective_name())
            .is_none()
            && self
                .type_dec_by_type_name(import.effective_name())
                .is_none()
    }

    /// Adds the `import`.
    ///
    /// # Safety
    /// The `import` must be able to be added.
    pub unsafe fn add_import<I>(&mut self, import: I)
    where
        I: Into<Import>,
    {
        let import: Import = import.into();

        debug_assert!(self.can_add_import(&import));

        self.imports.push(import);
    }

    /// Adds the `import`.
    ///
    /// # Safety
    /// The `import` must be able to be added.
    pub unsafe fn with_import<I>(mut self, import: I) -> Self
    where
        I: Into<Import>,
    {
        self.add_import(import);
        self
    }
}

impl SchemaFile {
    //! Type Declarations

    /// Gets the type declarations.
    pub fn type_decs(&self) -> &[TypeDec] {
        self.type_decs.as_slice()
    }

    /// Gets the optional type dec by the `type_name`.
    pub fn type_dec_by_type_name<S>(&self, type_name: S) -> Option<&TypeDec>
    where
        S: AsRef<str>,
    {
        self.type_decs
            .iter()
            .find(|d| d.type_name() == type_name.as_ref())
    }

    /// Checks if the `type_dec` can be added.
    pub fn can_add_type_dec(&self, type_dec: &TypeDec) -> bool {
        self.import_by_effective_name(type_dec.type_name())
            .is_none()
            && self.type_dec_by_type_name(type_dec.type_name()).is_none()
    }

    /// Adds the `type_dec`.
    ///
    /// # Safety
    /// The `type_dec` must be able to be added.
    pub unsafe fn add_type_dec<D>(&mut self, type_dec: D)
    where
        D: Into<TypeDec>,
    {
        let type_dec: TypeDec = type_dec.into();

        debug_assert!(self.can_add_type_dec(&type_dec));

        self.type_decs.push(type_dec);
    }

    /// Adds the `type_dec`.
    ///
    /// # Safety
    /// The `type_dec` must be able to be added.
    pub unsafe fn with_type_dec<D>(mut self, type_dec: D) -> Self
    where
        D: Into<TypeDec>,
    {
        self.add_type_dec(type_dec);
        self
    }
}
