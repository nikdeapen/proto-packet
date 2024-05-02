use std::fs;

use crate::CompilerError;
use proto_packet_parse::parse;
use proto_packet_tree::SchemaFile;

use crate::CompilerError::{FileReadError, Other};

/// Responsible for reading schema files.
#[derive(Clone, Debug, Default)]
pub struct SchemaFileReader {
    _nothing: (),
}

impl SchemaFileReader {
    //! Read Schema File

    /// Reads the schema file.
    pub fn read_schema_file(&self, schema_file_name: &str) -> Result<SchemaFile, CompilerError> {
        let source_code: String = fs::read_to_string(schema_file_name)
            .map_err(|e| FileReadError(schema_file_name.to_string(), e))?;
        parse(&source_code).map_err(|e| Other(e))
    }
}
