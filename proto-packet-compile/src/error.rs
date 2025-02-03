use clerr::{Code, PrimaryEntry, Report};
use colored::Colorize;

use crate::Error::*;

/// A compiler error.
#[derive(Debug)]
pub enum Error {
    /// A `file_storage` error.
    Storage(file_storage::Error),

    /// A parsing error.
    Parse(Report),

    /// A validation error.
    Validate(Report),

    /// A linking error.
    Link(proto_packet_link::Error),

    /// A code generation error.
    Gen(Report),
}

impl From<file_storage::Error> for Error {
    fn from(error: file_storage::Error) -> Self {
        Storage(error)
    }
}

impl From<proto_packet_link::Error> for Error {
    fn from(error: proto_packet_link::Error) -> Self {
        Self::Link(error)
    }
}

impl Error {
    //! Report

    //! Converts the error to a report.
    pub fn to_report(self) -> Report {
        match self {
            Storage(error) => {
                let code: Code = Code::error("FILE_IO", "a file IO error had occurred");
                let primary: PrimaryEntry =
                    PrimaryEntry::new(code).with_all_info(vec![error.to_string().normal()]);
                Report::new(primary)
            }
            Parse(report) => report,
            Validate(report) => report,
            Link(error) => error.to_report(),
            Gen(report) => report,
        }
    }
}
