use clerr::Report;
use file_storage::FolderPath;

/// A compiler error.
#[derive(Debug)]
pub enum Error {
    /// An error reading a schema file.
    ReadSchema(file_storage::Error),

    /// An error writing a source file.
    WriteSource(file_storage::Error),

    /// An error reading a config file.
    ReadConfig(file_storage::Error),

    /// An error parsing a config file.
    ParseConfig(serde_json::Error),

    /// An error reading a project.
    ReadProject(file_storage::Error),

    /// An error parsing a schema file.
    Parse(Report),

    /// An error validating a schema file.
    Validate(Report),

    /// The mod path was invalid.
    InvalidModPath(String),

    /// The mod name was invalid.
    InvalidModName {
        folder: String,
        file: String,
        mod_name: String,
    },

    /// The generated source file name was invalid.
    InvalidSourceFileName { root: FolderPath, file_name: String },

    /// An error generating source code.
    Gen(Report),
}

impl Error {
    //! Report

    /// Converts the error to an error report.
    pub fn to_report(self) -> Report {
        match self {
            Error::ReadSchema(_) => todo!(),
            Error::WriteSource(_) => todo!(),
            Error::ReadConfig(_) => todo!(),
            Error::ParseConfig(_) => todo!(),
            Error::ReadProject(_) => todo!(),
            Error::Parse(report) => report,
            Error::Validate(report) => report,
            Error::InvalidModPath(_) => todo!(),
            Error::InvalidModName { .. } => todo!(),
            Error::InvalidSourceFileName { .. } => todo!(),
            Error::Gen(report) => report,
        }
    }
}

impl From<Error> for Report {
    fn from(error: Error) -> Self {
        error.to_report()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
