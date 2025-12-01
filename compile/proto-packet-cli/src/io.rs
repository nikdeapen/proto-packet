use clerr::{Code, Report};
use colored::Colorize;
use file_storage::FilePath;

/// Converts the `file` to a string of its content.
pub fn file_content(file: &FilePath) -> Result<String, Report> {
    file.read_as_string().map_err(|e| {
        Report::new(Code::error(
            "CLI_IO",
            format!("error reading file: {}", file),
        ))
        .with_entry(vec![e.to_string().normal()])
    })
}
