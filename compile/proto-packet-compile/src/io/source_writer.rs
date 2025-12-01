use crate::Error;
use clerr::Report;
use code_gen::{CodeBuffer, Source, Statement};
use file_storage::{FilePath, FolderPath, StoragePath};
use proto_packet_gen::Writer;

/// Responsible for writing source files.
#[derive(Debug)]
pub struct SourceWriter {
    root: FolderPath,
}

impl From<FolderPath> for SourceWriter {
    fn from(root: FolderPath) -> Self {
        Self { root }
    }
}

impl Writer for SourceWriter {
    fn write(&self, source: &Source, file_name: &str) -> Result<(), Report> {
        let mut buffer: CodeBuffer = CodeBuffer::default();
        source.write(&mut buffer, 0);
        let source: String = buffer.to_string();

        let file: FilePath = self.file_path(file_name)?;
        file.write_str(source.as_str())
            .map_err(|e| Error::WriteSource(e))?;

        Ok(())
    }
}

impl SourceWriter {
    //! File Path

    /// Gets the file path for the given `file_name`.
    pub fn file_path(&self, file_name: &str) -> Result<FilePath, Error> {
        let path: StoragePath = self
            .root
            .clone_with_extra_capacity(file_name.len())
            .with_appended(file_name);

        if let Ok(file) = path.to_file() {
            Ok(file)
        } else {
            Err(Error::InvalidSourceFileName {
                root: self.root.clone(),
                file_name: file_name.to_string(),
            })?
        }
    }
}
