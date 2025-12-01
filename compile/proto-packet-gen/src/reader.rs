use clerr::Report;
use proto_packet_tree::{ModPathRef, SchemaFile};

/// Responsible for reading a schema file.
pub trait Reader {
    /// Gets the mod path.
    fn mod_path(&self) -> ModPathRef<'_>;

    /// Reads the schema file.
    fn read(&self) -> Result<SchemaFile, Report>;
}
