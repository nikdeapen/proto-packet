use clerr::{Code, PrimaryEntry, Report};

use proto_packet_tree::TypeName;

/// A rust code generation error.
#[derive(Debug)]
pub enum Error {
    TypeNameNotConvertableToModName {
        type_name: TypeName,
        mod_name: String,
        error_message: &'static str,
    },
    InvalidFileExtension {
        file_extension: String,
    },
}

impl Error {
    //! Report

    /// Converts the error to a command-line error report.
    pub fn to_report(self) -> Report {
        Report::new(PrimaryEntry::new(Code::error("IMPLEMENT", "ME")))
    }
}
