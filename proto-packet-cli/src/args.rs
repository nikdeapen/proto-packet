use clerr::{Code, PrimaryEntry, Report};
use file_storage::{FilePath, FolderPath, StoragePath};

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
