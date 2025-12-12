use crate::error::P_TYPE_TAG;
use crate::ParseTypeTagError::*;
use crate::{parse_qualified_name, Error, ErrorInfo};
use lex::{Context, ParseResult, Token};
use proto_packet_tree::{PrimitiveType, SpecialType};
use std::str::FromStr;

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
    List {
        base: Box<TypeTagTree<'a>>,
    },
}

#[derive(Debug)]
pub enum ParseTypeTagError {
    UnrecognizedType,
    ExpectedClosingBracket,
}

impl<'a> Error for ParseTypeTagError {
    fn info(&self, token: &str) -> ErrorInfo {
        let expected: &'static str = match self {
            UnrecognizedType => "a recognized type",
            ExpectedClosingBracket => "a closing bracket",
        };
        P_TYPE_TAG.expected_got_instead(expected, token)
    }
}

/// Parses a type tag tree.
///
/// Returns `Ok(type_tag, after_type_tag)`.
pub fn parse_type_tag(c: Context) -> ParseResult<TypeTagTree, ParseTypeTagError> {
    let (_white, c) = c.whitespace();
    if let (Some(primitive), after_primitive) = parse_primitive_type(c)? {
        Ok((primitive, after_primitive))
    } else if let (Some(special), after_special) = parse_special_type(c)? {
        Ok((special, after_special))
    } else if let (Some(named), after_named) = parse_named_type(c)? {
        Ok((named, after_named))
    } else if let (Some(list), after_list) = parse_list_type(c)? {
        Ok((list, after_list))
    } else {
        Err(c.to_error(UnrecognizedType))
    }
}

fn parse_primitive_type(c: Context) -> ParseResult<Option<TypeTagTree>, ParseTypeTagError> {
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

fn parse_special_type(c: Context) -> ParseResult<Option<TypeTagTree>, ParseTypeTagError> {
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

fn parse_named_type(c: Context) -> ParseResult<Option<TypeTagTree>, ParseTypeTagError> {
    if let (Some(name), after_name) = parse_qualified_name(c) {
        Ok((Some(TypeTagTree::Named { name }), after_name))
    } else {
        Ok((None, c))
    }
}

fn parse_list_type(c: Context) -> ParseResult<Option<TypeTagTree>, ParseTypeTagError> {
    if let (Some(_open), after_open) = c.exact("[") {
        let (_white, after_white) = after_open.whitespace();
        if let (Some(_close), after_close) = after_white.exact("]") {
            let (base, after_base) = parse_type_tag(after_close)?;
            Ok((
                Some(TypeTagTree::List {
                    base: Box::new(base),
                }),
                after_base,
            ))
        } else {
            Err(after_white.to_error(ExpectedClosingBracket))
        }
    } else {
        Ok((None, c))
    }
}
