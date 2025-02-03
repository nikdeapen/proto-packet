use file_storage::{FilePath, FolderPath, StoragePath};

use proto_packet_tree::ModPathRef;

use crate::rust::Error;

/// Responsible for naming files.
#[derive(Clone, Debug)]
pub struct FileNaming {
    dot_schema: String,
    dot_target: String,
}

impl Default for FileNaming {
    fn default() -> Self {
        Self {
            dot_schema: ".pps".to_string(),
            dot_target: ".rs".to_string(),
        }
    }
}

impl FileNaming {
    //! Properties

    /// Gets the `.{schema}` extension.
    pub fn dot_schema(&self) -> &str {
        self.dot_schema.as_str()
    }

    /// Gets the `.{target}` extension.
    pub fn dot_target(&self) -> &str {
        self.dot_target.as_str()
    }
}

impl FileNaming {
    //! File for ModPath

    /// Gets the file for the given `mod_path` in the `target` folder.
    pub fn file_for_mod_path(
        &self,
        target: &FolderPath,
        mod_path: ModPathRef,
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
        target.append(self.dot_target.as_str());

        if target.is_file() {
            Ok(target.to_file().unwrap())
        } else {
            Err(Error::InvalidFileExtension {
                file_extension: self.dot_target.clone(),
            })
        }
    }
}
