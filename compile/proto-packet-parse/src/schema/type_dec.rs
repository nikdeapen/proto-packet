use crate::ParseTypeDecError::*;
use crate::TypeDecTree::*;
use crate::{parse_struct, Error, ErrorInfo, ParseStructError, StructTree};
use lex::{Context, ParseResult, Token};

#[derive(Debug)]
pub enum TypeDecTree<'a> {
    StructDec(StructTree<'a>),
}

impl<'a> TypeDecTree<'a> {
    //! Properties

    /// Gets the type name token.
    pub fn type_name_token(&self) -> Token<'_> {
        match self {
            StructDec(s) => s.struct_name,
        }
    }
}

#[derive(Debug)]
pub enum ParseTypeDecError {
    InvalidStruct(ParseStructError),
}

impl Error for ParseTypeDecError {
    fn info(&self, token: &str) -> ErrorInfo {
        match self {
            InvalidStruct(e) => e.info(token),
        }
    }
}

/// Parses an optional type dec tree.
///
/// Returns `(Some(type_dec), after_type_dec)`.
/// Returns `(None, c)` if the next non-white token does not start a valid type dec.
pub fn parse_type_dec(c: Context) -> ParseResult<Option<TypeDecTree>, ParseTypeDecError> {
    let (comments, after_white) = c.line_comment_block();

    match parse_struct(after_white) {
        Ok((Some(mut structure), after_struct)) => {
            structure.comments = comments;
            return Ok((Some(StructDec(structure)), after_struct));
        }
        Err(e) => return Err(e.map(|e| InvalidStruct(e))),
        _ => {}
    }

    Ok((None, c))
}
