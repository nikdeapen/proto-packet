use clerr::{Code, PrimaryEntry, Report};
use file_storage::{FilePath, FolderPath, StoragePath};

use proto_packet_gen::rust::GenRust;
use proto_packet_gen::Generator;

/// Gets the `StoragePath` from the `path` argument.
pub fn path_arg(path: String) -> Result<StoragePath, Report> {
    let cwd: FolderPath = FolderPath::current_working_directory()
        .map_err(|e| Report::new(PrimaryEntry::new(Code::error("CLI", e.to_string()))))?;
    Ok(cwd.with_appended(path))
}

/// Gets the `FilePath` from the `path` argument.
pub fn file_arg(path: String) -> Result<FilePath, Report> {
    let path: StoragePath = path_arg(path)?;
    if path.is_file() {
        Ok(path.to_file().unwrap())
    } else {
        Err(Report::new(PrimaryEntry::new(Code::error(
            "CLI",
            format!("path not a file: {}", path),
        ))))
    }
}

/// Gets the `FolderPath` from the `path` argument.
pub fn folder_arg(path: String) -> Result<FolderPath, Report> {
    let path: StoragePath = path_arg(path)?;
    if path.is_folder() {
        Ok(path.to_folder().unwrap())
    } else {
        Err(Report::new(PrimaryEntry::new(Code::error(
            "CLI",
            format!("path not a folder: {}", path),
        ))))
    }
}

/// Creates the generator for the `lang`.
pub fn create_generator(lang: &str) -> Result<Box<dyn Generator>, Report> {
    Ok(match lang {
        "rust" => Box::new(GenRust::default()),
        _ => {
            return Err(Report::new(PrimaryEntry::new(Code::error(
                "CLI",
                format!("unrecognized language: {}", lang),
            ))))
        }
    })
}
