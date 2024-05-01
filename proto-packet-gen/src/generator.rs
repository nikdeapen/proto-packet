use std::collections::HashMap;

use proto_packet_tree::SchemaFile;

use crate::GenError;

/// Responsible for generating the code for schema files.
pub trait Generator {
    /// Generates the code for the schema file.
    /// - Returns a map of `file_name -> source_code`.
    fn generate(&self, schema_file: &SchemaFile) -> Result<HashMap<String, String>, GenError>;
}
