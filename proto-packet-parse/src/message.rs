use lex::{block_comments, exact, exact_optional, symbol, whitespace_optional, LexResult, Token};

use proto_packet_tree::{Message, WithComments};

use crate::{message_field, white, white_block, white_optional, LINE_COMMENT_DELIMITER};

/// Parses an optional message.
/// - Returns `None` when the next non-whitespace token is not `message`.
pub fn message(token: Token) -> LexResult<Option<Message>, ()> {
    let (comments, rest) = white_block(token);
    if let (Some(_message), token) = exact_optional(rest, "message") {
        let (_white, token) = white(token)?;
        let (name, token) = symbol(token)?;
        let (_white, token) = whitespace_optional(token);
        let (_open_curly, token) = exact(token, "{")?;
        let mut message: Message = name.value().into();

        let mut token: Token = token;
        while let (Some(field), t) = message_field(token)? {
            message.add_field(field);
            token = t;
        }

        let (_whitespace, token) = white_optional(token);
        let (_close_curly, token) = exact(token, "}")?;

        block_comments(comments, LINE_COMMENT_DELIMITER, |comment| {
            message.add_comment(comment)
        });

        Ok((Some(message), token))
    } else {
        Ok((None, token))
    }
}
