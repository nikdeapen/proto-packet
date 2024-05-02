use lex::{LexError, Token};

use proto_packet_tree::SchemaFile;

use crate::{message, white_optional};

/// Parses a source file.
pub fn schema_file(token: Token) -> Result<SchemaFile, LexError<()>> {
    let mut source: SchemaFile = SchemaFile::default();
    let mut token: Token = token;
    loop {
        let mut matched: bool = false;

        if let (Some(message), r) = message(token)? {
            source.add_declaration(message);
            matched = true;
            token = r;
        }

        if !matched {
            let (_white, unknown) = white_optional(token);
            return if !unknown.is_empty() {
                Err(unknown.into())
            } else {
                Ok(source)
            };
        }
    }
}
