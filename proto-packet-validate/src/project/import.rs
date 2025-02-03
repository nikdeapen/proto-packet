use lex::{ParseContext, Token};

use proto_packet_parse::ImportTree;
use proto_packet_tree::{Import, QualifiedName, TypeName};

use crate::error::invalid_name;
use crate::InvalidImportError::*;
use crate::{Error, ErrorInfo};

#[derive(Debug)]
pub enum InvalidImportError<'a> {
    InvalidName {
        name: Token<'a>,
        error_message: &'static str,
    },
    InvalidAlias {
        name: Token<'a>,
        alias: Token<'a>,
        error_message: &'static str,
    },
}

impl<'a> Error for InvalidImportError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        let code: &str = "V_IMPORT";
        match self {
            InvalidName {
                name,
                error_message,
            } => invalid_name(file_name, context, "import", *name, *error_message, code),
            InvalidAlias {
                alias,
                error_message,
                ..
            } => invalid_name(
                file_name,
                context,
                "import alias",
                *alias,
                *error_message,
                code,
            ),
        }
    }
}

/// Validates the import statement.
pub fn validate_import<'a>(tree: &'a ImportTree) -> Result<Import, InvalidImportError<'a>> {
    let name: QualifiedName =
        QualifiedName::new(tree.name.value()).map_err(|error_message| InvalidName {
            name: tree.name,
            error_message,
        })?;
    let mut import: Import = Import::from(name);
    if let Some(alias) = tree.alias {
        let alias: TypeName =
            TypeName::new(alias.value()).map_err(|error_message| InvalidAlias {
                name: tree.name,
                alias,
                error_message,
            })?;
        import.set_alias(alias);
    }
    Ok(import)
}
