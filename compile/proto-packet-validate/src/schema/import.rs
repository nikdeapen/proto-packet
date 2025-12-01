use crate::InvalidImportError::*;
use crate::{Error, ErrorInfo, V_IMPORT};
use lex::{Context, Token};
use proto_packet_parse::ImportTree;
use proto_packet_tree::{Import, QualifiedName, TypeName};

#[derive(Debug)]
pub enum InvalidImportError<'a> {
    InvalidName {
        name: Token<'a>,
        error: &'static str,
    },
    InvalidAlias {
        name: Token<'a>,
        alias: Token<'a>,
        error: &'static str,
    },
}

impl<'a> Error for InvalidImportError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match self {
            InvalidName { name, error } => {
                V_IMPORT.invalid_name(file_name, context, "schema", *name, *error)
            }
            InvalidAlias { alias, error, .. } => {
                V_IMPORT.invalid_name(file_name, context, "schema alias", *alias, *error)
            }
        }
    }
}

pub fn validate_import<'a>(tree: &'a ImportTree) -> Result<Import, InvalidImportError<'a>> {
    let name: QualifiedName =
        QualifiedName::new(tree.name.value()).map_err(|error| InvalidName {
            name: tree.name,
            error,
        })?;
    let mut import: Import = Import::from(name);
    if let Some(alias) = tree.alias {
        let alias: TypeName = TypeName::new(alias.value()).map_err(|error| InvalidAlias {
            name: tree.name,
            alias,
            error,
        })?;
        import.set_alias(alias);
    }
    Ok(import)
}
