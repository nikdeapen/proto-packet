use clerr::{Code, Report};
use colored::Colorize;
use file_storage::{FilePath, FolderPath, StoragePath};
use proto_packet_compile::Language;

/// Gets the `StoragePath` from the `path` argument.
pub fn path_arg(path: String) -> Result<StoragePath, Report> {
    let cwd: FolderPath = FolderPath::current_working_directory().map_err(|e| {
        Report::new(Code::error(
            "CLI_IO",
            "error resolving the current working directory",
        ))
        .with_entry(vec![e.to_string().normal()])
    })?;
    Ok(cwd.with_appended(path)) // todo -- normalize the path
}

/// Gets the `FilePath` from the `path` argument.
pub fn file_arg(path: String) -> Result<FilePath, Report> {
    let path: StoragePath = path_arg(path)?;
    match path.to_file() {
        Ok(file) => Ok(file),
        Err(path) => Err(Report::new(Code::error(
            "CLI_IO",
            format!("path not a file: {}", path),
        ))),
    }
}

/// Gets the `FolderPath` from the `path` argument.
pub fn folder_arg(path: String) -> Result<FolderPath, Report> {
    let path: StoragePath = path_arg(path)?;
    match path.to_folder() {
        Ok(folder) => Ok(folder),
        Err(path) => Err(Report::new(Code::error(
            "CLI_IO",
            format!("path not a folder: {}", path),
        ))),
    }
}

/// Gets the target language from the `lang`.
pub fn target_lang(lang: &str) -> Result<Language, Report> {
    Ok(match lang {
        "rust" => Language::Rust,
        _ => {
            return Err(Report::new(Code::error(
                "CLI_ARG",
                format!("unrecognized language: {}", lang),
            )))
        }
    })
}
