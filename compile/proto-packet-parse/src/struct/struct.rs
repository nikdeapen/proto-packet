use crate::ParseStructErrorReason::*;
use crate::{
    parse_struct_field, Error, ErrorInfo, ParseStructFieldError, StructFieldTree, P_STRUCT,
};
use lex::{ParseContext, ParseResult, Token};
use proto_packet_tree::FieldNameRef;

#[derive(Debug)]
pub struct StructTree<'a> {
    pub comments: Vec<Token<'a>>,
    pub struct_name: Token<'a>,
    pub fields: Vec<StructFieldTree<'a>>,
}

impl<'a> StructTree<'a> {
    //! Fields

    /// Gets the matching `field_name` tokens.
    pub fn field_name_tokens(&self, field_name: FieldNameRef) -> Vec<Token<'_>> {
        self.fields
            .iter()
            .map(|field| field.field_name)
            .filter(|name| field_name.as_ref() == name.value())
            .collect()
    }
}

#[derive(Debug)]
pub struct ParseStructError<'a> {
    pub struct_name: Option<Token<'a>>,
    pub reason: ParseStructErrorReason<'a>,
}

#[derive(Debug)]
pub enum ParseStructErrorReason<'a> {
    ExpectedWhitespace,
    ExpectedStructName,
    ExpectedOpenCurly,
    InvalidStructField(ParseStructFieldError<'a>),
    ExpectedCloseCurly,
}

impl<'a> Error for ParseStructError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match &self.reason {
            ExpectedWhitespace => "whitespace",
            ExpectedStructName => "a struct name",
            ExpectedOpenCurly => "an opening curly bracket '{'",
            InvalidStructField(e) => return e.info(token),
            ExpectedCloseCurly => "a closing curly bracket '}'",
        };
        P_STRUCT.expected_got_instead(expected, token)
    }
}

/// Parses an optional struct.
///
/// Returns `Ok(struct, after_close_curly)`.
/// Returns `Ok(None, c)` if the next token is not `struct`.
pub fn parse_struct(c: ParseContext) -> ParseResult<Option<StructTree>, ParseStructError> {
    match c.exact_symbol("struct") {
        (Some(_struct), after_struct) => match after_struct.whitespace() {
            (Some(_white), after_white) => match after_white.symbol() {
                (Some(struct_name), after_struct_name) => {
                    parse_struct_block(struct_name, after_struct_name)
                }
                (None, _) => Err(after_white.to_error(ParseStructError {
                    struct_name: None,
                    reason: ExpectedStructName,
                })),
            },
            (None, _) => Err(after_struct.to_error(ParseStructError {
                struct_name: None,
                reason: ExpectedWhitespace,
            })),
        },
        _ => Ok((None, c)),
    }
}

fn parse_struct_block<'a>(
    struct_name: Token<'a>,
    c: ParseContext<'a>,
) -> ParseResult<'a, Option<StructTree<'a>>, ParseStructError<'a>> {
    let (_white, after_white) = c.whitespace();
    match after_white.exact("{") {
        (Some(_open), after_open) => {
            let (fields, after_fields) = parse_struct_fields(struct_name, after_open)?;
            let (_white, after_white) = after_fields.white_line_comments();
            match after_white.exact("}") {
                (Some(_close), after_close) => {
                    let tree: StructTree = StructTree {
                        comments: vec![],
                        struct_name,
                        fields,
                    };
                    Ok((Some(tree), after_close))
                }
                _ => Err(after_white.to_error(ParseStructError {
                    struct_name: Some(struct_name),
                    reason: ExpectedCloseCurly,
                })),
            }
        }
        _ => Err(after_white.to_error(ParseStructError {
            struct_name: Some(struct_name),
            reason: ExpectedOpenCurly,
        })),
    }
}

fn parse_struct_fields<'a>(
    struct_name: Token<'a>,
    mut c: ParseContext<'a>,
) -> ParseResult<'a, Vec<StructFieldTree<'a>>, ParseStructError<'a>> {
    let mut fields: Vec<StructFieldTree> = Vec::default();
    while let (Some(field), after_field) = parse_struct_field(c).map_err(|e| {
        e.map(|e| ParseStructError {
            struct_name: Some(struct_name),
            reason: InvalidStructField(e),
        })
    })? {
        fields.push(field);
        c = after_field;
    }
    Ok((fields, c))
}
