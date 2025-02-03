use clerr::{Code, PrimaryEntry, Report};

use proto_packet_tree::{QualifiedName, TypeName};

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
    InvalidModFileName {
        file_name: String,
    },
    DuplicateTypeName {
        name: QualifiedName,
    },
    TypeNameConflictsWithModName {
        name: QualifiedName,
    },
}

impl Error {
    //! Report

    /// Converts the error to a command-line error report.
    pub fn to_report(self) -> Report {
        Report::new(PrimaryEntry::new(Code::error("IMPLEMENT", "ME")))
    }
}
