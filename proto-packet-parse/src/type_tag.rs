use std::str::FromStr;

use lex::{symbol_optional, LexResult, Token};

use proto_packet_tree::{PrimitiveType, SpecialType, TypeTag};

/// Parses a type tag.
pub fn type_tag(token: Token) -> LexResult<TypeTag, ()> {
    if let (Some(symbol), token) = symbol_optional(token) {
        if let Ok(primitive) = PrimitiveType::from_str(symbol.value()) {
            Ok((primitive.to_type_tag(), token))
        } else if let Ok(special) = SpecialType::from_str(symbol.value()) {
            Ok((special.to_type_tag(), token))
        } else {
            Ok((TypeTag::from(symbol.value()), token))
        }
    } else {
        Err(token.into())
    }
}
