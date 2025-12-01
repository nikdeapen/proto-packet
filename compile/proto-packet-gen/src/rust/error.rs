use clerr::{Code, Properties, Report};
use proto_packet_tree::TypeName;
use std::fmt::{Display, Formatter};

/// An error generating rust code.
#[derive(Debug)]
pub enum Error {
    /// There was an error converting the `type_name` to a `ModName`.
    TypeNameToModName { type_name: TypeName, error: String },
}

impl Error {
    //! Report

    /// Converts the error to an error report.
    pub fn to_report(self) -> Report {
        // todo -- `Properties` quality of life improvements
        match self {
            Self::TypeNameToModName { type_name, error } => Report::new(Code::error(
                "G_RUST_TYPE_NAME_TO_MOD_NAME",
                "the type name could not be converted to a mod name",
            ))
            .with_entry(
                Properties {
                    properties: vec![
                        ("type_name".to_string(), type_name.to_string()),
                        ("error".to_string(), error),
                    ],
                }
                .entry(),
            ),
        }
    }
}

impl From<Error> for Report {
    fn from(error: Error) -> Self {
        error.to_report()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
