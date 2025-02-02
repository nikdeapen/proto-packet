use crate::{Import, TypeDec, TypeNameRef, WithTypeName};

/// A schema file.
///
/// # Invariants
/// 1. No two imports can have the same effective type name.
/// 2. No two type declarations can have the same type name.
/// 3. No type declarations can have the same name as an import's effective type name.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
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

    /// Gets the optional import with the effective `name`.
    pub fn get_import_by_effective_name(&self, name: TypeNameRef) -> Option<&Import> {
        for import in self.imports() {
            if name == import.effective_name() {
                return Some(import);
            }
        }
        None
    }

    /// Adds the import.
    ///
    /// # Unsafe
    /// The schema file cannot have duplicate names.
    pub unsafe fn add_import<I>(&mut self, import: I)
    where
        I: Into<Import>,
    {
        let import: Import = import.into();

        debug_assert!(self
            .get_import_by_effective_name(import.effective_name())
            .is_none());
        debug_assert!(self.get_dec_by_name(import.effective_name()).is_none());

        self.imports.push(import);
    }

    /// Adds the import.
    ///
    /// # Unsafe
    /// The schema file cannot have duplicate names.
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

    /// Gets the optional type declaration with the given `type_name`.
    pub fn get_dec_by_name<S>(&self, type_name: S) -> Option<&TypeDec>
    where
        S: AsRef<str>,
    {
        for dec in &self.type_decs {
            if dec.type_name() == type_name {
                return Some(dec);
            }
        }
        None
    }

    /// Adds the type declaration.
    ///
    /// # Unsafe
    /// The schema file cannot have duplicate names.
    pub unsafe fn add_type_dec<D>(&mut self, type_dec: D)
    where
        D: Into<TypeDec>,
    {
        let type_dec: TypeDec = type_dec.into();

        debug_assert!(self
            .get_import_by_effective_name(type_dec.type_name())
            .is_none());
        debug_assert!(self.get_dec_by_name(type_dec.type_name()).is_none());

        self.type_decs.push(type_dec.into());
    }

    /// Adds the type declaration.
    ///
    /// # Unsafe
    /// The schema file cannot have duplicate declaration names.
    pub unsafe fn with_type_dec<D>(mut self, type_dec: D) -> Self
    where
        D: Into<TypeDec>,
    {
        self.add_type_dec(type_dec);
        self
    }
}
