use crate::Error;
use proto_packet_tree::{Import, ModPathRef, QualifiedName, QualifiedNameRef, TypeNameRef};

/// Responsible for resolving names.
#[derive(Debug)]
pub struct Resolver<'a> {
    mod_path: ModPathRef<'a>,
    local_names: &'a [TypeNameRef<'a>],
    imports: &'a [Import],
}

impl<'a> Resolver<'a> {
    //! Construction

    /// Creates a new resolver.
    pub unsafe fn new_unchecked(
        mod_path: ModPathRef<'a>,
        local_names: &'a [TypeNameRef<'a>],
        imports: &'a [Import],
    ) -> Self {
        // todo -- validation

        Self {
            mod_path,
            local_names,
            imports,
        }
    }
}

impl<'a> Resolver<'a> {
    //! Resolve

    /// Resolves the `name`.
    pub fn resolve(&self, name: QualifiedNameRef<'a>) -> Result<QualifiedName, Error> {
        if name.mod_path().is_some() {
            Ok(name.to_owned())
        } else {
            for local_name in self.local_names {
                if name.type_name() == local_name {
                    return Ok(self.mod_path.to_qualified_name(name.type_name()));
                }
            }
            for import in self.imports {
                if name.type_name() == import.effective_name() {
                    return Ok(import.name().to_owned());
                }
            }
            Err(Error::UnresolvableName {
                mod_path: self.mod_path.to_owned(),
                type_name: name.type_name().to_owned(),
            })
        }
    }
}
