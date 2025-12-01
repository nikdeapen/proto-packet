use proto_packet_tree::{ModPath, TypeName};
use std::fmt::{Display, Formatter};

/// An error linking a schema file.
#[derive(Debug)]
pub enum Error {
    /// The `type_name` was not resolvable.
    UnresolvableName {
        mod_path: ModPath,
        type_name: TypeName,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
