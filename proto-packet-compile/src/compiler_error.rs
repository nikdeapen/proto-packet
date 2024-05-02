use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::io;

use proto_packet_gen::GenError;

/// An error compiling a ProtoPacket schema project.
#[derive(Debug)]
pub enum CompilerError {
    /// An uncategorized error.
    Other(String),

    /// An error reading a directory.
    DirectoryReadError(String, io::Error),

    /// An error converting an OS path name to a UTF-8 string.
    PathNameUTF8Error(OsString),

    /// The schema file was too large. (>= 4 GiB)
    FileTooLarge(String, usize),

    /// An error reading a schema file.
    FileReadError(String, io::Error),

    /// An error writing a source file.
    FileWriteError(String, io::Error),

    /// An error parsing a schema file.
    ParseError(String),

    /// An error generating the output code.
    GeneratorError(GenError),
}

impl std::error::Error for CompilerError {}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
