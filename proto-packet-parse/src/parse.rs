use lex::Token;

use proto_packet_tree::SchemaFile;

use crate::schema_file;

/// Parses a schema file.
pub fn parse(content: &str) -> Result<SchemaFile, String> {
    let token: Token =
        Token::try_from(content).map_err(|_| format!("file size >= 4 GiB: {}", content.len()))?;
    schema_file(token).map_err(|e| e.token().to_string())
}
