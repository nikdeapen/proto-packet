use crate::Error;
use crate::Error::*;
use clerr::Report;
use file_storage::FilePath;
use lex::{Context, Token};
use proto_packet_gen::Reader;
use proto_packet_link::SchemaLinker;
use proto_packet_parse::{parse_schema_file, SchemaFileTree};
use proto_packet_tree::{ModPath, ModPathRef, SchemaFile};
use proto_packet_validate::validate_schema_file;

/// Responsible for reading schema files.
#[derive(Debug)]
pub struct SchemaReader {
    mod_path: ModPath,
    schema_file: FilePath,
}

impl SchemaReader {
    //! Construction

    /// Creates a new schema reader.
    pub const fn new(mod_path: ModPath, schema_file: FilePath) -> Self {
        Self {
            mod_path,
            schema_file,
        }
    }
}

impl SchemaReader {
    //! Read

    /// Reads the schema file.
    fn read_schema(&self) -> Result<SchemaFile, Error> {
        let source: String = self
            .schema_file
            .read_as_string()
            .map_err(|e| ReadSchema(e))?;
        let token: Token = Token::new(source.as_str(), 0, 0);
        let config: lex::Config = proto_packet_parse::config();
        let context: Context = Context::new(token, &config);
        let parsed: SchemaFileTree = parse_schema_file(context).map_err(|e| {
            let token: Token = e.token();
            let report: Report = e
                .to_error()
                .to_report(self.schema_file.as_str(), context, token);
            Parse(report)
        })?;
        let validated: SchemaFile = validate_schema_file(&parsed)
            .map_err(|e| Validate(e.to_report(self.schema_file.as_str(), context)))?;
        let linked: SchemaFile =
            unsafe { SchemaLinker::new_unchecked(self.mod_path.to_ref()).link(&validated) }?;
        Ok(linked)
    }
}

impl Reader for SchemaReader {
    fn mod_path(&self) -> ModPathRef<'_> {
        self.mod_path.to_ref()
    }

    fn read(&self) -> Result<SchemaFile, Report> {
        self.read_schema().map_err(|e| e.to_report())
    }
}
