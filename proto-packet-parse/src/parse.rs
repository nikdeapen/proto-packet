use lex::Token;

use proto_packet_tree::SchemaFile;

use crate::schema_file;

/// Parses a schema file from the source code.
pub fn parse(source_code: &str) -> Result<SchemaFile, String> {
    let token: Token = Token::try_from(source_code)
        .map_err(|_| format!("file size >= 4 GiB: {}", source_code.len()))?;
    schema_file(token).map_err(|e| e.token().to_string())
}
