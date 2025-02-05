use std::str::FromStr;

use lex::{ParseContext, Token};

use proto_packet_tree::{PrimitiveType, SpecialType};

use crate::error::P_TYPE_TAG;
use crate::ParseTypeTagError::*;
use crate::{expected_got_instead, parse_qualified_name, Error, ErrorInfo};

#[derive(Debug)]
pub enum TypeTagTree<'a> {
    Primitive {
        symbol: Token<'a>,
        primitive: PrimitiveType,
    },
    Special {
        symbol: Token<'a>,
        special: SpecialType,
    },
    Named {
        name: Token<'a>,
    },
    Slice {
        base: Box<TypeTagTree<'a>>,
    },
}

#[derive(Debug)]
pub enum ParseTypeTagError {
    UnrecognizedType,
    ExpectedClosingSliceBracket,
}

impl<'a> Error for ParseTypeTagError {
    fn info(&self, token: &str) -> ErrorInfo {
        let message: String = match self {
            UnrecognizedType => expected_got_instead("a recognized type", token),
            ExpectedClosingSliceBracket => {
                expected_got_instead("a slice closing bracket `]`", token)
            }
        };
        ErrorInfo {
            code: P_TYPE_TAG,
            header: "invalid type tag",
            message,
        }
    }
}

/// Parses a type tag tree.
///
/// Returns `Ok(type_tag_tree, after_type_tag_tree)`.
pub fn parse_type_tag(c: ParseContext) -> lex::Result<TypeTagTree, ParseTypeTagError> {
    if let (Some(primitive), after_primitive) = parse_primitive_type(c)? {
        Ok((primitive, after_primitive))
    } else if let (Some(special), after_special) = parse_special_type(c)? {
        Ok((special, after_special))
    } else if let (Some(name), after_name) = parse_named_type(c)? {
        Ok((name, after_name))
    } else if let (Some(slice), after_slice) = parse_slice_type(c)? {
        Ok((slice, after_slice))
    } else {
        Err(c.to_error(UnrecognizedType))
    }
}

fn parse_primitive_type(c: ParseContext) -> lex::Result<Option<TypeTagTree>, ParseTypeTagError> {
    if let (Some(symbol), after_symbol) = c.symbol() {
        if let Ok(primitive) = PrimitiveType::from_str(symbol.value()) {
            let tree: TypeTagTree = TypeTagTree::Primitive { symbol, primitive };
            Ok((Some(tree), after_symbol))
        } else {
            Ok((None, c))
        }
    } else {
        Ok((None, c))
    }
}

fn parse_special_type(c: ParseContext) -> lex::Result<Option<TypeTagTree>, ParseTypeTagError> {
    if let (Some(symbol), after_symbol) = c.symbol() {
        if let Ok(special) = SpecialType::from_str(symbol.value()) {
            let tree: TypeTagTree = TypeTagTree::Special { symbol, special };
            Ok((Some(tree), after_symbol))
        } else {
            Ok((None, c))
        }
    } else {
        Ok((None, c))
    }
}

fn parse_named_type(c: ParseContext) -> lex::Result<Option<TypeTagTree>, ParseTypeTagError> {
    if let (Some(name), after_name) = parse_qualified_name(c) {
        Ok((Some(TypeTagTree::Named { name }), after_name))
    } else {
        Ok((None, c))
    }
}

fn parse_slice_type(c: ParseContext) -> lex::Result<Option<TypeTagTree>, ParseTypeTagError> {
    if let (Some(_open), c) = c.mark('[') {
        let (_white, c) = c.white_line_comments();
        if let (Some(_close), c) = c.mark(']') {
            let (_white, c) = c.white_line_comments();
            let (base, c) = parse_type_tag(c)?;
            Ok((
                Some(TypeTagTree::Slice {
                    base: Box::new(base),
                }),
                c,
            ))
        } else {
            Err(c.to_error(ExpectedClosingSliceBracket))
        }
    } else {
        Ok((None, c))
    }
}
