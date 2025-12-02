use crate::{QualifiedName, QualifiedNameRef, TypeName, TypeNameRef};

/// An import declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Import {
    name: QualifiedName,
    alias: Option<TypeName>,
}

impl<N: Into<QualifiedName>> From<N> for Import {
    fn from(name: N) -> Self {
        Self {
            name: name.into(),
            alias: None,
        }
    }
}

impl Import {
    //! Properties

    /// Gets the qualified name.
    pub fn name(&self) -> QualifiedNameRef<'_> {
        self.name.to_ref()
    }

    /// Gets the alias if not `None`, otherwise returns the qualified name`s type name.
    pub fn effective_name(&self) -> TypeNameRef<'_> {
        if let Some(alias) = &self.alias {
            alias.to_ref()
        } else {
            self.name.type_name()
        }
    }
}

impl Import {
    //! Alias

    /// Gets the optional alias.
    pub fn alias(&self) -> Option<TypeNameRef<'_>> {
        self.alias.as_ref().map(|s| s.to_ref())
    }

    /// Sets the `alias`.
    pub fn set_alias<N>(&mut self, alias: N)
    where
        N: Into<TypeName>,
    {
        self.alias = Some(alias.into());
    }

    /// Sets the `alias`.
    pub fn with_alias<N>(mut self, alias: N) -> Self
    where
        N: Into<TypeName>,
    {
        self.set_alias(alias);
        self
    }
}
