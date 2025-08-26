use clerr::{Code, Report};
use colored::Colorize;

use crate::Error::*;

/// A compiler error.
#[derive(Debug)]
pub enum Error {
    /// An invalid file name error.
    InvalidFileName { file_name: String, context: String },

    /// A `file_storage` error.
    Storage(file_storage::Error),

    /// A serialization error.
    Serial(serde_json::Error),

    /// A parsing error.
    Parse(Report),

    /// A validation error.
    Validate(Report),

    /// A link error.
    Link(proto_packet_link::Error),

    /// A code generation error.
    Gen(Report),

    /// Another uncategorized error.
    Other(Report),
}

impl From<file_storage::Error> for Error {
    fn from(error: file_storage::Error) -> Self {
        Storage(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Serial(error)
    }
}

impl From<proto_packet_link::Error> for Error {
    fn from(error: proto_packet_link::Error) -> Self {
        Link(error)
    }
}

impl Error {
    //! Report

    //! Creates the error report.
    pub fn report(self) -> Report {
        match self {
            InvalidFileName { file_name, context } => {
                let code: Code = Code::error(
                    "FILE_NAME",
                    "an error occurred while constructing a file name",
                );
                Report::new(code).with_entry(vec![
                    file_name.to_string().normal(),
                    "\n".normal(),
                    context.normal(),
                ])
            }
            Storage(error) => {
                let code: Code = Code::error("FILE_IO", "a file IO error had occurred");
                Report::new(code).with_entry(vec![error.to_string().normal()])
            }
            Serial(error) => {
                let code: Code = Code::error("SERDE", "a serialization error had occurred");
                Report::new(code).with_entry(vec![error.to_string().normal()])
            }
            Parse(report) => report,
            Validate(report) => report,
            Link(error) => error.to_report(),
            Gen(report) => report,
            Other(report) => report,
        }
    }
}
