use lex::{ParseContext, Token};

use crate::ParseVarErrorReason::*;
use crate::{parse_type_tag, ParseTypeTagError, TypeTagTree};

#[derive(Debug)]
pub struct VarTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub name: Token<'a>,
    pub type_tag: TypeTagTree<'a>,
}

#[derive(Debug)]
pub struct ParseVarError<'a> {
    pub name: Token<'a>,
    pub reason: ParseVarErrorReason,
}

#[derive(Debug)]
pub enum ParseVarErrorReason {
    ExpectedColon,
    InvalidTypeTag(ParseTypeTagError),
}

/// Parses an optional variable.
///
/// Returns `Ok(Some(var), after_type_tag)`.
/// Returns `Ok(None, c)` if the next non-white token is not a symbol.
pub fn parse_var(c: ParseContext) -> lex::Result<Option<VarTree>, ParseVarError> {
    let (comments, after_comments) = c.line_comment_block();
    if let (Some(name), after_name) = after_comments.symbol() {
        let (_white, after_white) = after_name.white_line_comments();
        if let (Some(_colon), after_colon) = after_white.mark(':') {
            let (_white, after_white) = after_colon.white_line_comments();
            match parse_type_tag(after_white) {
                Ok((type_tag, after_type_tag)) => {
                    let tree: VarTree = VarTree {
                        comments,
                        name,
                        type_tag,
                    };
                    Ok((Some(tree), after_type_tag))
                }
                Err(e) => Err(e.map(|e| ParseVarError {
                    name,
                    reason: InvalidTypeTag(e),
                })),
            }
        } else {
            Err(after_white.to_error(ParseVarError {
                name,
                reason: ExpectedColon,
            }))
        }
    } else {
        Ok((None, c))
    }
}
