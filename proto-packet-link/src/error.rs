use clerr::{Code, PrimaryEntry, Report};
use colored::Colorize;

use proto_packet_tree::{ModPath, QualifiedName};

/// A linking error.
#[derive(Debug)]
pub enum Error {
    /// An unrecognized name was encountered.
    UnrecognizedName {
        context: ModPath,
        name: QualifiedName,
    },
}

impl Error {
    //! Report

    /// Converts the error to a report.
    pub fn to_report(self) -> Report {
        match self {
            Self::UnrecognizedName { context, name } => {
                let code: Code = Code::error("LINK", "encountered an unrecognized name");
                let primary: PrimaryEntry = PrimaryEntry::new(code).with_all_info(vec![
                    "context: ".bright_blue(),
                    context.to_string().normal(),
                    "\n".normal(),
                    "name:    ".bright_blue(),
                    name.to_string().normal(),
                    "\n".normal(),
                ]);
                Report::new(primary)
            }
        }
    }
}
