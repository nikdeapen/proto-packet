use std::str::FromStr;

use lex::{symbol_optional, LexResult, Token};

use proto_packet_tree::{PrimitiveType, TypeTag};

/// Parses a type tag.
pub fn type_tag(token: Token) -> LexResult<TypeTag, ()> {
    if let (Some(symbol), token) = symbol_optional(token) {
        if let Ok(primitive) = PrimitiveType::from_str(symbol.value()) {
            Ok((primitive.to_type_tag(), token))
        } else {
            Err(token.into())
        }
    } else {
        Err(token.into())
    }
}
