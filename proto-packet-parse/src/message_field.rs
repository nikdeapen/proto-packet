use std::str::FromStr;

use lex::{exact, exact_optional, symbol, symbol_optional, LexError, LexResult, Token};

use proto_packet_tree::MessageField;

use crate::{type_tag, white, white_block, white_optional};

/// Parses an optional message field.
/// - Returns `None` if the next non-whitespace token is not a symbol.
pub fn message_field(token: Token) -> LexResult<Option<MessageField>, ()> {
    let (_whitespace, rest) = white_block(token);
    if let (Some(name), token) = symbol_optional(rest) {
        let (_whitespace, token) = white(token)?;
        let (type_tag, token) = type_tag(token)?;
        let mut field: MessageField = (name.value(), type_tag).into();

        let mut token: Token = token;
        if let (Some(field_number), t) = field_number(token)? {
            field.set_field_number(field_number);
            token = t;
        }

        let (_whitespace, token) = white_optional(token);
        let (_semi_colon, token) = exact(token, ";")?;

        Ok((Some(field), token))
    } else {
        Ok((None, token))
    }
}

/// Parses an optional message field number.
/// - Returns `None` if the next none-whitespace token is not a `=`.
fn field_number(token: Token) -> LexResult<Option<u32>, ()> {
    let (_whitespace, rest) = white_optional(token);
    if let (Some(_eq), token) = exact_optional(rest, "=") {
        let (_whitespace, token) = white_optional(token);
        let (symbol, token) = symbol(token)?;
        let value: u32 = u32::from_str(symbol.value()).map_err(|_| LexError::from(symbol))?;
        Ok((Some(value), token))
    } else {
        Ok((None, token))
    }
}
