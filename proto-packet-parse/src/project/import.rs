use lex::{ParseContext, Token};

use crate::ParseImportError::{
    ExpectedAlias, ExpectedAsWhitespace, ExpectedName, ExpectedSemicolon, ExpectedWhitespace,
};
use crate::{expected_got_instead, parse_qualified_name, Error, ErrorInfo, P_IMPORT};

#[derive(Debug)]
pub struct ImportTree<'a> {
    pub name: Token<'a>,
    pub alias: Option<Token<'a>>,
}

#[derive(Debug)]
pub enum ParseImportError<'a> {
    ExpectedWhitespace,
    ExpectedName,
    ExpectedAsWhitespace { name: Token<'a> },
    ExpectedAlias { name: Token<'a> },
    ExpectedSemicolon { name: Token<'a> },
}

impl<'a> Error for ParseImportError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let message: String = match self {
            ExpectedWhitespace => expected_got_instead("whitespace", token),
            ExpectedName => expected_got_instead("an import name", token),
            ExpectedAsWhitespace { .. } => expected_got_instead("whitespace", token),
            ExpectedAlias { .. } => expected_got_instead("an alias name", token),
            ExpectedSemicolon { .. } => expected_got_instead("a semicolon", token),
        };
        ErrorInfo {
            code: P_IMPORT,
            header: "invalid import declaration",
            message,
        }
    }
}

/// Parses an optional import statement.
///
/// Returns `Ok(Some(import), after_semicolon)`.
/// Returns `Ok(None)` when the next non-white symbol is not `import`.
pub fn parse_import(c: ParseContext) -> lex::Result<Option<ImportTree>, ParseImportError> {
    let (_white, after_white) = c.white_line_comments();
    if let (Some(_import), after_import) = after_white.exact_symbol("import") {
        let (white, after_white) = after_import.white_line_comments();
        if white.is_none() {
            return Err(after_import.to_error(ExpectedWhitespace));
        }
        if let (Some(name), after_name) = parse_qualified_name(after_white) {
            let (alias, after_alias) = parse_import_alias(name, after_name)?;
            let (_white, after_white) = after_alias.white_line_comments();
            let (_semi, after_semi) = after_white.exact(";");
            let tree: ImportTree = ImportTree { name, alias };
            Ok((Some(tree), after_semi))
        } else {
            Err(after_white.to_error(ExpectedName))
        }
    } else {
        Ok((None, c))
    }
}

fn parse_import_alias<'a>(
    name: Token<'a>,
    c: ParseContext<'a>,
) -> lex::Result<'a, Option<Token<'a>>, ParseImportError<'a>> {
    let (_white, after_white) = c.white_line_comments();
    if let (Some(_as), after_as) = after_white.exact_symbol("as") {
        let (white, after_white) = after_as.white_line_comments();
        if white.is_none() {
            return Err(after_as.to_error(ExpectedAsWhitespace { name }));
        }
        if let (Some(alias), after_alias) = after_white.symbol() {
            Ok((Some(alias), after_alias))
        } else {
            Err(after_white.to_error(ExpectedAlias { name }))
        }
    } else {
        Ok((None, c))
    }
}
