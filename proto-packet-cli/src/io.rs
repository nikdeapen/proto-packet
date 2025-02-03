use clerr::{Code, PrimaryEntry, Report};
use file_storage::FilePath;

/// Converts the `file` to a string of its content.
pub fn file_content(file: &FilePath) -> Result<String, Report> {
    file.read_as_string().map_err(|e| {
        Report::new(
            PrimaryEntry::new(Code::error("CLI", format!("error reading file: {}", file)))
                .with_info(e.to_string()),
        )
    })
}
