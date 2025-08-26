use file_storage::{FilePath, FolderPath, StoragePath};

use proto_packet_tree::ModPathRef;

use crate::rust::Error;

/// Responsible for managing file names.
#[derive(Copy, Clone, Debug)]
pub struct FileNaming<'a> {
    dot_schema: &'a str,
    dot_target: &'a str,
}

impl Default for FileNaming<'static> {
    fn default() -> Self {
        Self {
            dot_schema: ".pps",
            dot_target: ".rs",
        }
    }
}

impl<'a> FileNaming<'a> {
    //! Properties

    /// Gets the `.{schema}` extension.
    pub fn dot_schema(&self) -> &str {
        self.dot_schema
    }

    /// Gets the `.{target}` extension.
    pub fn dot_target(&self) -> &str {
        self.dot_target
    }
}

impl<'a> FileNaming<'a> {
    //! File for ModPath

    /// Gets the file path for the given `mod_path` in the `target` folder.
    pub fn file_for_mod_path(
        &self,
        mod_path: ModPathRef,
        target: &FolderPath,
    ) -> Result<FilePath, Error> {
        let fs: char = target.path().file_separator();
        let fs_len: usize = fs.len_utf8();
        let dot_count: usize = mod_path
            .value()
            .as_bytes()
            .iter()
            .filter(|c| **c == b'.')
            .count();
        let extra_capacity: usize =
            mod_path.value().len() + ((fs_len - 1) * dot_count) + self.dot_target.len();

        let mut target: StoragePath = target.clone_with_extra_capacity(extra_capacity).to_path();
        let mut rem: &str = mod_path.value();
        while let Some(dot) = rem.as_bytes().iter().position(|c| *c == b'.') {
            target.append(&rem[..dot]);
            target.append_char(fs);
            rem = &rem[dot + fs_len..];
        }
        target.append(rem);
        target.append(self.dot_target);

        target
            .to_file()
            .map_err(|file_path| Error::InvalidFileExtension {
                file_path: file_path.export_path(),
                file_extension: self.dot_target.to_string(),
            })
    }
}
