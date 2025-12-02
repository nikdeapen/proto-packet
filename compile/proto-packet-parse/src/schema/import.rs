use crate::ParseImportError::{
    ExpectedAlias, ExpectedAsWhitespace, ExpectedName, ExpectedSemicolon, ExpectedWhitespace,
};
use crate::{parse_qualified_name, Error, ErrorInfo, P_IMPORT};
use lex::{Context, ParseResult, Token};

#[derive(Debug)]
pub struct ImportTree<'a> {
    pub name: Token<'a>,
    pub alias: Option<Token<'a>>,
}

#[derive(Debug)]
pub enum ParseImportError {
    ExpectedWhitespace,
    ExpectedName,
    ExpectedAsWhitespace,
    ExpectedAlias,
    ExpectedSemicolon,
}

impl Error for ParseImportError {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match self {
            ExpectedWhitespace => "whitespace",
            ExpectedName => "an schema name",
            ExpectedAsWhitespace { .. } => "whitespace",
            ExpectedAlias { .. } => "an alias name",
            ExpectedSemicolon { .. } => "a semicolon",
        };
        P_IMPORT.expected_got_instead(expected, token)
    }
}

/// Parses an optional impor declaration.
///
/// Returns `Ok(Some(import), after_semicolon)`.
/// Returns `Ok(None)` when the next token is not `import`.
pub fn parse_import(c: Context) -> ParseResult<Option<ImportTree>, ParseImportError> {
    let (_white, c) = c.white_line_comments();
    if let (Some(_import), after_import) = c.exact_symbol("schema") {
        let (white, after_white) = after_import.whitespace();
        if white.is_none() {
            return Err(after_import.to_error(ExpectedWhitespace));
        }
        if let (Some(name), after_name) = parse_qualified_name(after_white) {
            let (alias, after_alias) = parse_import_alias(after_name)?;
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

fn parse_import_alias(c: Context) -> ParseResult<Option<Token>, ParseImportError> {
    let (_white, after_white) = c.white_line_comments();
    if let (Some(_as), after_as) = after_white.exact_symbol("as") {
        let (white, after_white) = after_as.white_line_comments();
        if white.is_none() {
            return Err(after_as.to_error(ExpectedAsWhitespace));
        }
        if let (Some(alias), after_alias) = after_white.symbol() {
            Ok((Some(alias), after_alias))
        } else {
            Err(after_white.to_error(ExpectedAlias))
        }
    } else {
        Ok((None, c))
    }
}
