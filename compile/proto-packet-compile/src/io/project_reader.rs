use crate::Error::{InvalidModName, InvalidModPath, ReadProject};
use crate::{Config, Error, SchemaReader};
use file_storage::{FilePath, FolderPath};
use proto_packet_tree::{ModName, ModPath};

/// Responsible for reading projects.
#[derive(Debug)]
pub struct ProjectReader<'a> {
    config: &'a Config,
}

impl<'a> From<&'a Config> for ProjectReader<'a> {
    fn from(config: &'a Config) -> Self {
        Self { config }
    }
}

impl<'a> ProjectReader<'a> {
    //! Read

    /// Reads the project from the schema file `root`.
    pub fn read(&self, root: &FolderPath) -> Result<Vec<SchemaReader>, Error> {
        let mut schemas: Vec<FilePath> = root
            .list_files_as_vec_unsorted()
            .map_err(|e| ReadProject(e))?;
        let mut readers: Vec<SchemaReader> = Vec::with_capacity(schemas.len());

        for file in schemas.drain(..) {
            if let Some(mod_path) = self.mod_path(root, &file)? {
                readers.push(SchemaReader::new(mod_path, file));
            }
        }

        Ok(readers)
    }
}

impl<'a> ProjectReader<'a> {
    //! Mod Path

    /// Gets the mod path for the `file` relative to the `root` schema folder.
    fn mod_path(&self, root: &FolderPath, file: &FilePath) -> Result<Option<ModPath>, Error> {
        if root.path().file_separator() != file.path().file_separator() {
            todo!()
        } else {
            if !file.as_str().starts_with(root.as_str()) {
                todo!()
            } else {
                let file_name: &str = &file.as_str()[root.as_str().len()..];
                if !file_name.ends_with(self.config.dot_schema.as_str()) {
                    Ok(None)
                } else {
                    let file_name: &str =
                        &file_name[..(file_name.len() - self.config.dot_schema.len())];
                    let mut mod_path: String = String::with_capacity(file_name.len());
                    for mod_name in file_name.split(root.path().file_separator()) {
                        let mod_name: ModName = match ModName::new(mod_name) {
                            Ok(mod_name) => mod_name,
                            Err(_e) => {
                                return Err(InvalidModName {
                                    folder: root.to_string(),
                                    file: file.to_string(),
                                    mod_name: mod_name.to_string(),
                                })
                            }
                        };
                        if !mod_path.is_empty() {
                            mod_path.push('.');
                        }
                        mod_path.push_str(mod_name.as_ref());
                    }
                    let mod_path: ModPath =
                        ModPath::new(mod_path).map_err(|e| InvalidModPath(e.to_string()))?;
                    Ok(Some(mod_path))
                }
            }
        }
    }
}
