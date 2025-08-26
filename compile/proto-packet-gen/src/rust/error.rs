use clerr::{Code, Report};
use file_storage::StoragePath;
use proto_packet_tree::{ModName, QualifiedName};

/// A rust code generation error.
#[derive(Debug)]
pub enum Error {
    /// An invalid file extension was given for the file system.
    InvalidFileExtension {
        file_path: StoragePath,
        file_extension: String,
    },

    /// An invalid mod file name was given for the file system.
    InvalidModFileName {
        file_path: StoragePath,
        mod_file_name: String,
    },

    /// An invalid mod name was constructed from the qualified type name.
    InvalidModNameFromTypeName {
        qualified_name: QualifiedName,
        error_message: &'static str,
    },

    /// A duplicate mod path was generated from a type name.
    DuplicateModNameFromTypeName {
        qualified_name: QualifiedName,
        mod_name: ModName,
    },
}

impl Error {
    //! Error Codes

    const GEN_RUST_INV_FILE_EXT: &'static str = "GEN_RUST_INV_FILE_EXT";
    const GEN_RUST_INV_MOD_FILE: &'static str = "GEN_RUST_INV_MOD_FILE";
    const GEN_RUST_INV_MOD_NAME_FROM_TYPE_NAME: &'static str =
        "GEN_RUST_INV_MOD_NAME_FROM_TYPE_NAME";
    const DUPLICATE_MOD_NAME_FROM_TYPE_NAME: &'static str = "DUPLICATE_MOD_NAME_FROM_TYPE_NAME";
}

impl Error {
    //! Report

    /// Converts the error to an error report.
    pub fn to_report(self) -> Report {
        match self {
            Self::InvalidFileExtension {
                file_path,
                file_extension,
            } => Report::new(Code::error(
                Self::GEN_RUST_INV_FILE_EXT,
                "encountered an invalid file extension",
            ))
            .with_properties(vec![
                ("file-path".to_string(), file_path.to_string()), // todo -- nit: .into()
                ("file_extension".to_string(), file_extension),
            ]),
            Self::InvalidModFileName {
                file_path,
                mod_file_name,
            } => Report::new(Code::error(
                Self::GEN_RUST_INV_MOD_FILE,
                "encountered an invalid mod file name",
            ))
            .with_properties(vec![
                ("file-path".to_string(), file_path.to_string()), // todo -- nit: .into()
                ("mod-file-name".to_string(), mod_file_name),
            ]),
            Self::InvalidModNameFromTypeName {
                qualified_name,
                error_message,
            } => Report::new(Code::error(
                Self::GEN_RUST_INV_MOD_NAME_FROM_TYPE_NAME,
                "encountered an invalid mod name generated from a file name",
            ))
            .with_properties(vec![
                ("qualified-name".to_string(), qualified_name.to_string()),
                ("error-message".to_string(), error_message.to_string()),
            ]),
            Self::DuplicateModNameFromTypeName {
                qualified_name,
                mod_name,
            } => Report::new(Code::error(
                Self::DUPLICATE_MOD_NAME_FROM_TYPE_NAME,
                "encountered a duplicate mod name generated from a type name",
            ))
            .with_properties(vec![
                ("qualified-name".to_string(), qualified_name.to_string()),
                ("mod-name".to_string(), mod_name.to_string()), // todo -- nit: .into()
            ]),
        }
    }
}
